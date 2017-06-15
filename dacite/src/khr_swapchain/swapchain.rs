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
use core;
use khr_swapchain::AcquireNextImageResultKhr;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ptr;
use std::sync::Arc;
use vks;
use {TryDestroyError, TryDestroyErrorKind, VulkanObject};

/// See [`VkSwapchainKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSwapchainKHR)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SwapchainKhr(Arc<Inner>);

impl VulkanObject for SwapchainKhr {
    type NativeVulkanObject = vks::VkSwapchainKHR;

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

impl SwapchainKhr {
    pub(crate) fn new(handle: vks::VkSwapchainKHR, device: core::Device, allocator: Option<AllocatorHelper>) -> Self {
        SwapchainKhr(Arc::new(Inner {
            handle: handle,
            device: device,
            allocator: allocator,
        }))
    }

    #[inline]
    pub(crate) fn handle(&self) -> vks::VkSwapchainKHR {
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

    /// See [`vkGetSwapchainImagesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetSwapchainImagesKHR)
    pub fn get_images_khr(&self) -> Result<Vec<core::Image>, core::Error> {
        let mut num = 0;
        let res = unsafe {
            (self.loader().khr_swapchain.vkGetSwapchainImagesKHR)(self.device_handle(), self.handle(), &mut num, ptr::null_mut())
        };

        if res != vks::VK_SUCCESS {
            return Err(res.into());
        }

        let mut images = Vec::with_capacity(num as usize);
        let res = unsafe {
            images.set_len(num as usize);
            (self.loader().khr_swapchain.vkGetSwapchainImagesKHR)(self.device_handle(), self.handle(), &mut num, images.as_mut_ptr())
        };

        if res == vks::VK_SUCCESS {
            Ok(images.iter().map(|i| core::Image::new(*i, self.0.device.clone(), None, false)).collect())
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkAcquireNextImageKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkAcquireNextImageKHR)
    pub fn acquire_next_image_khr(&self, timeout: core::Timeout, semaphore: Option<&core::Semaphore>, fence: Option<&core::Fence>) -> Result<AcquireNextImageResultKhr, core::Error> {
        let semaphore = semaphore.map_or(ptr::null_mut(), |s| s.handle());
        let fence = fence.map_or(ptr::null_mut(), |f| f.handle());

        let mut index = 0;
        let res = unsafe {
            (self.loader().khr_swapchain.vkAcquireNextImageKHR)(self.device_handle(), self.handle(), timeout.as_nanoseconds(), semaphore, fence, &mut index)
        };

        match res {
            vks::VK_SUCCESS => Ok(AcquireNextImageResultKhr::Index(index as usize)),
            vks::VK_TIMEOUT => Ok(AcquireNextImageResultKhr::Timeout),
            vks::VK_NOT_READY => Ok(AcquireNextImageResultKhr::NotReady),
            vks::VK_SUBOPTIMAL_KHR => Ok(AcquireNextImageResultKhr::Suboptimal(index as usize)),
            _ => Err(res.into()),
        }
    }
}

#[derive(Debug)]
struct Inner {
    handle: vks::VkSwapchainKHR,
    device: core::Device,
    allocator: Option<AllocatorHelper>,
}

impl Drop for Inner {
    fn drop(&mut self) {
        let allocator = match self.allocator {
            Some(ref allocator) => allocator.callbacks(),
            None => ptr::null(),
        };

        unsafe {
            (self.device.loader().khr_swapchain.vkDestroySwapchainKHR)(self.device.handle(), self.handle, allocator);
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
