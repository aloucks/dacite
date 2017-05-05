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

use core::Device;
use core::allocator_helper::AllocatorHelper;
use std::ptr;
use std::sync::Arc;
use vk_sys;

#[derive(Debug)]
struct Inner {
    handle: vk_sys::VkFramebuffer,
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
            (self.device.loader().core.vkDestroyFramebuffer)(self.device.handle(), self.handle, allocator);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Framebuffer(Arc<Inner>);

impl Framebuffer {
    pub(crate) fn new(handle: vk_sys::VkFramebuffer, device: Device, allocator: Option<AllocatorHelper>) -> Self {
        Framebuffer(Arc::new(Inner {
            handle: handle,
            device: device,
            allocator: allocator,
        }))
    }

    #[inline]
    pub(crate) fn handle(&self) -> vk_sys::VkFramebuffer {
        self.0.handle
    }
}
