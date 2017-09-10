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
use core;
use khr_display::{self, DisplayModeKhr};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ptr;
use vks;

/// See [`VkDisplayKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayKHR)
#[derive(Debug, Clone)]
pub struct DisplayKhr {
    pub(crate) handle: vks::khr_display::VkDisplayKHR,
    physical_device: core::PhysicalDevice,
}

unsafe impl Send for DisplayKhr { }

unsafe impl Sync for DisplayKhr { }

impl PartialEq for DisplayKhr {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}

impl Eq for DisplayKhr { }

impl PartialOrd for DisplayKhr {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.handle.partial_cmp(&other.handle)
    }
}

impl Ord for DisplayKhr {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.handle.cmp(&other.handle)
    }
}

impl Hash for DisplayKhr {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.handle.hash(state);
    }
}

impl VulkanObject for DisplayKhr {
    type NativeVulkanObject = vks::khr_display::VkDisplayKHR;

    #[inline]
    fn id(&self) -> u64 {
        self.handle
    }

    #[inline]
    fn as_native_vulkan_object(&self) -> Self::NativeVulkanObject {
        self.handle
    }

    fn try_destroy(self) -> Result<(), TryDestroyError<Self>> {
        Err(TryDestroyError::new(self, TryDestroyErrorKind::Unsupported))
    }
}

impl FromNativeObject for DisplayKhr {
    type Parameters = core::PhysicalDevice;

    unsafe fn from_native_object(object: Self::NativeVulkanObject, params: Self::Parameters) -> Self {
        DisplayKhr::new(object, params)
    }
}

impl DisplayKhr {
    pub(crate) fn new(display: vks::khr_display::VkDisplayKHR, physical_device: core::PhysicalDevice) -> Self {
        DisplayKhr {
            handle: display,
            physical_device: physical_device,
        }
    }

    #[inline]
    pub(crate) fn loader(&self) -> &vks::InstanceProcAddrLoader {
        self.physical_device.loader()
    }

    #[inline]
    pub(crate) fn physical_device_handle(&self) -> vks::core::VkPhysicalDevice {
        self.physical_device.handle
    }

    #[inline]
    pub(crate) fn instance(&self) -> &core::Instance {
        &self.physical_device.instance
    }

    /// See [`vkGetDisplayModePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDisplayModePropertiesKHR)
    pub fn get_display_mode_properties_khr(&self) -> Result<Vec<khr_display::DisplayModePropertiesKhr>, core::Error> {
        let mut len = 0;
        let res = unsafe {
            self.loader().khr_display.vkGetDisplayModePropertiesKHR(self.physical_device_handle(), self.handle, &mut len, ptr::null_mut())
        };

        if res != vks::core::VK_SUCCESS {
            return Err(res.into());
        }

        let mut properties = Vec::with_capacity(len as usize);
        let res = unsafe {
            properties.set_len(len as usize);
            self.loader().khr_display.vkGetDisplayModePropertiesKHR(self.physical_device_handle(), self.handle, &mut len, properties.as_mut_ptr())
        };

        if res == vks::core::VK_SUCCESS {
            Ok(properties.iter().map(|p| khr_display::DisplayModePropertiesKhr::from_vks(p, self.clone())).collect())
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkCreateDisplayModeKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDisplayModeKHR)
    pub fn create_display_mode_khr(&self, create_info: &khr_display::DisplayModeCreateInfoKhr, allocator: Option<Box<core::Allocator>>) -> Result<DisplayModeKhr, core::Error> {
        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), AllocatorHelper::callbacks);
        let create_info_wrapper = khr_display::VkDisplayModeCreateInfoKHRWrapper::new(create_info, true);

        let mut display_mode = Default::default();
        let res = unsafe {
            self.loader().khr_display.vkCreateDisplayModeKHR(self.physical_device_handle(), self.handle, &create_info_wrapper.vks_struct, allocation_callbacks, &mut display_mode)
        };

        if res == vks::core::VK_SUCCESS {
            if let Some(allocator_helper) = allocator_helper {
                self.instance().add_display_mode_allocator(allocator_helper);
            }

            Ok(DisplayModeKhr::new(display_mode, self.clone()))
        }
        else {
            Err(res.into())
        }
    }
}
