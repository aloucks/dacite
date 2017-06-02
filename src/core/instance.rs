// Copyright (c) 2017, Dennis Hamester <dennis.hamester@startmail.com>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
// REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND
// FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
// INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
// LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
// OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
// PERFORMANCE OF THIS SOFTWARE.

use core::PhysicalDevice;
use core::allocator_helper::AllocatorHelper;
use core;
use libloading;
use std::cmp::Ordering;
use std::error;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem;
use std::ptr;
use std::sync::Arc;
use utils;
use vks;
use {TryDestroyError, TryDestroyErrorKind, VulkanObject};

#[cfg(unix)]
use libloading::os::unix::Symbol;

#[cfg(windows)]
use libloading::os::windows::Symbol;

#[cfg(feature = "ext_debug_report_1")]
use ext_debug_report;

#[cfg(any(feature = "khr_display_21",
          feature = "khr_xlib_surface_6",
          feature = "khr_wayland_surface_5"))]
use khr_surface;

#[cfg(feature = "khr_display_21")]
use khr_display;

#[cfg(feature = "khr_display_21")]
use std::sync::Mutex;

#[cfg(feature = "khr_xlib_surface_6")]
use khr_xlib_surface;

#[cfg(feature = "khr_wayland_surface_5")]
use khr_wayland_surface;

const VK_GET_INSTANCE_PROC_ADDR: &'static str = "vkGetInstanceProcAddr";

/// Indicates an error, which occurred before an Instance was created.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EarlyInstanceError {
    /// The Vulkan library could be loaded.
    LoadLibraryFailed(String),

    /// A required symbol was not found in the Vulkan library.
    SymbolNotFound(String),

    /// A Vulkan error occurred.
    VulkanError(core::Error),
}

impl From<vks::VkResult> for EarlyInstanceError {
    fn from(res: vks::VkResult) -> Self {
        EarlyInstanceError::VulkanError(res.into())
    }
}

impl fmt::Display for EarlyInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EarlyInstanceError::LoadLibraryFailed(ref l) => write!(f, "Failed to load library {}", l),
            EarlyInstanceError::SymbolNotFound(ref s) => write!(f, "Symbol {} not found", s),
            EarlyInstanceError::VulkanError(e) => e.fmt(f),
        }
    }
}

impl error::Error for EarlyInstanceError {
    fn description(&self) -> &str {
        match *self {
            EarlyInstanceError::LoadLibraryFailed(_) => "LoadLibraryFailed",
            EarlyInstanceError::SymbolNotFound(_) => "SymbolNotFound",
            EarlyInstanceError::VulkanError(ref e) => e.description(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckInstanceExtensionsError {
    Missing(Vec<core::InstanceExtensionProperties>),
    EarlyInstanceError(EarlyInstanceError),
}

impl From<EarlyInstanceError> for CheckInstanceExtensionsError {
    fn from(e: EarlyInstanceError) -> Self {
        CheckInstanceExtensionsError::EarlyInstanceError(e)
    }
}

/// See [`VkInstance`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkInstance)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instance(Arc<Inner>);

impl VulkanObject for Instance {
    type NativeVulkanObject = vks::VkInstance;

    #[inline]
    fn as_native_vulkan_object(&self) -> Self::NativeVulkanObject {
        self.handle()
    }

    fn try_destroy(self) -> Result<(), TryDestroyError<Self>> {
        let strong_count = Arc::strong_count(&self.0);
        if strong_count == 1 {
            Ok(())
        }
        else {
            Err(TryDestroyError::new(self, TryDestroyErrorKind::InUse(Some(strong_count))))
        }
    }
}

impl Instance {
    #[inline]
    pub(crate) fn handle(&self) -> vks::VkInstance {
        self.0.handle
    }

    #[inline]
    pub(crate) fn loader(&self) -> &vks::InstanceProcAddrLoader {
        &self.0.loader
    }

    #[cfg(feature = "khr_display_21")]
    pub(crate) fn add_display_mode_allocator(&self, allocator: AllocatorHelper) {
        self.0.display_mode_allocators.lock().unwrap().push(allocator);
    }

    pub fn check_instance_extensions(extensions: Vec<core::InstanceExtensionProperties>) -> Result<Vec<core::InstanceExtension>, CheckInstanceExtensionsError> {
        let mut found = Vec::new();
        let mut missing = extensions;

        let instance_extensions = Instance::enumerate_instance_extension_properties(None)?;
        for extension in instance_extensions {
            let pos = missing.iter().position(|e| (e.extension == extension.extension) && (e.spec_version <= extension.spec_version));
            if let Some(pos) = pos {
                found.push(missing.swap_remove(pos).extension);
            }
        }

        if missing.is_empty() {
            Ok(found)
        }
        else {
            Err(CheckInstanceExtensionsError::Missing(missing))
        }
    }

    /// See [`vkCreateInstance`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateInstance)
    pub fn create(create_info: &core::InstanceCreateInfo, allocator: Option<Box<core::Allocator>>) -> Result<Instance, EarlyInstanceError> {
        let (library, vk_get_instance_proc_addr) = unsafe {
            let library = libloading::Library::new(vks::VULKAN_LIBRARY_NAME)
                .map_err(|_| EarlyInstanceError::LoadLibraryFailed(vks::VULKAN_LIBRARY_NAME.to_owned()))?;
            let vk_get_instance_proc_addr = {
                let vk_get_instance_proc_addr: libloading::Symbol<vks::PFN_vkGetInstanceProcAddr> = library
                    .get(VK_GET_INSTANCE_PROC_ADDR.as_bytes())
                    .map_err(|_| EarlyInstanceError::SymbolNotFound(VK_GET_INSTANCE_PROC_ADDR.to_owned()))?;
                vk_get_instance_proc_addr.into_raw()
            };
            (library, vk_get_instance_proc_addr)
        };

        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), AllocatorHelper::callbacks);

        let mut loader = vks::InstanceProcAddrLoader::from_get_instance_proc_addr(*vk_get_instance_proc_addr);
        unsafe {
            loader.load_core_null_instance();
        }

        let create_info_wrapper = core::VkInstanceCreateInfoWrapper::new(create_info, true);
        let mut instance = ptr::null_mut();
        let res = unsafe {
            (loader.core_null_instance.vkCreateInstance)(&create_info_wrapper.vks_struct, allocation_callbacks, &mut instance)
        };
        if res != vks::VK_SUCCESS {
            return Err(res.into());
        }

        unsafe {
            loader.load_core(instance);

            for instance_extension in &create_info.enabled_extensions {
                match *instance_extension {
                    #[cfg(feature = "khr_surface_25")]
                    core::InstanceExtension::KhrSurface => loader.load_khr_surface(instance),

                    #[cfg(feature = "ext_debug_report_1")]
                    core::InstanceExtension::ExtDebugReport => loader.load_ext_debug_report(instance),

                    #[cfg(feature = "khr_display_21")]
                    core::InstanceExtension::KhrDisplay => loader.load_khr_display(instance),

                    #[cfg(feature = "khr_xlib_surface_6")]
                    core::InstanceExtension::KhrXlibSurface => loader.load_khr_xlib_surface(instance),

                    #[cfg(feature = "khr_wayland_surface_5")]
                    core::InstanceExtension::KhrWaylandSurface => loader.load_khr_wayland_surface(instance),

                    core::InstanceExtension::Unknown(_) => { },
                }
            }
        }

        let inner = unsafe {
            let mut inner: Inner = mem::uninitialized();
            ptr::write(&mut inner.handle, instance);
            ptr::write(&mut inner.allocator, allocator_helper);
            ptr::write(&mut inner.loader, loader);
            ptr::write(&mut inner.library, library);
            ptr::write(&mut inner.vk_get_instance_proc_addr, vk_get_instance_proc_addr);

            #[cfg(feature = "ext_debug_report_1")]
            {
                ptr::write(&mut inner.debug_report_callback, None);

                if let Some(ref chain) = create_info.chain {
                    if let Some(ref debug_report_callback_create_info_ext) = chain.debug_report_callback_create_info_ext {
                        ptr::write(&mut inner.debug_report_callback, Some(debug_report_callback_create_info_ext.callback.clone()));
                    }
                }
            }

            #[cfg(feature = "khr_display_21")]
            ptr::write(&mut inner.display_mode_allocators, Mutex::new(Vec::new()));

            inner
        };

        Ok(Instance(Arc::new(inner)))
    }

    /// See [`vkEnumeratePhysicalDevices`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumeratePhysicalDevices)
    pub fn enumerate_physical_devices(&self) -> Result<Vec<PhysicalDevice>, core::Error> {
        let mut num_physical_devices = 0;
        let res = unsafe {
            (self.loader().core.vkEnumeratePhysicalDevices)(self.handle(), &mut num_physical_devices, ptr::null_mut())
        };
        if res != vks::VK_SUCCESS {
            return Err(res.into());
        }

        let mut physical_devices = Vec::with_capacity(num_physical_devices as usize);
        let res = unsafe {
            (self.loader().core.vkEnumeratePhysicalDevices)(self.handle(), &mut num_physical_devices, physical_devices.as_mut_ptr())
        };
        if res != vks::VK_SUCCESS {
            return Err(res.into());
        }
        unsafe {
            physical_devices.set_len(num_physical_devices as usize);
        }

        let physical_devices: Vec<_> = physical_devices
            .iter()
            .map(|&d| core::PhysicalDevice::new(d, self.clone()))
            .collect();

        Ok(physical_devices)
    }

    /// See [`vkEnumerateInstanceLayerProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumerateInstanceLayerProperties)
    pub fn enumerate_instance_layer_properties() -> Result<core::LayerPropertiesIterator, EarlyInstanceError> {
        unsafe {
            let library = libloading::Library::new(vks::VULKAN_LIBRARY_NAME)
                .map_err(|_| EarlyInstanceError::LoadLibraryFailed(vks::VULKAN_LIBRARY_NAME.to_owned()))?;
            let vk_get_instance_proc_addr: libloading::Symbol<vks::PFN_vkGetInstanceProcAddr> = library
                .get(VK_GET_INSTANCE_PROC_ADDR.as_bytes())
                .map_err(|_| EarlyInstanceError::SymbolNotFound(VK_GET_INSTANCE_PROC_ADDR.to_owned()))?;

            let mut loader = vks::instance_proc_addr_loader::CoreNullInstance::new();
            loader.load(*vk_get_instance_proc_addr, ptr::null_mut());

            let mut num_layer_properties = 0;
            let res = (loader.vkEnumerateInstanceLayerProperties)(&mut num_layer_properties, ptr::null_mut());
            if res != vks::VK_SUCCESS {
                return Err(res.into());
            }

            let mut layer_properties = Vec::with_capacity(num_layer_properties as usize);
            let res = (loader.vkEnumerateInstanceLayerProperties)(&mut num_layer_properties, layer_properties.as_mut_ptr());
            if res != vks::VK_SUCCESS {
                return Err(res.into());
            }
            layer_properties.set_len(num_layer_properties as usize);

            Ok(core::LayerPropertiesIterator(layer_properties.into_iter()))
        }
    }

    /// See [`vkEnumerateInstanceExtensionProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumerateInstanceExtensionProperties)
    pub fn enumerate_instance_extension_properties(layer_name: Option<&str>) -> Result<core::InstanceExtensionPropertiesIterator, EarlyInstanceError> {
        unsafe {
            let library = libloading::Library::new(vks::VULKAN_LIBRARY_NAME)
                .map_err(|_| EarlyInstanceError::LoadLibraryFailed(vks::VULKAN_LIBRARY_NAME.to_owned()))?;
            let vk_get_instance_proc_addr: libloading::Symbol<vks::PFN_vkGetInstanceProcAddr> = library
                .get(VK_GET_INSTANCE_PROC_ADDR.as_bytes())
                .map_err(|_| EarlyInstanceError::SymbolNotFound(VK_GET_INSTANCE_PROC_ADDR.to_owned()))?;

            let mut loader = vks::instance_proc_addr_loader::CoreNullInstance::new();
            loader.load(*vk_get_instance_proc_addr, ptr::null_mut());

            let layer_name_cstr = utils::cstr_from_str(layer_name);

            let mut num_extension_properties = 0;
            let res = (loader.vkEnumerateInstanceExtensionProperties)(layer_name_cstr.1, &mut num_extension_properties, ptr::null_mut());
            if res != vks::VK_SUCCESS {
                return Err(res.into());
            }

            let mut extension_properties = Vec::with_capacity(num_extension_properties as usize);
            let res = (loader.vkEnumerateInstanceExtensionProperties)(layer_name_cstr.1, &mut num_extension_properties, extension_properties.as_mut_ptr());
            if res != vks::VK_SUCCESS {
                return Err(res.into());
            }
            extension_properties.set_len(num_extension_properties as usize);

            Ok(core::InstanceExtensionPropertiesIterator(extension_properties.into_iter()))
        }
    }

    #[cfg(feature = "ext_debug_report_1")]
    /// See [`vkCreateDebugReportCallbackEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDebugReportCallbackEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    pub fn create_debug_report_callback_ext(&self, create_info: &ext_debug_report::DebugReportCallbackCreateInfoExt, allocator: Option<Box<core::Allocator>>) -> Result<ext_debug_report::DebugReportCallbackExt, core::Error> {
        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), AllocatorHelper::callbacks);
        let create_info_wrapper = ext_debug_report::VkDebugReportCallbackCreateInfoEXTWrapper::new(create_info, true);

        let mut debug_report_callback = ptr::null_mut();
        let res = unsafe {
            (self.loader().ext_debug_report.vkCreateDebugReportCallbackEXT)(self.handle(), &create_info_wrapper.vks_struct, allocation_callbacks, &mut debug_report_callback)
        };

        if res == vks::VK_SUCCESS {
            Ok(ext_debug_report::DebugReportCallbackExt::new(debug_report_callback, self.clone(), allocator_helper, create_info_wrapper.callback_helper))
        }
        else {
            Err(res.into())
        }
    }

    #[cfg(feature = "ext_debug_report_1")]
    /// See [`vkDebugReportMessageEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDebugReportMessageEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    pub fn debug_report_message_ext(&self, flags: ext_debug_report::DebugReportFlagsExt, object_type: ext_debug_report::DebugReportObjectTypeExt, object: u64, location: usize, message_code: i32, layer_prefix: &str, message: &str) {
        use std::ffi::CString;

        let layer_prefix = CString::new(layer_prefix).unwrap();
        let message = CString::new(message).unwrap();

        unsafe {
            (self.loader().ext_debug_report.vkDebugReportMessageEXT)(self.handle(), flags, object_type.into(), object, location, message_code, layer_prefix.as_ptr(), message.as_ptr());
        }
    }

    #[cfg(feature = "khr_display_21")]
    /// See [`vkCreateDisplayPlaneSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDisplayPlaneSurfaceKHR)
    /// and extensions [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display),
    /// [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
    pub fn create_display_plane_surface_khr(&self, create_info: &khr_display::DisplaySurfaceCreateInfoKhr, allocator: Option<Box<core::Allocator>>) -> Result<khr_surface::SurfaceKhr, core::Error> {
        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), AllocatorHelper::callbacks);
        let create_info_wrapper = khr_display::VkDisplaySurfaceCreateInfoKHRWrapper::new(create_info, true);

        let mut surface = ptr::null_mut();
        let res = unsafe {
            (self.loader().khr_display.vkCreateDisplayPlaneSurfaceKHR)(self.handle(), &create_info_wrapper.vks_struct, allocation_callbacks, &mut surface)
        };

        if res == vks::VK_SUCCESS {
            Ok(khr_surface::SurfaceKhr::new(surface, self.clone(), allocator_helper))
        }
        else {
            Err(res.into())
        }
    }

    #[cfg(feature = "khr_xlib_surface_6")]
    /// See [`vkCreateXlibSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateXlibSurfaceKHR)
    /// and extension [`VK_KHR_xlib_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_xlib_surface)
    pub fn create_xlib_surface_khr(&self, create_info: &khr_xlib_surface::XlibSurfaceCreateInfoKhr, allocator: Option<Box<core::Allocator>>) -> Result<khr_surface::SurfaceKhr, core::Error> {
        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), AllocatorHelper::callbacks);
        let create_info_wrapper = khr_xlib_surface::VkXlibSurfaceCreateInfoKHRWrapper::new(create_info, true);

        let mut surface = ptr::null_mut();
        let res = unsafe {
            (self.loader().khr_xlib_surface.vkCreateXlibSurfaceKHR)(self.handle(), &create_info_wrapper.vks_struct, allocation_callbacks, &mut surface)
        };

        if res == vks::VK_SUCCESS {
            Ok(khr_surface::SurfaceKhr::new(surface, self.clone(), allocator_helper))
        }
        else {
            Err(res.into())
        }
    }

    #[cfg(feature = "khr_wayland_surface_5")]
    /// See [`vkCreateWaylandSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateWaylandSurfaceKHR)
    /// and extension [`VK_KHR_wayland_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_wayland_surface)
    pub fn create_wayland_surface_khr(&self, create_info: &khr_wayland_surface::WaylandSurfaceCreateInfoKhr, allocator: Option<Box<core::Allocator>>) -> Result<khr_surface::SurfaceKhr, core::Error> {
        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), AllocatorHelper::callbacks);
        let create_info_wrapper = khr_wayland_surface::VkWaylandSurfaceCreateInfoKHRWrapper::new(create_info, true);

        let mut surface = ptr::null_mut();
        let res = unsafe {
            (self.loader().khr_wayland_surface.vkCreateWaylandSurfaceKHR)(self.handle(), &create_info_wrapper.vks_struct, allocation_callbacks, &mut surface)
        };

        if res == vks::VK_SUCCESS {
            Ok(khr_surface::SurfaceKhr::new(surface, self.clone(), allocator_helper))
        }
        else {
            Err(res.into())
        }
    }
}

#[derive(Debug)]
struct Inner {
    handle: vks::VkInstance,
    allocator: Option<AllocatorHelper>,
    loader: vks::InstanceProcAddrLoader,
    library: libloading::Library,
    vk_get_instance_proc_addr: Symbol<vks::PFN_vkGetInstanceProcAddr>,

    #[cfg(feature = "ext_debug_report_1")]
    debug_report_callback: Option<Arc<ext_debug_report::DebugReportCallbacksExt>>,

    #[cfg(feature = "khr_display_21")]
    display_mode_allocators: Mutex<Vec<AllocatorHelper>>,
}

impl Drop for Inner {
    fn drop(&mut self) {
        let allocator = match self.allocator {
            Some(ref allocator) => allocator.callbacks(),
            None => ptr::null(),
        };

        unsafe {
            (self.loader.core.vkDestroyInstance)(self.handle, allocator);
        }
    }
}

unsafe impl Send for Inner { }

unsafe impl Sync for Inner { }

impl PartialEq for Inner {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}

impl Eq for Inner { }

impl PartialOrd for Inner {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.handle.partial_cmp(&other.handle)
    }
}

impl Ord for Inner {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.handle.cmp(&other.handle)
    }
}

impl Hash for Inner {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.handle.hash(state);
    }
}
