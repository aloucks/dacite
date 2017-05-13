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

use core::{self, DescriptorPool};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ptr;
use std::sync::Arc;
use vks;
use {TryDestroyError, TryDestroyErrorKind, VulkanObject};

/// See [`VkDescriptorSet`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorSet)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DescriptorSet(Arc<Inner>);

impl VulkanObject for DescriptorSet {
    type NativeVulkanObject = vks::VkDescriptorSet;

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

impl DescriptorSet {
    pub(crate) fn new(handle: vks::VkDescriptorSet, descriptor_pool: DescriptorPool) -> Self {
        DescriptorSet(Arc::new(Inner {
            handle: handle,
            descriptor_pool: descriptor_pool,
        }))
    }

    #[inline]
    pub(crate) fn handle(&self) -> vks::VkDescriptorSet {
        self.0.handle
    }

    #[inline]
    pub(crate) fn loader(&self) -> &vks::DeviceProcAddrLoader {
        self.0.descriptor_pool.loader()
    }

    #[inline]
    pub(crate) fn device_handle(&self) -> vks::VkDevice {
        self.0.descriptor_pool.device_handle()
    }

    /// See [`vkUpdateDescriptorSets`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkUpdateDescriptorSets)
    pub fn update(writes: Option<&[core::WriteDescriptorSet]>, copies: Option<&[core::CopyDescriptorSet]>) {
        let (loader, device_handle) = match (writes, copies) {
            (Some(writes), _) if !writes.is_empty() => (writes[0].dst_set.loader(), writes[0].dst_set.device_handle()),
            (_, Some(copies)) if !copies.is_empty() => (copies[0].src_set.loader(), copies[0].src_set.device_handle()),
            _ => return,
        };

        let (writes_count, writes_ptr, _, _) = match writes {
            Some(writes) => {
                let writes_wrappers: Vec<core::VkWriteDescriptorSetWrapper> = writes.iter().map(From::from).collect();
                let writes: Vec<_> = writes_wrappers.iter().map(AsRef::as_ref).cloned().collect();
                (writes.len() as u32, writes.as_ptr(), Some(writes), Some(writes_wrappers))
            }

            None => (0, ptr::null(), None, None),
        };

        let (copies_count, copies_ptr, _, _) = match copies {
            Some(copies) => {
                let copies_wrappers: Vec<core::VkCopyDescriptorSetWrapper> = copies.iter().map(From::from).collect();
                let copies: Vec<_> = copies_wrappers.iter().map(AsRef::as_ref).cloned().collect();
                (copies.len() as u32, copies.as_ptr(), Some(copies), Some(copies_wrappers))
            }

            None => (0, ptr::null(), None, None),
        };

        unsafe {
            (loader.core.vkUpdateDescriptorSets)(device_handle, writes_count, writes_ptr, copies_count, copies_ptr);
        }
    }
}

#[derive(Debug)]
struct Inner {
    handle: vks::VkDescriptorSet,
    descriptor_pool: DescriptorPool,
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            let res = (self.descriptor_pool.loader().core.vkFreeDescriptorSets)(self.descriptor_pool.device_handle(), self.descriptor_pool.handle(), 1, &self.handle);
            assert_eq!(res, vks::VK_SUCCESS);
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
