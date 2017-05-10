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

use core::allocator_helper::AllocatorHelper;
use core::{self, Device};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ptr;
use std::sync::Arc;
use std::time::Duration;
use utils;
use vks;
use {TryDestroyError, TryDestroyErrorKind, VulkanObject};

/// See [`VkFence`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFence)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fence(Arc<Inner>);

impl VulkanObject for Fence {
    type NativeVulkanObject = vks::VkFence;

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

impl Fence {
    pub(crate) fn new(handle: vks::VkFence, device: Device, allocator: Option<AllocatorHelper>) -> Self {
        Fence(Arc::new(Inner {
            handle: handle,
            device: device,
            allocator: allocator,
        }))
    }

    #[inline]
    pub(crate) fn handle(&self) -> vks::VkFence {
        self.0.handle
    }

    #[inline]
    pub(crate) fn loader(&self) -> &vks::DeviceProcAddrLoader {
        self.0.device.loader()
    }

    #[inline]
    pub(crate) fn device_handle(&self) -> vks::VkDevice {
        self.0.device.handle()
    }

    /// See [`vkWaitForFences`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkWaitForFences)
    pub fn wait_for_fences(fences: &[Self], wait_all: bool, timeout: Option<Duration>) -> Result<bool, core::Error> {
        let loader = fences[0].loader();
        let device = fences[0].device_handle();
        let fences: Vec<_> = fences.iter().map(Fence::handle).collect();

        let timeout = match timeout {
            Some(timeout) => 1000000000u64 * timeout.as_secs() + timeout.subsec_nanos() as u64,
            None => 0,
        };

        let res = unsafe {
            (loader.core.vkWaitForFences)(device, fences.len() as u32, fences.as_ptr(), utils::to_vk_bool(wait_all), timeout)
        };

        match res {
            vks::VK_SUCCESS => Ok(true),
            vks::VK_TIMEOUT => Ok(false),
            _ => Err(res.into()),
        }
    }

    /// See [`vkWaitForFences`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkWaitForFences)
    #[inline]
    pub fn wait_for(&self, wait_all: bool, timeout: Option<Duration>) -> Result<bool, core::Error> {
        Fence::wait_for_fences(&[self.clone()], wait_all, timeout)
    }

    /// See [`vkResetFences`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkResetFences)
    pub fn reset_fences(fences: &[Self]) -> Result<(), core::Error> {
        let loader = fences[0].loader();
        let device = fences[0].device_handle();
        let fences: Vec<_> = fences.iter().map(Fence::handle).collect();

        let res = unsafe {
            (loader.core.vkResetFences)(device, fences.len() as u32, fences.as_ptr())
        };

        if res == vks::VK_SUCCESS {
            Ok(())
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkResetFences`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkResetFences)
    #[inline]
    pub fn reset(&self) -> Result<(), core::Error> {
        Fence::reset_fences(&[self.clone()])
    }

    /// See [`vkGetFenceStatus`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetFenceStatus)
    pub fn get_status(&self) -> Result<bool, core::Error> {
        let res = unsafe {
            (self.loader().core.vkGetFenceStatus)(self.device_handle(), self.handle())
        };

        match res {
            vks::VK_SUCCESS => Ok(true),
            vks::VK_NOT_READY => Ok(false),
            _ => Err(res.into()),
        }
    }
}

#[derive(Debug)]
struct Inner {
    handle: vks::VkFence,
    device: Device,
    allocator: Option<AllocatorHelper>,
}

impl Drop for Inner {
    fn drop(&mut self) {
        let allocator = match self.allocator {
            Some(ref allocator) => allocator.callbacks(),
            None => ptr::null(),
        };

        unsafe {
            (self.device.loader().core.vkDestroyFence)(self.device.handle(), self.handle, allocator);
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
