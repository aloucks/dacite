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
use ext_debug_report;
use khr_android_surface;
use khr_display;
use khr_mir_surface;
use khr_surface;
use khr_wayland_surface;
use khr_win32_surface;
use khr_xcb_surface;
use khr_xlib_surface;
use libloading;
use std::cmp::Ordering;
use std::error;
use std::ffi::CStr;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::ptr;
use std::sync::Arc;
use std::sync::Mutex;
use utils;
use vks;
use {TryDestroyError, TryDestroyErrorKind, VulkanObject};

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

impl From<vks::vk::VkResult> for EarlyInstanceError {
    fn from(res: vks::vk::VkResult) -> Self {
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

/// See [`VkInstance`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkInstance)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instance(Arc<Inner>);

impl VulkanObject for Instance {
    type NativeVulkanObject = vks::vk::VkInstance;

    #[inline]
    fn id(&self) -> u64 {
        self.as_native_vulkan_object() as u64
    }

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
    pub(crate) fn handle(&self) -> vks::vk::VkInstance {
        self.0.handle
    }

    #[inline]
    pub(crate) fn loader(&self) -> &vks::InstanceProcAddrLoader {
        &self.0.loader
    }

    pub(crate) fn add_display_mode_allocator(&self, allocator: AllocatorHelper) {
        self.0.display_mode_allocators.lock().unwrap().push(allocator);
    }

    pub fn get_enabled_extensions(&self) -> &core::InstanceExtensions {
        &self.0.enabled_extensions
    }

    /// See [`vkCreateInstance`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateInstance)
    pub fn create(create_info: &core::InstanceCreateInfo, allocator: Option<Box<core::Allocator>>) -> Result<Instance, EarlyInstanceError> {
        let (library, vk_get_instance_proc_addr) = unsafe {
            let library = libloading::Library::new(vks::VULKAN_LIBRARY_NAME)
                .map_err(|_| EarlyInstanceError::LoadLibraryFailed(vks::VULKAN_LIBRARY_NAME.to_owned()))?;
            let vk_get_instance_proc_addr = {
                let vk_get_instance_proc_addr: libloading::Symbol<vks::vk::PFN_vkGetInstanceProcAddr> = library
                    .get(VK_GET_INSTANCE_PROC_ADDR.as_bytes())
                    .map_err(|_| EarlyInstanceError::SymbolNotFound(VK_GET_INSTANCE_PROC_ADDR.to_owned()))?;
                *vk_get_instance_proc_addr
            };
            (library, vk_get_instance_proc_addr)
        };

        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), AllocatorHelper::callbacks);

        let mut loader = vks::InstanceProcAddrLoader::from_get_instance_proc_addr(vk_get_instance_proc_addr);
        unsafe {
            loader.load_vk_global();
        }

        let create_info_wrapper = core::VkInstanceCreateInfoWrapper::new(create_info, true);
        let mut instance = ptr::null_mut();
        let res = unsafe {
            loader.vk_global.vkCreateInstance(&create_info_wrapper.vks_struct, allocation_callbacks, &mut instance)
        };
        if res != vks::vk::VK_SUCCESS {
            return Err(res.into());
        }

        unsafe {
            loader.load_vk(instance);
            create_info.enabled_extensions.load_instance(&mut loader, instance);
        }

        let debug_report_callback = if let Some(ref chain) = create_info.chain {
            if let Some(ref debug_report_callback_create_info_ext) = chain.debug_report_callback_create_info_ext {
                Some(Arc::clone(&debug_report_callback_create_info_ext.callback))
            }
            else {
                None
            }
        }
        else {
            None
        };

        Ok(Instance(Arc::new(Inner {
            handle: instance,
            allocator: allocator_helper,
            loader: loader,
            library: library,
            enabled_extensions: create_info.enabled_extensions.clone(),
            debug_report_callback: debug_report_callback,
            display_mode_allocators: Mutex::new(Vec::new()),
        })))
    }

    /// See [`vkEnumeratePhysicalDevices`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumeratePhysicalDevices)
    pub fn enumerate_physical_devices(&self) -> Result<Vec<PhysicalDevice>, core::Error> {
        let mut num_physical_devices = 0;
        let res = unsafe {
            self.loader().vk.vkEnumeratePhysicalDevices(self.handle(), &mut num_physical_devices, ptr::null_mut())
        };
        if res != vks::vk::VK_SUCCESS {
            return Err(res.into());
        }

        let mut physical_devices = Vec::with_capacity(num_physical_devices as usize);
        let res = unsafe {
            self.loader().vk.vkEnumeratePhysicalDevices(self.handle(), &mut num_physical_devices, physical_devices.as_mut_ptr())
        };
        if res != vks::vk::VK_SUCCESS {
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
    pub fn enumerate_instance_layer_properties<B>() -> Result<B, EarlyInstanceError>
        where B: FromIterator<core::LayerProperties>
    {
        unsafe {
            let library = libloading::Library::new(vks::VULKAN_LIBRARY_NAME)
                .map_err(|_| EarlyInstanceError::LoadLibraryFailed(vks::VULKAN_LIBRARY_NAME.to_owned()))?;
            let vk_get_instance_proc_addr: libloading::Symbol<vks::vk::PFN_vkGetInstanceProcAddr> = library
                .get(VK_GET_INSTANCE_PROC_ADDR.as_bytes())
                .map_err(|_| EarlyInstanceError::SymbolNotFound(VK_GET_INSTANCE_PROC_ADDR.to_owned()))?;

            let mut loader = vks::instance_proc_addr_loader::VkGlobal::new();
            loader.load(*vk_get_instance_proc_addr, ptr::null_mut());

            let mut num_layer_properties = 0;
            let res = loader.vkEnumerateInstanceLayerProperties(&mut num_layer_properties, ptr::null_mut());
            if res != vks::vk::VK_SUCCESS {
                return Err(res.into());
            }

            let mut layer_properties = Vec::with_capacity(num_layer_properties as usize);
            layer_properties.set_len(num_layer_properties as usize);
            let res = loader.vkEnumerateInstanceLayerProperties(&mut num_layer_properties, layer_properties.as_mut_ptr());
            if res != vks::vk::VK_SUCCESS {
                return Err(res.into());
            }

            Ok(layer_properties.iter().map(From::from).collect())
        }
    }

    /// See [`vkEnumerateInstanceExtensionProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumerateInstanceExtensionProperties)
    pub fn get_instance_extension_properties(layer_name: Option<&str>) -> Result<core::InstanceExtensionsProperties, EarlyInstanceError> {
        unsafe {
            let library = libloading::Library::new(vks::VULKAN_LIBRARY_NAME)
                .map_err(|_| EarlyInstanceError::LoadLibraryFailed(vks::VULKAN_LIBRARY_NAME.to_owned()))?;
            let vk_get_instance_proc_addr: libloading::Symbol<vks::vk::PFN_vkGetInstanceProcAddr> = library
                .get(VK_GET_INSTANCE_PROC_ADDR.as_bytes())
                .map_err(|_| EarlyInstanceError::SymbolNotFound(VK_GET_INSTANCE_PROC_ADDR.to_owned()))?;

            let mut loader = vks::instance_proc_addr_loader::VkGlobal::new();
            loader.load(*vk_get_instance_proc_addr, ptr::null_mut());

            let layer_name_cstr = utils::cstr_from_str(layer_name);

            let mut num_extension_properties = 0;
            let res = loader.vkEnumerateInstanceExtensionProperties(layer_name_cstr.1, &mut num_extension_properties, ptr::null_mut());
            if res != vks::vk::VK_SUCCESS {
                return Err(res.into());
            }

            let mut extension_properties = Vec::with_capacity(num_extension_properties as usize);
            extension_properties.set_len(num_extension_properties as usize);
            let res = loader.vkEnumerateInstanceExtensionProperties(layer_name_cstr.1, &mut num_extension_properties, extension_properties.as_mut_ptr());
            if res != vks::vk::VK_SUCCESS {
                return Err(res.into());
            }

            let mut res = core::InstanceExtensionsProperties::new();
            for extension in extension_properties {
                let name = CStr::from_ptr(extension.extensionName.as_ptr()).to_str().unwrap();
                res.add(name, extension.specVersion);
            }

            Ok(res)
        }
    }

    /// See [`vkCreateDebugReportCallbackEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDebugReportCallbackEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    pub fn create_debug_report_callback_ext(&self, create_info: &ext_debug_report::DebugReportCallbackCreateInfoExt, allocator: Option<Box<core::Allocator>>) -> Result<ext_debug_report::DebugReportCallbackExt, core::Error> {
        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), AllocatorHelper::callbacks);
        let create_info_wrapper = ext_debug_report::VkDebugReportCallbackCreateInfoEXTWrapper::new(create_info, true);

        let mut debug_report_callback = Default::default();
        let res = unsafe {
            self.loader().ext_debug_report.vkCreateDebugReportCallbackEXT(self.handle(), &create_info_wrapper.vks_struct, allocation_callbacks, &mut debug_report_callback)
        };

        if res == vks::vk::VK_SUCCESS {
            Ok(ext_debug_report::DebugReportCallbackExt::new(debug_report_callback, true, self.clone(), allocator_helper, Some(create_info_wrapper.callback_helper)))
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkDebugReportMessageEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDebugReportMessageEXT)
    /// and extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    #[cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
    pub fn debug_report_message_ext(&self, flags: ext_debug_report::DebugReportFlagsExt, object_type: ext_debug_report::DebugReportObjectTypeExt, object: u64, location: usize, message_code: i32, layer_prefix: &str, message: &str) {
        use std::ffi::CString;

        let layer_prefix = CString::new(layer_prefix).unwrap();
        let message = CString::new(message).unwrap();

        unsafe {
            self.loader().ext_debug_report.vkDebugReportMessageEXT(self.handle(), flags.bits(), object_type.into(), object, location, message_code, layer_prefix.as_ptr(), message.as_ptr());
        }
    }

    /// See [`vkCreateDisplayPlaneSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDisplayPlaneSurfaceKHR)
    /// and extensions [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display),
    /// [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
    pub fn create_display_plane_surface_khr(&self, create_info: &khr_display::DisplaySurfaceCreateInfoKhr, allocator: Option<Box<core::Allocator>>) -> Result<khr_surface::SurfaceKhr, core::Error> {
        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), AllocatorHelper::callbacks);
        let create_info_wrapper = khr_display::VkDisplaySurfaceCreateInfoKHRWrapper::new(create_info, true);

        let mut surface = Default::default();
        let res = unsafe {
            self.loader().khr_display.vkCreateDisplayPlaneSurfaceKHR(self.handle(), &create_info_wrapper.vks_struct, allocation_callbacks, &mut surface)
        };

        if res == vks::vk::VK_SUCCESS {
            Ok(khr_surface::SurfaceKhr::new(surface, true, self.clone(), allocator_helper))
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkCreateXlibSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateXlibSurfaceKHR)
    /// and extensions [`VK_KHR_xlib_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_xlib_surface),
    /// [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
    pub fn create_xlib_surface_khr(&self, create_info: &khr_xlib_surface::XlibSurfaceCreateInfoKhr, allocator: Option<Box<core::Allocator>>) -> Result<khr_surface::SurfaceKhr, core::Error> {
        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), AllocatorHelper::callbacks);
        let create_info_wrapper = khr_xlib_surface::VkXlibSurfaceCreateInfoKHRWrapper::new(create_info, true);

        let mut surface = Default::default();
        let res = unsafe {
            self.loader().khr_xlib_surface.vkCreateXlibSurfaceKHR(self.handle(), &create_info_wrapper.vks_struct, allocation_callbacks, &mut surface)
        };

        if res == vks::vk::VK_SUCCESS {
            Ok(khr_surface::SurfaceKhr::new(surface, true, self.clone(), allocator_helper))
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkCreateWaylandSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateWaylandSurfaceKHR)
    /// and extensions [`VK_KHR_wayland_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_wayland_surface),
    /// [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
    pub fn create_wayland_surface_khr(&self, create_info: &khr_wayland_surface::WaylandSurfaceCreateInfoKhr, allocator: Option<Box<core::Allocator>>) -> Result<khr_surface::SurfaceKhr, core::Error> {
        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), AllocatorHelper::callbacks);
        let create_info_wrapper = khr_wayland_surface::VkWaylandSurfaceCreateInfoKHRWrapper::new(create_info, true);

        let mut surface = Default::default();
        let res = unsafe {
            self.loader().khr_wayland_surface.vkCreateWaylandSurfaceKHR(self.handle(), &create_info_wrapper.vks_struct, allocation_callbacks, &mut surface)
        };

        if res == vks::vk::VK_SUCCESS {
            Ok(khr_surface::SurfaceKhr::new(surface, true, self.clone(), allocator_helper))
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkCreateXcbSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateXcbSurfaceKHR)
    /// and extensions [`VK_KHR_xcb_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_xcb_surface),
    /// [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
    pub fn create_xcb_surface_khr(&self, create_info: &khr_xcb_surface::XcbSurfaceCreateInfoKhr, allocator: Option<Box<core::Allocator>>) -> Result<khr_surface::SurfaceKhr, core::Error> {
        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), AllocatorHelper::callbacks);
        let create_info_wrapper = khr_xcb_surface::VkXcbSurfaceCreateInfoKHRWrapper::new(create_info, true);

        let mut surface = Default::default();
        let res = unsafe {
            self.loader().khr_xcb_surface.vkCreateXcbSurfaceKHR(self.handle(), &create_info_wrapper.vks_struct, allocation_callbacks, &mut surface)
        };

        if res == vks::vk::VK_SUCCESS {
            Ok(khr_surface::SurfaceKhr::new(surface, true, self.clone(), allocator_helper))
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkCreateMirSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateMirSurfaceKHR)
    /// and extensions [`VK_KHR_mir_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_mir_surface),
    /// [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
    pub fn create_mir_surface_khr(&self, create_info: &khr_mir_surface::MirSurfaceCreateInfoKhr, allocator: Option<Box<core::Allocator>>) -> Result<khr_surface::SurfaceKhr, core::Error> {
        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), AllocatorHelper::callbacks);
        let create_info_wrapper = khr_mir_surface::VkMirSurfaceCreateInfoKHRWrapper::new(create_info, true);

        let mut surface = Default::default();
        let res = unsafe {
            self.loader().khr_mir_surface.vkCreateMirSurfaceKHR(self.handle(), &create_info_wrapper.vks_struct, allocation_callbacks, &mut surface)
        };

        if res == vks::vk::VK_SUCCESS {
            Ok(khr_surface::SurfaceKhr::new(surface, true, self.clone(), allocator_helper))
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkCreateAndroidSurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateAndroidSurfaceKHR)
    /// and extensions [`VK_KHR_android_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_android_surface),
    /// [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
    pub fn create_android_surface_khr(&self, create_info: &khr_android_surface::AndroidSurfaceCreateInfoKhr, allocator: Option<Box<core::Allocator>>) -> Result<khr_surface::SurfaceKhr, core::Error> {
        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), AllocatorHelper::callbacks);
        let create_info_wrapper = khr_android_surface::VkAndroidSurfaceCreateInfoKHRWrapper::new(create_info, true);

        let mut surface = Default::default();
        let res = unsafe {
            self.loader().khr_android_surface.vkCreateAndroidSurfaceKHR(self.handle(), &create_info_wrapper.vks_struct, allocation_callbacks, &mut surface)
        };

        if res == vks::vk::VK_SUCCESS {
            Ok(khr_surface::SurfaceKhr::new(surface, true, self.clone(), allocator_helper))
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkCreateWin32SurfaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateWin32SurfaceKHR)
    /// and extensions [`VK_KHR_win32_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_win32_surface),
    /// [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
    pub fn create_win32_surface_khr(&self, create_info: &khr_win32_surface::Win32SurfaceCreateInfoKhr, allocator: Option<Box<core::Allocator>>) -> Result<khr_surface::SurfaceKhr, core::Error> {
        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), AllocatorHelper::callbacks);
        let create_info_wrapper = khr_win32_surface::VkWin32SurfaceCreateInfoKHRWrapper::new(create_info, true);

        let mut surface = Default::default();
        let res = unsafe {
            self.loader().khr_win32_surface.vkCreateWin32SurfaceKHR(self.handle(), &create_info_wrapper.vks_struct, allocation_callbacks, &mut surface)
        };

        if res == vks::vk::VK_SUCCESS {
            Ok(khr_surface::SurfaceKhr::new(surface, true, self.clone(), allocator_helper))
        }
        else {
            Err(res.into())
        }
    }
}

#[derive(Debug)]
struct Inner {
    handle: vks::vk::VkInstance,
    allocator: Option<AllocatorHelper>,
    loader: vks::InstanceProcAddrLoader,
    library: libloading::Library,
    enabled_extensions: core::InstanceExtensions,
    debug_report_callback: Option<Arc<ext_debug_report::DebugReportCallbacksExt>>,
    display_mode_allocators: Mutex<Vec<AllocatorHelper>>,
}

impl Drop for Inner {
    fn drop(&mut self) {
        let allocator = match self.allocator {
            Some(ref allocator) => allocator.callbacks(),
            None => ptr::null(),
        };

        unsafe {
            self.loader.vk.vkDestroyInstance(self.handle, allocator);
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
