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
use core::allocator_helper::AllocatorHelper;
use core::{self, Device};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ptr;
use std::sync::Arc;
use vks;

/// See [`VkSampler`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSampler)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sampler(Arc<Inner>);

impl VulkanObject for Sampler {
    type NativeVulkanObject = vks::vk::VkSampler;

    #[inline]
    fn id(&self) -> u64 {
        self.handle()
    }

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

pub struct FromNativeSamplerParameters {
    /// `true`, if this `Sampler` should destroy the underlying Vulkan object, when it is dropped.
    pub owned: bool,

    /// The `Device`, from which this `Sampler` was created.
    pub device: Device,

    /// An `Allocator` compatible with the one used to create this `Sampler`.
    ///
    /// This parameter is ignored, if `owned` is `false`.
    pub allocator: Option<Box<core::Allocator>>,
}

impl FromNativeSamplerParameters {
    #[inline]
    pub fn new(owned: bool, device: Device, allocator: Option<Box<core::Allocator>>) -> Self {
        FromNativeSamplerParameters {
            owned: owned,
            device: device,
            allocator: allocator,
        }
    }
}

impl FromNativeObject for Sampler {
    type Parameters = FromNativeSamplerParameters;

    unsafe fn from_native_object(object: Self::NativeVulkanObject, params: Self::Parameters) -> Self {
        Sampler::new(object, params.owned, params.device, params.allocator.map(AllocatorHelper::new))
    }
}

impl Sampler {
    pub(crate) fn new(handle: vks::vk::VkSampler, owned: bool, device: Device, allocator: Option<AllocatorHelper>) -> Self {
        Sampler(Arc::new(Inner {
            handle: handle,
            owned: owned,
            device: device,
            allocator: allocator,
        }))
    }

    #[inline]
    pub(crate) fn handle(&self) -> vks::vk::VkSampler {
        self.0.handle
    }
}

#[derive(Debug)]
struct Inner {
    handle: vks::vk::VkSampler,
    owned: bool,
    device: Device,
    allocator: Option<AllocatorHelper>,
}

impl Drop for Inner {
    fn drop(&mut self) {
        if self.owned {
            let allocator = match self.allocator {
                Some(ref allocator) => allocator.callbacks(),
                None => ptr::null(),
            };

            unsafe {
                self.device.loader().vk.vkDestroySampler(self.device.handle(), self.handle, allocator);
            }
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
