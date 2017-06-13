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

use FromNativeObject;
use TryDestroyError;
use TryDestroyErrorKind;
use VulkanObject;
use core::allocator_helper::AllocatorHelper;
use core::{self, Device, Instance};
use khr_display;
use khr_get_physical_device_properties2;
use khr_surface;
use mir_wrapper;
use std::cmp::Ordering;
use std::ffi::CStr;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::mem;
use std::ptr;
use utils;
use vks;
use wayland_wrapper;
use xcb_wrapper;
use xlib_wrapper;

/// See [`VkPhysicalDevice`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDevice)
#[derive(Debug, Clone)]
pub struct PhysicalDevice {
    pub(crate) handle: vks::VkPhysicalDevice,
    pub(crate) instance: Instance,
}

unsafe impl Send for PhysicalDevice { }

unsafe impl Sync for PhysicalDevice { }

impl PartialEq for PhysicalDevice {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}

impl Eq for PhysicalDevice { }

impl PartialOrd for PhysicalDevice {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.handle.partial_cmp(&other.handle)
    }
}

impl Ord for PhysicalDevice {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.handle.cmp(&other.handle)
    }
}

impl Hash for PhysicalDevice {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.handle.hash(state);
    }
}

impl VulkanObject for PhysicalDevice {
    type NativeVulkanObject = vks::VkPhysicalDevice;

    #[inline]
    fn id(&self) -> u64 {
        self.as_native_vulkan_object() as u64
    }

    #[inline]
    fn as_native_vulkan_object(&self) -> Self::NativeVulkanObject {
        self.handle
    }

    fn try_destroy(self) -> Result<(), TryDestroyError<Self>> {
        Err(TryDestroyError::new(self, TryDestroyErrorKind::Unsupported))
    }
}

impl FromNativeObject for PhysicalDevice {
    type Parameters = Instance;

    unsafe fn from_native_object(object: Self::NativeVulkanObject, params: Self::Parameters) -> Self {
        PhysicalDevice::new(object, params)
    }
}

impl PhysicalDevice {
    pub(crate) fn new(handle: vks::VkPhysicalDevice, instance: Instance) -> Self {
        PhysicalDevice {
            handle: handle,
            instance: instance,
        }
    }

    #[inline]
    pub(crate) fn loader(&self) -> &vks::InstanceProcAddrLoader {
        self.instance.loader()
    }

    /// See [`vkGetPhysicalDeviceProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceProperties)
    pub fn get_properties(&self) -> core::PhysicalDeviceProperties {
        unsafe {
            let mut properties = mem::uninitialized();
            (self.loader().core.vkGetPhysicalDeviceProperties)(self.handle, &mut properties);
            (&properties).into()
        }
    }

    /// See [`vkGetPhysicalDeviceFeatures`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceFeatures)
    pub fn get_features(&self) -> core::PhysicalDeviceFeatures {
        unsafe {
            let mut features = mem::uninitialized();
            (self.loader().core.vkGetPhysicalDeviceFeatures)(self.handle, &mut features);
            (&features).into()
        }
    }

    /// See [`vkEnumerateDeviceLayerProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumerateDeviceLayerProperties)
    pub fn enumerate_device_layer_properties<B>(&self) -> Result<B, core::Error>
        where B: FromIterator<core::LayerProperties>
    {
        unsafe {
            let mut num_layer_properties = 0;
            let res = (self.loader().core.vkEnumerateDeviceLayerProperties)(self.handle, &mut num_layer_properties, ptr::null_mut());
            if res != vks::VK_SUCCESS {
                return Err(res.into());
            }

            let mut layer_properties = Vec::with_capacity(num_layer_properties as usize);
            let res = (self.loader().core.vkEnumerateDeviceLayerProperties)(self.handle, &mut num_layer_properties, layer_properties.as_mut_ptr());
            layer_properties.set_len(num_layer_properties as usize);
            if res != vks::VK_SUCCESS {
                return Err(res.into());
            }

            Ok(layer_properties.iter().map(From::from).collect())
        }
    }

    /// See [`vkEnumerateDeviceExtensionProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumerateDeviceExtensionProperties)
    pub fn get_device_extension_properties(&self, layer_name: Option<&str>) -> Result<core::DeviceExtensionsProperties, core::Error> {
        unsafe {
            let layer_name_cstr = utils::cstr_from_str(layer_name);

            let mut num_extension_properties = 0;
            let res = (self.loader().core.vkEnumerateDeviceExtensionProperties)(self.handle, layer_name_cstr.1, &mut num_extension_properties, ptr::null_mut());
            if res != vks::VK_SUCCESS {
                return Err(res.into());
            }

            let mut extension_properties = Vec::with_capacity(num_extension_properties as usize);
            extension_properties.set_len(num_extension_properties as usize);
            let res = (self.loader().core.vkEnumerateDeviceExtensionProperties)(self.handle, layer_name_cstr.1, &mut num_extension_properties, extension_properties.as_mut_ptr());
            if res != vks::VK_SUCCESS {
                return Err(res.into());
            }

            let mut res = core::DeviceExtensionsProperties::new();
            for extension in extension_properties {
                let name = CStr::from_ptr(extension.extensionName.as_ptr()).to_str().unwrap();
                res.add(name, extension.specVersion);
            }

            Ok(res)
        }
    }

    /// See [`vkGetPhysicalDeviceFormatProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceFormatProperties)
    pub fn get_format_properties(&self, format: core::Format) -> core::FormatProperties {
        let mut properties = unsafe { mem::uninitialized() };

        unsafe {
            (self.loader().core.vkGetPhysicalDeviceFormatProperties)(self.handle, format.into(), &mut properties);
        }

        (&properties).into()
    }

    /// See [`vkGetPhysicalDeviceImageFormatProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceImageFormatProperties)
    pub fn get_image_format_properties(&self, format: core::Format, image_type: core::ImageType, tiling: core::ImageTiling, usage: core::ImageUsageFlags, flags: core::ImageCreateFlags) -> Result<core::ImageFormatProperties, core::Error> {
        let mut properties = unsafe { mem::uninitialized() };

        let res = unsafe {
            (self.loader().core.vkGetPhysicalDeviceImageFormatProperties)(self.handle, format.into(), image_type.into(), tiling.into(), usage, flags, &mut properties)
        };

        if res == vks::VK_SUCCESS {
            Ok((&properties).into())
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkGetPhysicalDeviceSparseImageFormatProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSparseImageFormatProperties)
    pub fn get_sparse_image_format_properties<B>(&self, format: core::Format, image_type: core::ImageType, samples: core::SampleCountFlagBits, usage: core::ImageUsageFlags, tiling: core::ImageTiling) -> B
        where B: FromIterator<core::SparseImageFormatProperties>
    {
        let mut num_properties = 0;
        unsafe {
            (self.loader().core.vkGetPhysicalDeviceSparseImageFormatProperties)(self.handle, format.into(), image_type.into(), samples, usage, tiling.into(), &mut num_properties, ptr::null_mut());
        }

        let mut properties = Vec::with_capacity(num_properties as usize);
        unsafe {
            (self.loader().core.vkGetPhysicalDeviceSparseImageFormatProperties)(self.handle, format.into(), image_type.into(), samples, usage, tiling.into(), &mut num_properties, properties.as_mut_ptr());
            properties.set_len(num_properties as usize);
        }

        properties.iter().map(From::from).collect()
    }

    /// See [`vkGetPhysicalDeviceQueueFamilyProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceQueueFamilyProperties)
    pub fn get_queue_family_properties<B>(&self) -> B
        where B: FromIterator<core::QueueFamilyProperties>
    {
        let mut num_properties = 0;
        unsafe {
            (self.loader().core.vkGetPhysicalDeviceQueueFamilyProperties)(self.handle, &mut num_properties, ptr::null_mut());
        }

        let mut properties = Vec::with_capacity(num_properties as usize);
        unsafe {
            (self.loader().core.vkGetPhysicalDeviceQueueFamilyProperties)(self.handle, &mut num_properties, properties.as_mut_ptr());
            properties.set_len(num_properties as usize);
        }

        properties.iter().map(From::from).collect()
    }

    /// See [`vkGetPhysicalDeviceMemoryProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceMemoryProperties)
    pub fn get_memory_properties(&self) -> core::PhysicalDeviceMemoryProperties {
        let mut properties = unsafe { mem::uninitialized() };

        unsafe {
            (self.loader().core.vkGetPhysicalDeviceMemoryProperties)(self.handle, &mut properties);
        }

        (&properties).into()
    }

    /// See [`vkCreateDevice`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDevice)
    pub fn create_device(&self, create_info: &core::DeviceCreateInfo, allocator: Option<Box<core::Allocator>>) -> Result<Device, core::Error> {
        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), AllocatorHelper::callbacks);
        let create_info_wrapper = core::VkDeviceCreateInfoWrapper::new(create_info, true);

        let mut device = ptr::null_mut();
        let res = unsafe {
            (self.loader().core.vkCreateDevice)(self.handle, &create_info_wrapper.vks_struct, allocation_callbacks, &mut device)
        };

        if res == vks::VK_SUCCESS {
            let mut loader = vks::DeviceProcAddrLoader::from_get_device_proc_addr(self.loader().core.vkGetDeviceProcAddr);

            unsafe {
                loader.load_core(device);
                create_info.enabled_extensions.load_device(&mut loader, device);
                self.instance.get_enabled_extensions().load_device(&mut loader, device);
            }

            Ok(Device::new(device, self.instance.clone(), allocator_helper, loader, create_info.enabled_extensions.clone()))
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkGetPhysicalDeviceSurfaceSupportKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSurfaceSupportKHR)
    /// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
    pub fn get_surface_support_khr(&self, queue_family_index: u32, surface: &khr_surface::SurfaceKhr) -> Result<bool, core::Error> {
        let mut supported = vks::VK_FALSE;
        let res = unsafe {
            (self.loader().khr_surface.vkGetPhysicalDeviceSurfaceSupportKHR)(self.handle, queue_family_index, surface.handle(), &mut supported)
        };

        if res == vks::VK_SUCCESS {
            Ok(utils::from_vk_bool(supported))
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkGetPhysicalDeviceSurfaceCapabilitiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSurfaceCapabilitiesKHR)
    /// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
    pub fn get_surface_capabilities_khr(&self, surface: &khr_surface::SurfaceKhr) -> Result<khr_surface::SurfaceCapabilitiesKhr, core::Error> {
        unsafe {
            let mut capabilities = mem::uninitialized();
            let res = (self.loader().khr_surface.vkGetPhysicalDeviceSurfaceCapabilitiesKHR)(self.handle, surface.handle(), &mut capabilities);

            if res == vks::VK_SUCCESS {
                Ok((&capabilities).into())
            }
            else {
                Err(res.into())
            }
        }
    }

    /// See [`vkGetPhysicalDeviceSurfaceFormatsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSurfaceFormatsKHR)
    /// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
    pub fn get_surface_formats_khr<B>(&self, surface: &khr_surface::SurfaceKhr) -> Result<B, core::Error>
        where B: FromIterator<khr_surface::SurfaceFormatKhr>
    {
        let mut num_formats = 0;
        let res = unsafe {
            (self.loader().khr_surface.vkGetPhysicalDeviceSurfaceFormatsKHR)(self.handle, surface.handle(), &mut num_formats, ptr::null_mut())
        };

        if (res != vks::VK_SUCCESS) && (res != vks::VK_INCOMPLETE) {
            return Err(res.into());
        }

        let mut formats = Vec::with_capacity(num_formats as usize);
        let res = unsafe {
            (self.loader().khr_surface.vkGetPhysicalDeviceSurfaceFormatsKHR)(self.handle, surface.handle(), &mut num_formats, formats.as_mut_ptr())
        };

        if res == vks::VK_SUCCESS {
            unsafe {
                formats.set_len(num_formats as usize);
            }

            Ok(formats.iter().map(From::from).collect())
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkGetPhysicalDeviceSurfacePresentModesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSurfacePresentModesKHR)
    /// and extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
    pub fn get_surface_present_modes_khr<B>(&self, surface: &khr_surface::SurfaceKhr) -> Result<B, core::Error>
        where B: FromIterator<khr_surface::PresentModeKhr>
    {
        let mut num_modes = 0;
        let res = unsafe {
            (self.loader().khr_surface.vkGetPhysicalDeviceSurfacePresentModesKHR)(self.handle, surface.handle(), &mut num_modes, ptr::null_mut())
        };

        if (res != vks::VK_SUCCESS) && (res != vks::VK_INCOMPLETE) {
            return Err(res.into());
        }

        let mut modes = Vec::with_capacity(num_modes as usize);
        let res = unsafe {
            (self.loader().khr_surface.vkGetPhysicalDeviceSurfacePresentModesKHR)(self.handle, surface.handle(), &mut num_modes, modes.as_mut_ptr())
        };

        if res == vks::VK_SUCCESS {
            unsafe {
                modes.set_len(num_modes as usize);
            }

            Ok(modes.into_iter().map(From::from).collect())
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkGetPhysicalDeviceDisplayPropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceDisplayPropertiesKHR)
    /// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
    pub fn get_display_properties_khr(&self) -> Result<Vec<khr_display::DisplayPropertiesKhr>, core::Error> {
        let mut len = 0;
        let res = unsafe {
            (self.loader().khr_display.vkGetPhysicalDeviceDisplayPropertiesKHR)(self.handle, &mut len, ptr::null_mut())
        };

        if res != vks::VK_SUCCESS {
            return Err(res.into());
        }

        let mut properties = Vec::with_capacity(len as usize);
        let res = unsafe {
            properties.set_len(len as usize);
            (self.loader().khr_display.vkGetPhysicalDeviceDisplayPropertiesKHR)(self.handle, &mut len, properties.as_mut_ptr())
        };

        if res == vks::VK_SUCCESS {
            unsafe {
                Ok(properties.iter().map(|p| khr_display::DisplayPropertiesKhr::from_vks(p, self.clone())).collect())
            }
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkGetPhysicalDeviceDisplayPlanePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceDisplayPlanePropertiesKHR)
    /// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
    pub fn get_display_plane_properties_khr(&self) -> Result<Vec<khr_display::DisplayPlanePropertiesKhr>, core::Error> {
        let mut len = 0;
        let res = unsafe {
            (self.loader().khr_display.vkGetPhysicalDeviceDisplayPlanePropertiesKHR)(self.handle, &mut len, ptr::null_mut())
        };

        if res != vks::VK_SUCCESS {
            return Err(res.into());
        }

        let mut properties = Vec::with_capacity(len as usize);
        let res = unsafe {
            properties.set_len(len as usize);
            (self.loader().khr_display.vkGetPhysicalDeviceDisplayPlanePropertiesKHR)(self.handle, &mut len, properties.as_mut_ptr())
        };

        if res == vks::VK_SUCCESS {
            unsafe {
                Ok(properties.iter().map(|p| khr_display::DisplayPlanePropertiesKhr::from_vks(p, self)).collect())
            }
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkGetDisplayPlaneSupportedDisplaysKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDisplayPlaneSupportedDisplaysKHR)
    /// and extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)
    pub fn get_display_plane_supported_displays_khr(&self, plane_index: u32) -> Result<Vec<khr_display::DisplayKhr>, core::Error> {
        let mut len = 0;
        let res = unsafe {
            (self.loader().khr_display.vkGetDisplayPlaneSupportedDisplaysKHR)(self.handle, plane_index, &mut len, ptr::null_mut())
        };

        if res != vks::VK_SUCCESS {
            return Err(res.into());
        }

        let mut displays = Vec::with_capacity(len as usize);
        let res = unsafe {
            displays.set_len(len as usize);
            (self.loader().khr_display.vkGetDisplayPlaneSupportedDisplaysKHR)(self.handle, plane_index, &mut len, displays.as_mut_ptr())
        };

        if res == vks::VK_SUCCESS {
            Ok(displays.iter().map(|d| khr_display::DisplayKhr::new(*d, self.clone())).collect())
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkGetPhysicalDeviceXlibPresentationSupportKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceXlibPresentationSupportKHR)
    /// and extension [`VK_KHR_xlib_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_xlib_surface)
    pub fn get_xlib_presentation_support_khr(&self, queue_family_index: u32, dpy: *mut xlib_wrapper::Display, visual_id: xlib_wrapper::VisualID) -> bool {
        let res = unsafe {
            (self.loader().khr_xlib_surface.vkGetPhysicalDeviceXlibPresentationSupportKHR)(self.handle, queue_family_index, dpy, visual_id)
        };

        utils::from_vk_bool(res)
    }

    /// See [`vkGetPhysicalDeviceWaylandPresentationSupportKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceWaylandPresentationSupportKHR)
    /// and extension [`VK_KHR_wayland_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_wayland_surface)
    pub fn get_wayland_presentation_support_khr(&self, queue_family_index: u32, display: *mut wayland_wrapper::wl_display) -> bool {
        let res = unsafe {
            (self.loader().khr_wayland_surface.vkGetPhysicalDeviceWaylandPresentationSupportKHR)(self.handle, queue_family_index, display)
        };

        utils::from_vk_bool(res)
    }

    /// See [`vkGetPhysicalDeviceXcbPresentationSupportKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceXcbPresentationSupportKHR)
    /// and extension [`VK_KHR_xcb_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_xcb_surface)
    pub fn get_xcb_presentation_support_khr(&self, queue_family_index: u32, connection: *mut xcb_wrapper::xcb_connection_t, visual_id: xcb_wrapper::xcb_visualid_t) -> bool {
        let res = unsafe {
            (self.loader().khr_xcb_surface.vkGetPhysicalDeviceXcbPresentationSupportKHR)(self.handle, queue_family_index, connection, visual_id)
        };

        utils::from_vk_bool(res)
    }

    /// See [`vkGetPhysicalDeviceMirPresentationSupportKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceMirPresentationSupportKHR)
    /// and extension [`VK_KHR_mir_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_mir_surface)
    pub fn get_mir_presentation_support_khr(&self, queue_family_index: u32, connection: *mut mir_wrapper::MirConnection) -> bool {
        let res = unsafe {
            (self.loader().khr_mir_surface.vkGetPhysicalDeviceMirPresentationSupportKHR)(self.handle, queue_family_index, connection)
        };

        utils::from_vk_bool(res)
    }

    /// See [`vkGetPhysicalDeviceWin32PresentationSupportKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceWin32PresentationSupportKHR)
    /// and extension [`VK_KHR_win32_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_win32_surface)
    pub fn get_win32_presentation_support_khr(&self, queue_family_index: u32) -> bool {
        let res = unsafe {
            (self.loader().khr_win32_surface.vkGetPhysicalDeviceWin32PresentationSupportKHR)(self.handle, queue_family_index)
        };

        utils::from_vk_bool(res)
    }

    /// See [`vkGetPhysicalDeviceFeatures2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceFeatures2KHR)
    /// and extension [`VK_KHR_get_physical_device_properties2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_physical_device_properties2)
    pub fn get_features2_khr(&self, chain_query: Option<&khr_get_physical_device_properties2::PhysicalDeviceFeatures2ChainQueryKhr>) -> khr_get_physical_device_properties2::PhysicalDeviceFeatures2Khr {
        let mut chain_query_wrapper = khr_get_physical_device_properties2::PhysicalDeviceFeatures2ChainQueryKhrWrapper::new_optional(chain_query);
        unsafe {
            (self.loader().khr_get_physical_device_properties2.vkGetPhysicalDeviceFeatures2KHR)(self.handle, &mut chain_query_wrapper.vks_struct);
            khr_get_physical_device_properties2::PhysicalDeviceFeatures2Khr::from_vks(&chain_query_wrapper.vks_struct, true)
        }
    }

    /// See [`vkGetPhysicalDeviceProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceProperties2KHR)
    /// and extension [`VK_KHR_get_physical_device_properties2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_physical_device_properties2)
    pub fn get_properties2_khr(&self, chain_query: Option<&khr_get_physical_device_properties2::PhysicalDeviceProperties2ChainQueryKhr>) -> khr_get_physical_device_properties2::PhysicalDeviceProperties2Khr {
        let mut chain_query_wrapper = khr_get_physical_device_properties2::PhysicalDeviceProperties2ChainQueryKhrWrapper::new_optional(chain_query);
        unsafe {
            (self.loader().khr_get_physical_device_properties2.vkGetPhysicalDeviceProperties2KHR)(self.handle, &mut chain_query_wrapper.vks_struct);
            khr_get_physical_device_properties2::PhysicalDeviceProperties2Khr::from_vks(&chain_query_wrapper.vks_struct, true)
        }
    }

    /// See [`vkGetPhysicalDeviceFormatProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceFormatProperties2KHR)
    /// and extension [`VK_KHR_get_physical_device_properties2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_physical_device_properties2)
    pub fn get_format_properties2_khr(&self, format: core::Format, chain_query: Option<&khr_get_physical_device_properties2::FormatProperties2ChainQueryKhr>) -> khr_get_physical_device_properties2::FormatProperties2Khr {
        let mut chain_query_wrapper = khr_get_physical_device_properties2::FormatProperties2ChainQueryKhrWrapper::new_optional(chain_query);
        unsafe {
            (self.loader().khr_get_physical_device_properties2.vkGetPhysicalDeviceFormatProperties2KHR)(self.handle, format.into(), &mut chain_query_wrapper.vks_struct);
            khr_get_physical_device_properties2::FormatProperties2Khr::from_vks(&chain_query_wrapper.vks_struct, true)
        }
    }

    /// See [`vkGetPhysicalDeviceImageFormatProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceImageFormatProperties2KHR)
    /// and extension [`VK_KHR_get_physical_device_properties2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_physical_device_properties2)
    pub fn get_image_format_properties2_khr(&self, image_format_info: &khr_get_physical_device_properties2::PhysicalDeviceImageFormatInfo2Khr, chain_query: Option<&khr_get_physical_device_properties2::ImageFormatProperties2ChainQueryKhr>) -> Result<khr_get_physical_device_properties2::ImageFormatProperties2Khr, core::Error> {
        let image_format_info_wrapper = khr_get_physical_device_properties2::VkPhysicalDeviceImageFormatInfo2KHRWrapper::new(image_format_info, true);
        let mut chain_query_wrapper = khr_get_physical_device_properties2::ImageFormatProperties2ChainQueryKhrWrapper::new_optional(chain_query);

        unsafe {
            let res = (self.loader().khr_get_physical_device_properties2.vkGetPhysicalDeviceImageFormatProperties2KHR)(self.handle, &image_format_info_wrapper.vks_struct, &mut chain_query_wrapper.vks_struct);

            if res == vks::VK_SUCCESS {
                Ok(khr_get_physical_device_properties2::ImageFormatProperties2Khr::from_vks(&chain_query_wrapper.vks_struct, true))
            }
            else {
                Err(res.into())
            }
        }
    }

    /// See [`vkGetPhysicalDeviceQueueFamilyProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceQueueFamilyProperties2KHR)
    /// and extension [`VK_KHR_get_physical_device_properties2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_physical_device_properties2)
    pub fn get_queue_family_properties2_khr<B>(&self, chain_query: Option<&khr_get_physical_device_properties2::QueueFamilyProperties2ChainQueryKhr>) -> B
        where B: FromIterator<khr_get_physical_device_properties2::QueueFamilyProperties2Khr>
    {
        unsafe {
            let mut num = 0;
            (self.loader().khr_get_physical_device_properties2.vkGetPhysicalDeviceQueueFamilyProperties2KHR)(self.handle, &mut num, ptr::null_mut());

            let mut chain_query_wrappers = Vec::with_capacity(num as usize);
            for _ in 0..num {
                chain_query_wrappers.push(khr_get_physical_device_properties2::QueueFamilyProperties2ChainQueryKhrWrapper::new_optional(chain_query));
            }

            let mut vks_structs: Vec<_> = chain_query_wrappers.iter().map(|w| w.vks_struct).collect();
            (self.loader().khr_get_physical_device_properties2.vkGetPhysicalDeviceQueueFamilyProperties2KHR)(self.handle, &mut num, vks_structs.as_mut_ptr());

            vks_structs.iter().map(|p| khr_get_physical_device_properties2::QueueFamilyProperties2Khr::from_vks(p, true)).collect()
        }
    }

    /// See [`vkGetPhysicalDeviceMemoryProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceMemoryProperties2KHR)
    /// and extension [`VK_KHR_get_physical_device_properties2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_physical_device_properties2)
    pub fn get_memory_properties2_khr(&self, chain_query: Option<&khr_get_physical_device_properties2::PhysicalDeviceMemoryProperties2ChainQueryKhr>) -> khr_get_physical_device_properties2::PhysicalDeviceMemoryProperties2Khr {
        let mut chain_query_wrapper = khr_get_physical_device_properties2::PhysicalDeviceMemoryProperties2ChainQueryKhrWrapper::new_optional(chain_query);
        unsafe {
            (self.loader().khr_get_physical_device_properties2.vkGetPhysicalDeviceMemoryProperties2KHR)(self.handle, &mut chain_query_wrapper.vks_struct);
            khr_get_physical_device_properties2::PhysicalDeviceMemoryProperties2Khr::from_vks(&chain_query_wrapper.vks_struct, true)
        }
    }

    /// See [`vkGetPhysicalDeviceSparseImageFormatProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSparseImageFormatProperties2KHR)
    /// and extension [`VK_KHR_get_physical_device_properties2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_physical_device_properties2)
    pub fn get_sparse_image_format_properties2_khr<B>(&self, format_info: &khr_get_physical_device_properties2::PhysicalDeviceSparseImageFormatInfo2Khr, chain_query: Option<&khr_get_physical_device_properties2::SparseImageFormatProperties2ChainQueryKhr>) -> B
        where B: FromIterator<khr_get_physical_device_properties2::SparseImageFormatProperties2Khr>
    {
        let format_info_wrapper = khr_get_physical_device_properties2::VkPhysicalDeviceSparseImageFormatInfo2KHRWrapper::new(format_info, true);

        unsafe {
            let mut num = 0;
            (self.loader().khr_get_physical_device_properties2.vkGetPhysicalDeviceSparseImageFormatProperties2KHR)(self.handle, &format_info_wrapper.vks_struct, &mut num, ptr::null_mut());

            let mut chain_query_wrappers = Vec::with_capacity(num as usize);
            for _ in 0..num {
                chain_query_wrappers.push(khr_get_physical_device_properties2::SparseImageFormatProperties2ChainQueryKhrWrapper::new_optional(chain_query));
            }

            let mut vks_structs: Vec<_> = chain_query_wrappers.iter().map(|w| w.vks_struct).collect();
            (self.loader().khr_get_physical_device_properties2.vkGetPhysicalDeviceSparseImageFormatProperties2KHR)(self.handle, &format_info_wrapper.vks_struct, &mut num, vks_structs.as_mut_ptr());

            vks_structs.iter().map(|p| khr_get_physical_device_properties2::SparseImageFormatProperties2Khr::from_vks(p, true)).collect()
        }
    }
}
