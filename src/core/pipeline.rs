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
use core::Device;
use core::allocator_helper::AllocatorHelper;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ptr;
use std::sync::Arc;
use vks;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pipeline(Arc<Inner>);

impl AsNativeVkObject for Pipeline {
    type NativeVkObject = vks::VkPipeline;

    #[inline]
    fn as_native_vk_object(&self) -> Self::NativeVkObject {
        self.handle()
    }
}

impl Pipeline {
    pub(crate) fn new(handle: vks::VkPipeline, device: Device, allocator: Option<AllocatorHelper>) -> Self {
        Pipeline(Arc::new(Inner {
            handle: handle,
            device: device,
            allocator: allocator,
        }))
    }

    #[inline]
    pub(crate) fn handle(&self) -> vks::VkPipeline {
        self.0.handle
    }
}

#[derive(Debug)]
struct Inner {
    handle: vks::VkPipeline,
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
            (self.device.loader().core.vkDestroyPipeline)(self.device.handle(), self.handle, allocator);
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
