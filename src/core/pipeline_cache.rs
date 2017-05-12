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
use libc::c_void;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ptr;
use std::sync::Arc;
use vks;
use {TryDestroyError, TryDestroyErrorKind, VulkanObject};

/// See [`VkPipelineCache`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCache)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineCache(Arc<Inner>);

impl VulkanObject for PipelineCache {
    type NativeVulkanObject = vks::VkPipelineCache;

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

impl PipelineCache {
    pub(crate) fn new(handle: vks::VkPipelineCache, device: Device, allocator: Option<AllocatorHelper>) -> Self {
        PipelineCache(Arc::new(Inner {
            handle: handle,
            device: device,
            allocator: allocator,
        }))
    }

    #[inline]
    pub(crate) fn handle(&self) -> vks::VkPipelineCache {
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

    /// See [`vkMergePipelineCaches`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkMergePipelineCaches)
    pub fn merge(&self, caches: &[Self]) -> Result<(), core::Error> {
        let caches: Vec<_> = caches.iter().map(PipelineCache::handle).collect();

        let res = unsafe {
            (self.loader().core.vkMergePipelineCaches)(self.device_handle(), self.handle(), caches.len() as u32, caches.as_ptr())
        };

        if res == vks::VK_SUCCESS {
            Ok(())
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkGetPipelineCacheData`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPipelineCacheData)
    pub fn get_data(&self, max_size: Option<usize>) -> Result<Vec<u8>, core::Error> {
        if let Some(mut max_size) = max_size {
            let mut data: Vec<u8> = Vec::with_capacity(max_size);
            let res = unsafe {
                data.set_len(max_size);
                (self.loader().core.vkGetPipelineCacheData)(self.device_handle(), self.handle(), &mut max_size, data.as_mut_ptr() as *mut c_void)
            };

            if (res == vks::VK_SUCCESS) || (res == vks::VK_INCOMPLETE) {
                Ok(data)
            }
            else {
                Err(res.into())
            }
        }
        else {
            let mut size = 0;
            let res = unsafe {
                (self.loader().core.vkGetPipelineCacheData)(self.device_handle(), self.handle(), &mut size, ptr::null_mut())
            };

            if (res != vks::VK_SUCCESS) && (res != vks::VK_INCOMPLETE) {
                return Err(res.into());
            }

            let mut data: Vec<u8> = Vec::with_capacity(size);
            let res = unsafe {
                data.set_len(size);
                (self.loader().core.vkGetPipelineCacheData)(self.device_handle(), self.handle(), &mut size, data.as_mut_ptr() as *mut c_void)
            };

            if (res == vks::VK_SUCCESS) || (res == vks::VK_INCOMPLETE) {
                Ok(data)
            }
            else {
                Err(res.into())
            }
        }
    }
}

#[derive(Debug)]
struct Inner {
    handle: vks::VkPipelineCache,
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
            (self.device.loader().core.vkDestroyPipelineCache)(self.device.handle(), self.handle, allocator);
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
