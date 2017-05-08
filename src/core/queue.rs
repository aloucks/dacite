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
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use vks;
use {TryDestroyError, TryDestroyErrorKind, VulkanObject};

/// See [`VkQueue`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueue)
#[derive(Debug, Clone)]
pub struct Queue {
    handle: vks::VkQueue,
    device: Device,
}

impl PartialEq for Queue {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}

impl Eq for Queue { }

impl PartialOrd for Queue {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.handle.partial_cmp(&other.handle)
    }
}

impl Ord for Queue {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.handle.cmp(&other.handle)
    }
}

impl Hash for Queue {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.handle.hash(state);
    }
}

impl VulkanObject for Queue {
    type NativeVulkanObject = vks::VkQueue;

    #[inline]
    fn as_native_vulkan_object(&self) -> Self::NativeVulkanObject {
        self.handle
    }

    fn try_destroy(self) -> Result<(), TryDestroyError<Self>> {
        Err(TryDestroyError::new(self, TryDestroyErrorKind::Unsupported))
    }
}

impl Queue {
    pub(crate) fn new(handle: vks::VkQueue, device: Device) -> Self {
        Queue {
            handle: handle,
            device: device,
        }
    }
}
