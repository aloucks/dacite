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
use core::{self, CommandBuffer, Device};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ptr;
use std::sync::Arc;
use vks;
use {TryDestroyError, TryDestroyErrorKind, VulkanObject};

/// See [`VkCommandPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandPool)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandPool(Arc<Inner>);

impl VulkanObject for CommandPool {
    type NativeVulkanObject = vks::VkCommandPool;

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

impl CommandPool {
    pub(crate) fn new(handle: vks::VkCommandPool, device: Device, allocator: Option<AllocatorHelper>) -> Self {
        CommandPool(Arc::new(Inner {
            handle: handle,
            device: device,
            allocator: allocator,
        }))
    }

    #[inline]
    pub(crate) fn handle(&self) -> vks::VkCommandPool {
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

    /// See [`vkResetCommandPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkResetCommandPool)
    pub fn reset(&self, flags: core::CommandPoolResetFlags) -> Result<(), core::Error> {
        let res = unsafe {
            (self.loader().core.vkResetCommandPool)(self.device_handle(), self.handle(), flags)
        };

        if res == vks::VK_SUCCESS {
            Ok(())
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkAllocateCommandBuffers`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkAllocateCommandBuffers)
    pub fn allocate_command_buffers(&self, allocate_info: &core::CommandBufferAllocateInfo) -> Result<Vec<CommandBuffer>, core::Error> {
        let mut command_buffers = Vec::with_capacity(allocate_info.command_buffer_count as usize);
        unsafe {
            command_buffers.set_len(allocate_info.command_buffer_count as usize);
        }

        let mut allocate_info: core::VkCommandBufferAllocateInfoWrapper = allocate_info.into();
        allocate_info.set_command_pool(Some(self.clone()));

        let res = unsafe {
            (self.loader().core.vkAllocateCommandBuffers)(self.device_handle(), allocate_info.as_ref(), command_buffers.as_mut_ptr())
        };

        if res == vks::VK_SUCCESS {
            Ok(command_buffers.iter().map(|&c| CommandBuffer::new(c, self.clone())).collect())
        }
        else {
            Err(res.into())
        }
    }
}

#[derive(Debug)]
struct Inner {
    handle: vks::VkCommandPool,
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
            (self.device.loader().core.vkDestroyCommandPool)(self.device.handle(), self.handle, allocator);
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
