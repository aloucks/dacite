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
use core::{self, DescriptorSet, Device};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ptr;
use std::sync::Arc;
use vks;
use {TryDestroyError, TryDestroyErrorKind, VulkanObject};

/// See [`VkDescriptorPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorPool)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DescriptorPool(Arc<Inner>);

impl VulkanObject for DescriptorPool {
    type NativeVulkanObject = vks::VkDescriptorPool;

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

impl DescriptorPool {
    pub(crate) fn new(handle: vks::VkDescriptorPool, device: Device, allocator: Option<AllocatorHelper>) -> Self {
        DescriptorPool(Arc::new(Inner {
            handle: handle,
            device: device,
            allocator: allocator,
        }))
    }

    #[inline]
    pub(crate) fn handle(&self) -> vks::VkDescriptorPool {
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

    /// See [`vkAllocateDescriptorSets`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkAllocateDescriptorSets)
    pub fn allocate_descriptor_sets(allocate_info: &core::DescriptorSetAllocateInfo) -> Result<Vec<DescriptorSet>, core::Error> {
        let descriptor_pool = &allocate_info.descriptor_pool;
        let allocate_info_wrapper: core::VkDescriptorSetAllocateInfoWrapper = allocate_info.into();

        let mut descriptor_sets = Vec::with_capacity(allocate_info.set_layouts.len());
        let res = unsafe {
            descriptor_sets.set_len(allocate_info.set_layouts.len());
            (descriptor_pool.loader().core.vkAllocateDescriptorSets)(descriptor_pool.device_handle(), &allocate_info_wrapper.vks_struct, descriptor_sets.as_mut_ptr())
        };

        if res == vks::VK_SUCCESS {
            Ok(descriptor_sets.iter().map(|s| DescriptorSet::new(*s, descriptor_pool.clone())).collect())
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkFreeDescriptorSets`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkFreeDescriptorSets)
    pub fn free_descriptor_sets(&self, descriptor_sets: &[DescriptorSet]) -> Result<(), core::Error> {
        let descriptor_sets: Vec<_> = descriptor_sets.iter().map(DescriptorSet::handle).collect();

        let res = unsafe {
            (self.loader().core.vkFreeDescriptorSets)(self.device_handle(), self.handle(), descriptor_sets.len() as u32, descriptor_sets.as_ptr())
        };

        if res == vks::VK_SUCCESS {
            Ok(())
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkResetDescriptorPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkResetDescriptorPool)
    pub fn reset(&self, flags: core::DescriptorPoolResetFlags) -> Result<(), core::Error> {
        let res = unsafe {
            (self.loader().core.vkResetDescriptorPool)(self.device_handle(), self.handle(), flags)
        };

        if res == vks::VK_SUCCESS {
            Ok(())
        }
        else {
            Err(res.into())
        }
    }
}

#[derive(Debug)]
struct Inner {
    handle: vks::VkDescriptorPool,
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
            (self.device.loader().core.vkDestroyDescriptorPool)(self.device.handle(), self.handle, allocator);
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
