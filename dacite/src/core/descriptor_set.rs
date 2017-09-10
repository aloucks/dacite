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
use core::{self, DescriptorPool};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ptr;
use vks;

/// See [`VkDescriptorSet`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorSet)
#[derive(Debug, Clone)]
pub struct DescriptorSet {
    handle: vks::core::VkDescriptorSet,
    descriptor_pool: DescriptorPool,
}

impl VulkanObject for DescriptorSet {
    type NativeVulkanObject = vks::core::VkDescriptorSet;

    #[inline]
    fn id(&self) -> u64 {
        self.handle
    }

    #[inline]
    fn as_native_vulkan_object(&self) -> Self::NativeVulkanObject {
        self.handle
    }

    fn try_destroy(self) -> Result<(), TryDestroyError<Self>> {
        self.free().map_err(|e| TryDestroyError::new(self, TryDestroyErrorKind::VulkanError(e)))
    }
}

impl FromNativeObject for DescriptorSet {
    type Parameters = DescriptorPool;

    unsafe fn from_native_object(object: Self::NativeVulkanObject, params: Self::Parameters) -> Self {
        DescriptorSet::new(object, params)
    }
}

unsafe impl Send for DescriptorSet { }

unsafe impl Sync for DescriptorSet { }

impl PartialEq for DescriptorSet {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}

impl Eq for DescriptorSet { }

impl PartialOrd for DescriptorSet {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.handle.partial_cmp(&other.handle)
    }
}

impl Ord for DescriptorSet {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.handle.cmp(&other.handle)
    }
}

impl Hash for DescriptorSet {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.handle.hash(state);
    }
}

impl DescriptorSet {
    pub(crate) fn new(handle: vks::core::VkDescriptorSet, descriptor_pool: DescriptorPool) -> Self {
        DescriptorSet {
            handle: handle,
            descriptor_pool: descriptor_pool,
        }
    }

    #[inline]
    pub(crate) fn handle(&self) -> vks::core::VkDescriptorSet {
        self.handle
    }

    #[inline]
    pub(crate) fn loader(&self) -> &vks::DeviceProcAddrLoader {
        self.descriptor_pool.loader()
    }

    #[inline]
    pub(crate) fn device_handle(&self) -> vks::core::VkDevice {
        self.descriptor_pool.device_handle()
    }

    /// See [`vkUpdateDescriptorSets`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkUpdateDescriptorSets)
    pub fn update(writes: Option<&[core::WriteDescriptorSet]>, copies: Option<&[core::CopyDescriptorSet]>) {
        let (loader, device_handle) = match (writes, copies) {
            (Some(writes), _) if !writes.is_empty() => (writes[0].dst_set.loader(), writes[0].dst_set.device_handle()),
            (_, Some(copies)) if !copies.is_empty() => (copies[0].src_set.loader(), copies[0].src_set.device_handle()),
            _ => return,
        };

        #[allow(unused_variables)]
        let (writes_count, writes_ptr, writes, writes_wrappers) = match writes {
            Some(writes) => {
                let writes_wrappers: Vec<_> = writes.iter().map(|w| core::VkWriteDescriptorSetWrapper::new(w, true)).collect();
                let writes: Vec<_> = writes_wrappers.iter().map(|w| w.vks_struct).collect();
                (writes.len() as u32, writes.as_ptr(), Some(writes), Some(writes_wrappers))
            }

            None => (0, ptr::null(), None, None),
        };

        #[allow(unused_variables)]
        let (copies_count, copies_ptr, copies, copies_wrappers) = match copies {
            Some(copies) => {
                let copies_wrappers: Vec<_> = copies.iter().map(|c| core::VkCopyDescriptorSetWrapper::new(c, true)).collect();
                let copies: Vec<_> = copies_wrappers.iter().map(|c| c.vks_struct).collect();
                (copies.len() as u32, copies.as_ptr(), Some(copies), Some(copies_wrappers))
            }

            None => (0, ptr::null(), None, None),
        };

        unsafe {
            loader.core.vkUpdateDescriptorSets(device_handle, writes_count, writes_ptr, copies_count, copies_ptr);
        }
    }

    /// See [`vkFreeDescriptorSets`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkFreeDescriptorSets)
    pub fn free(&self) -> Result<(), core::Error> {
        let res = unsafe {
            self.loader().core.vkFreeDescriptorSets(self.device_handle(), self.descriptor_pool.handle(), 1, &self.handle)
        };

        if res == vks::core::VK_SUCCESS {
            Ok(())
        }
        else {
            Err(res.into())
        }
    }
}
