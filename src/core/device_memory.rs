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
use core::{self, Device};
use libc::c_void;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ptr;
use std::sync::Arc;
use vks;
use {TryDestroyError, TryDestroyErrorKind, VulkanObject};

/// See [`VkDeviceMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceMemory)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceMemory(Arc<Inner>);

impl VulkanObject for DeviceMemory {
    type NativeVulkanObject = vks::VkDeviceMemory;

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

impl DeviceMemory {
    pub(crate) fn new(handle: vks::VkDeviceMemory, device: Device, allocator: Option<AllocatorHelper>, size: u64) -> Self {
        DeviceMemory(Arc::new(Inner {
            handle: handle,
            device: device,
            allocator: allocator,
            size: size,
        }))
    }

    #[inline]
    pub(crate) fn handle(&self) -> vks::VkDeviceMemory {
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

    pub fn size(&self) -> u64 {
        self.0.size
    }

    /// See [`vkGetDeviceMemoryCommitment`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDeviceMemoryCommitment)
    pub fn get_commitment(&self) -> u64 {
        let mut commitment = 0;
        unsafe {
            (self.loader().core.vkGetDeviceMemoryCommitment)(self.device_handle(), self.handle(), &mut commitment)
        };

        commitment
    }

    /// See [`vkMapMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkMapMemory)
    pub fn map(&self, offset: u64, size: core::OptionalDeviceSize, flags: core::MemoryMapFlags) -> Result<MappedMemory, core::Error> {
        let mut mapped = ptr::null_mut();
        let res = unsafe {
            (self.loader().core.vkMapMemory)(self.device_handle(), self.handle(), offset, size.into(), flags, &mut mapped)
        };

        if res == vks::VK_SUCCESS {
            let size = match size {
                core::OptionalDeviceSize::Size(size) => size,
                core::OptionalDeviceSize::WholeSize => self.0.size - offset,
            };

            Ok(MappedMemory {
                memory: self.clone(),
                mapped: mapped,
                offset: offset,
                size: size,
            })
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkFlushMappedMemoryRanges`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkFlushMappedMemoryRanges)
    pub fn flush(ranges: &[core::MappedMemoryRange]) -> Result<(), core::Error> {
        let loader = ranges[0].memory.loader();
        let device_handle = ranges[0].memory.device_handle();

        let ranges_wrappers: Vec<_> = ranges.iter().map(|r| core::VkMappedMemoryRangeWrapper::new(r, true)).collect();
        let ranges: Vec<_> = ranges_wrappers.iter().map(|r| r.vks_struct).collect();

        let res = unsafe {
            (loader.core.vkFlushMappedMemoryRanges)(device_handle, ranges.len() as u32, ranges.as_ptr())
        };

        if res == vks::VK_SUCCESS {
            Ok(())
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkInvalidateMappedMemoryRanges`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkInvalidateMappedMemoryRanges)
    pub fn invalidate(ranges: &[core::MappedMemoryRange]) -> Result<(), core::Error> {
        let loader = ranges[0].memory.loader();
        let device_handle = ranges[0].memory.device_handle();

        let ranges_wrappers: Vec<_> = ranges.iter().map(|r| core::VkMappedMemoryRangeWrapper::new(r, true)).collect();
        let ranges: Vec<_> = ranges_wrappers.iter().map(|r| r.vks_struct).collect();

        let res = unsafe {
            (loader.core.vkInvalidateMappedMemoryRanges)(device_handle, ranges.len() as u32, ranges.as_ptr())
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
pub struct MappedMemory {
    memory: DeviceMemory,
    mapped: *mut c_void,
    offset: u64,
    size: u64,
}

impl Drop for MappedMemory {
    fn drop(&mut self) {
        unsafe {
            (self.memory.loader().core.vkUnmapMemory)(self.memory.device_handle(), self.memory.handle());
        }
    }
}

impl MappedMemory {
    pub fn as_ptr(&self) -> *mut c_void {
        self.mapped
    }

    pub fn offset(&self) -> u64 {
        self.offset
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    /// See [`vkFlushMappedMemoryRanges`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkFlushMappedMemoryRanges)
    pub fn flush(&self, chain: &Option<core::MappedMemoryRangeChain>) -> Result<(), core::Error> {
        let (pnext, _) = core::MappedMemoryRangeChainWrapper::new_optional(chain, true);

        let range = vks::VkMappedMemoryRange {
            sType: vks::VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE,
            pNext: pnext,
            memory: self.memory.handle(),
            offset: self.offset,
            size: vks::VK_WHOLE_SIZE,
        };

        let res = unsafe {
            (self.memory.loader().core.vkFlushMappedMemoryRanges)(self.memory.device_handle(), 1, &range)
        };

        if res == vks::VK_SUCCESS {
            Ok(())
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkInvalidateMappedMemoryRanges`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkInvalidateMappedMemoryRanges)
    pub fn invalidate(&self, chain: &Option<core::MappedMemoryRangeChain>) -> Result<(), core::Error> {
        let (pnext, _) = core::MappedMemoryRangeChainWrapper::new_optional(chain, true);

        let range = vks::VkMappedMemoryRange {
            sType: vks::VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE,
            pNext: pnext,
            memory: self.memory.handle(),
            offset: self.offset,
            size: vks::VK_WHOLE_SIZE,
        };

        let res = unsafe {
            (self.memory.loader().core.vkInvalidateMappedMemoryRanges)(self.memory.device_handle(), 1, &range)
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
    handle: vks::VkDeviceMemory,
    device: Device,
    allocator: Option<AllocatorHelper>,
    size: u64,
}

impl Drop for Inner {
    fn drop(&mut self) {
        let allocator = match self.allocator {
            Some(ref allocator) => allocator.callbacks(),
            None => ptr::null(),
        };

        unsafe {
            (self.device.loader().core.vkFreeMemory)(self.device.handle(), self.handle, allocator);
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
