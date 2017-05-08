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

use AsNativeVkObject;
use core::allocator_helper::AllocatorHelper;
use core::{self, CommandBuffer, Device};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ptr;
use std::sync::Arc;
use vks;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandPool(Arc<Inner>);

impl AsNativeVkObject for CommandPool {
    type NativeVkObject = vks::VkCommandPool;

    #[inline]
    fn as_native_vk_object(&self) -> Self::NativeVkObject {
        self.handle()
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
            Some(ref allocator) => &allocator.callbacks,
            None => ptr::null(),
        };

        unsafe {
            (self.device.loader().core.vkDestroyCommandPool)(self.device.handle(), self.handle, allocator);
        }
    }
}

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
