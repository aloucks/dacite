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
use core::{self, Device};
use libc::c_void;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::mem;
use std::ptr;
use std::sync::Arc;
use vks;

/// See [`VkQueryPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPool)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryPool(Arc<Inner>);

impl VulkanObject for QueryPool {
    type NativeVulkanObject = vks::vk::VkQueryPool;

    #[inline]
    fn id(&self) -> u64 {
        self.handle()
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

pub struct FromNativeQueryPoolParameters {
    /// `true`, if this `QueryPool` should destroy the underlying Vulkan object, when it is dropped.
    pub owned: bool,

    /// The `Device`, from which this `QueryPool` was created.
    pub device: Device,

    /// An `Allocator` compatible with the one used to create this `QueryPool`.
    ///
    /// This parameter is ignored, if `owned` is `false`.
    pub allocator: Option<Box<core::Allocator>>,
}

impl FromNativeQueryPoolParameters {
    #[inline]
    pub fn new(owned: bool, device: Device, allocator: Option<Box<core::Allocator>>) -> Self {
        FromNativeQueryPoolParameters {
            owned: owned,
            device: device,
            allocator: allocator,
        }
    }
}

impl FromNativeObject for QueryPool {
    type Parameters = FromNativeQueryPoolParameters;

    unsafe fn from_native_object(object: Self::NativeVulkanObject, params: Self::Parameters) -> Self {
        QueryPool::new(object, params.owned, params.device, params.allocator.map(AllocatorHelper::new))
    }
}

impl QueryPool {
    pub(crate) fn new(handle: vks::vk::VkQueryPool, owned: bool, device: Device, allocator: Option<AllocatorHelper>) -> Self {
        QueryPool(Arc::new(Inner {
            handle: handle,
            owned: owned,
            device: device,
            allocator: allocator,
        }))
    }

    #[inline]
    pub(crate) fn handle(&self) -> vks::vk::VkQueryPool {
        self.0.handle
    }

    #[inline]
    pub(crate) fn loader(&self) -> &vks::DeviceProcAddrLoader {
        self.0.device.loader()
    }

    #[inline]
    pub(crate) fn device_handle(&self) -> vks::vk::VkDevice {
        self.0.device.handle()
    }

    /// See [`vkGetQueryPoolResults`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetQueryPoolResults)
    pub fn get_results(&self, first_query: u32, query_count: u32, stride: usize, flags: core::QueryResultFlags, results: &mut [core::QueryResult]) -> Result<bool, core::Error> {
        if flags.contains(core::QueryResultFlags::RESULT_64) {
            let mut data: Vec<u64> = Vec::with_capacity(results.len());
            let data_size = results.len() * mem::size_of::<u64>();
            let stride_u64 = (stride * mem::size_of::<u64>()) as u64;

            let res = unsafe {
                data.set_len(results.len());
                self.loader().vk.vkGetQueryPoolResults(self.device_handle(), self.handle(), first_query, query_count, data_size, data.as_mut_ptr() as *mut c_void, stride_u64, flags.bits())
            };

            match res {
                vks::vk::VK_SUCCESS => {
                    for (&src, dst) in data.iter().zip(results.iter_mut()) {
                        *dst = core::QueryResult::U64(src);
                    }

                    Ok(true)
                }

                vks::vk::VK_NOT_READY => Ok(false),
                _ => Err(res.into()),
            }
        }
        else {
            let mut data: Vec<u32> = Vec::with_capacity(results.len());
            let data_size = results.len() * mem::size_of::<u32>();
            let stride_u32 = (stride * mem::size_of::<u32>()) as u64;

            let res = unsafe {
                data.set_len(results.len());
                self.loader().vk.vkGetQueryPoolResults(self.device_handle(), self.handle(), first_query, query_count, data_size, data.as_mut_ptr() as *mut c_void, stride_u32, flags.bits())
            };

            match res {
                vks::vk::VK_SUCCESS => {
                    for (&src, dst) in data.iter().zip(results.iter_mut()) {
                        *dst = core::QueryResult::U32(src);
                    }

                    Ok(true)
                }

                vks::vk::VK_NOT_READY => Ok(false),
                _ => Err(res.into()),
            }
        }
    }
}

#[derive(Debug)]
struct Inner {
    handle: vks::vk::VkQueryPool,
    owned: bool,
    device: Device,
    allocator: Option<AllocatorHelper>,
}

impl Drop for Inner {
    fn drop(&mut self) {
        if self.owned {
            let allocator = match self.allocator {
                Some(ref allocator) => allocator.callbacks(),
                None => ptr::null(),
            };

            unsafe {
                self.device.loader().vk.vkDestroyQueryPool(self.device.handle(), self.handle, allocator);
            }
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
