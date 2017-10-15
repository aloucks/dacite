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
use core;
use khr_display::{self, DisplayKhr};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::mem;
use vks;

/// See [`VkDisplayKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayKHR)
#[derive(Debug, Clone)]
pub struct DisplayModeKhr {
    pub(crate) handle: vks::khr_display::VkDisplayModeKHR,
    display: DisplayKhr,
}

unsafe impl Send for DisplayModeKhr { }

unsafe impl Sync for DisplayModeKhr { }

impl PartialEq for DisplayModeKhr {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}

impl Eq for DisplayModeKhr { }

impl PartialOrd for DisplayModeKhr {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.handle.partial_cmp(&other.handle)
    }
}

impl Ord for DisplayModeKhr {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.handle.cmp(&other.handle)
    }
}

impl Hash for DisplayModeKhr {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.handle.hash(state);
    }
}

impl VulkanObject for DisplayModeKhr {
    type NativeVulkanObject = vks::khr_display::VkDisplayModeKHR;

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

impl FromNativeObject for DisplayModeKhr {
    type Parameters = DisplayKhr;

    unsafe fn from_native_object(object: Self::NativeVulkanObject, params: Self::Parameters) -> Self {
        DisplayModeKhr::new(object, params)
    }
}

impl DisplayModeKhr {
    pub(crate) fn new(display_mode: vks::khr_display::VkDisplayModeKHR, display: DisplayKhr) -> Self {
        DisplayModeKhr {
            handle: display_mode,
            display: display,
        }
    }

    #[inline]
    pub(crate) fn loader(&self) -> &vks::InstanceProcAddrLoader {
        self.display.loader()
    }

    #[inline]
    pub(crate) fn physical_device_handle(&self) -> vks::vk::VkPhysicalDevice {
        self.display.physical_device_handle()
    }

    /// See [`vkGetDisplayPlaneCapabilitiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDisplayPlaneCapabilitiesKHR)
    pub fn get_display_plane_capabilities_khr(&self, plane_index: u32) -> Result<khr_display::DisplayPlaneCapabilitiesKhr, core::Error> {
        unsafe {
            let mut capabilities = mem::uninitialized();
            let res = self.loader().khr_display.vkGetDisplayPlaneCapabilitiesKHR(self.physical_device_handle(), self.handle, plane_index, &mut capabilities);

            if res == vks::vk::VK_SUCCESS {
                Ok((&capabilities).into())
            }
            else {
                Err(res.into())
            }
        }
    }
}
