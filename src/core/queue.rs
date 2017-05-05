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
use vk_sys;

#[derive(Debug, Clone)]
pub struct Queue {
    handle: vk_sys::VkQueue,
    device: Device,
}

impl AsNativeVkObject for Queue {
    type NativeVkObject = vk_sys::VkQueue;

    #[inline]
    fn as_native_vk_object(&self) -> Self::NativeVkObject {
        self.handle
    }
}

impl Queue {
    pub(crate) fn new(handle: vk_sys::VkQueue, device: Device) -> Self {
        Queue {
            handle: handle,
            device: device,
        }
    }
}
