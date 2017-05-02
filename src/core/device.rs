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

use core::Instance;
use core::allocator_helper::AllocatorHelper;
use std::ptr;
use std::sync::Arc;
use vk_sys;

#[derive(Debug)]
pub(crate) struct Inner {
    pub handle: vk_sys::VkDevice,
    pub instance: Instance,
    pub allocator: Option<AllocatorHelper>,
    pub loader: vk_sys::DeviceProcAddrLoader,
}

impl Drop for Inner {
    fn drop(&mut self) {
        let allocator = match self.allocator {
            Some(ref allocator) => &allocator.callbacks,
            None => ptr::null(),
        };

        unsafe {
            (self.loader.core.vkDestroyDevice)(self.handle, allocator);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Device(pub(crate) Arc<Inner>);

impl Device {
    pub fn new(handle: vk_sys::VkDevice, instance: Instance, allocator: Option<AllocatorHelper>) -> Self {
        let mut loader = vk_sys::DeviceProcAddrLoader::new(instance.0.loader.core.vkGetDeviceProcAddr);
        unsafe {
            loader.load_core(handle);
        }

        Device(Arc::new(Inner {
            handle: handle,
            instance: instance,
            allocator: allocator,
            loader: loader,
        }))
    }
}
