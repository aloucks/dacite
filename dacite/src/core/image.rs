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
use core::{self, Device, DeviceMemory};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::mem;
use std::ptr;
use std::sync::Arc;
use vks;

/// See [`VkImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImage)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Image(Arc<Inner>);

impl VulkanObject for Image {
    type NativeVulkanObject = vks::core::VkImage;

    #[inline]
    fn id(&self) -> u64 {
        self.handle()
    }

    #[inline]
    fn as_native_vulkan_object(&self) -> Self::NativeVulkanObject {
        self.handle()
    }

    fn try_destroy(self) -> Result<(), TryDestroyError<Self>> {
        if self.0.owned {
            let strong_count = Arc::strong_count(&self.0);
            if strong_count == 1 {
                Ok(())
            }
            else {
                Err(TryDestroyError::new(self, TryDestroyErrorKind::InUse(Some(strong_count))))
            }
        }
        else {
            Err(TryDestroyError::new(self, TryDestroyErrorKind::Unsupported))
        }
    }
}

pub struct FromNativeImageParameters {
    /// `true`, if this `Image` should destroy the underlying Vulkan object, when it is dropped.
    pub owned: bool,

    /// The `Device`, from which this `Image` was created.
    pub device: Device,

    /// An `Allocator` compatible with the one used to create this `Image`.
    ///
    /// This parameter is ignored, if `owned` is `false`.
    pub allocator: Option<Box<core::Allocator>>,
}

impl FromNativeImageParameters {
    #[inline]
    pub fn new(owned: bool, device: Device, allocator: Option<Box<core::Allocator>>) -> Self {
        FromNativeImageParameters {
            owned: owned,
            device: device,
            allocator: allocator,
        }
    }
}

impl FromNativeObject for Image {
    type Parameters = FromNativeImageParameters;

    unsafe fn from_native_object(object: Self::NativeVulkanObject, params: Self::Parameters) -> Self {
        Image::new(object, params.owned, params.device, params.allocator.map(AllocatorHelper::new))
    }
}

impl Image {
    pub(crate) fn new(handle: vks::core::VkImage, owned: bool, device: Device, allocator: Option<AllocatorHelper>) -> Self {
        Image(Arc::new(Inner {
            handle: handle,
            owned: owned,
            device: device,
            allocator: allocator,
        }))
    }

    #[inline]
    pub(crate) fn handle(&self) -> vks::core::VkImage {
        self.0.handle
    }

    #[inline]
    pub(crate) fn loader(&self) -> &vks::DeviceProcAddrLoader {
        self.0.device.loader()
    }

    #[inline]
    pub(crate) fn device_handle(&self) -> vks::core::VkDevice {
        self.0.device.handle()
    }

    /// See [`vkBindImageMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkBindImageMemory)
    pub fn bind_memory(&self, memory: DeviceMemory, offset: u64) -> Result<(), core::Error> {
        let res = unsafe {
            (self.loader().core.vkBindImageMemory)(self.device_handle(), self.handle(), memory.handle(), offset)
        };

        if res == vks::core::VK_SUCCESS {
            Ok(())
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkGetImageMemoryRequirements`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetImageMemoryRequirements)
    pub fn get_memory_requirements(&self) -> core::MemoryRequirements {
        unsafe {
            let mut requirements = mem::uninitialized();
            (self.loader().core.vkGetImageMemoryRequirements)(self.device_handle(), self.handle(), &mut requirements);
            (&requirements).into()
        }
    }

    /// See [`vkGetImageSparseMemoryRequirements`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetImageSparseMemoryRequirements)
    pub fn get_sparse_memory_requirements<B>(&self) -> B
        where B: FromIterator<core::SparseImageMemoryRequirements>
    {
        let mut num_requirements = 0;
        unsafe {
            (self.loader().core.vkGetImageSparseMemoryRequirements)(self.device_handle(), self.handle(), &mut num_requirements, ptr::null_mut());
        }

        let mut requirements = Vec::with_capacity(num_requirements as usize);
        unsafe {
            requirements.set_len(num_requirements as usize);
            (self.loader().core.vkGetImageSparseMemoryRequirements)(self.device_handle(), self.handle(), &mut num_requirements, requirements.as_mut_ptr());
        }

        requirements.iter().map(From::from).collect()
    }

    /// See [`vkGetImageSubresourceLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetImageSubresourceLayout)
    pub fn get_subresource_layout(&self, subresource: &core::ImageSubresource) -> core::SubresourceLayout {
        let subresource = subresource.into();

        unsafe {
            let mut layout = mem::uninitialized();
            (self.loader().core.vkGetImageSubresourceLayout)(self.device_handle(), self.handle(), &subresource, &mut layout);
            (&layout).into()
        }
    }
}

#[derive(Debug)]
struct Inner {
    handle: vks::core::VkImage,
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
                (self.device.loader().core.vkDestroyImage)(self.device.handle(), self.handle, allocator);
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
