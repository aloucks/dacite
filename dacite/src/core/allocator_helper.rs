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

use core;
use libc::c_void;
use std::sync::Arc;
use vks;

unsafe extern "system" fn alloc(user_data: *mut c_void, size: usize, alignment: usize, allocation_scope: vks::vk::VkSystemAllocationScope) -> *mut c_void {
    let allocator = user_data as *const Box<core::Allocator>;
    (*allocator).alloc(size, alignment, allocation_scope.into())
}

unsafe extern "system" fn realloc(user_data: *mut c_void, original: *mut c_void, size: usize, alignment: usize, allocation_scope: vks::vk::VkSystemAllocationScope) -> *mut c_void {
    let allocator = user_data as *const Box<core::Allocator>;
    (*allocator).realloc(original, size, alignment, allocation_scope.into())
}

unsafe extern "system" fn free(user_data: *mut c_void, memory: *mut c_void) {
    let allocator = user_data as *const Box<core::Allocator>;
    (*allocator).free(memory);
}

unsafe extern "system" fn internal_alloc(user_data: *mut c_void, size: usize, allocation_type: vks::vk::VkInternalAllocationType, allocation_scope: vks::vk::VkSystemAllocationScope) {
    let allocator = user_data as *const Box<core::Allocator>;
    (*allocator).internal_alloc(size, allocation_type.into(), allocation_scope.into())
}

unsafe extern "system" fn internal_free(user_data: *mut c_void, size: usize, allocation_type: vks::vk::VkInternalAllocationType, allocation_scope: vks::vk::VkSystemAllocationScope) {
    let allocator = user_data as *const Box<core::Allocator>;
    (*allocator).internal_free(size, allocation_type.into(), allocation_scope.into())
}

#[derive(Debug, Clone)]
pub struct AllocatorHelper {
    callbacks: vks::vk::VkAllocationCallbacks,
    allocator: Arc<Box<core::Allocator>>,
}

impl AllocatorHelper {
    pub fn new(allocator: Box<core::Allocator>) -> Self {
        let has_internal_alloc = allocator.has_internal_alloc();
        let allocator = Arc::new(allocator);
        let allocator_ptr = Arc::into_raw(allocator);
        let allocator = unsafe { Arc::from_raw(allocator_ptr) };

        let mut allocation_callbacks = vks::vk::VkAllocationCallbacks {
            pUserData: allocator_ptr as *mut c_void,
            pfnAllocation: Some(alloc),
            pfnReallocation: Some(realloc),
            pfnFree: Some(free),
            pfnInternalAllocation: None,
            pfnInternalFree: None,
        };

        if has_internal_alloc {
            allocation_callbacks.pfnInternalAllocation = Some(internal_alloc);
            allocation_callbacks.pfnInternalFree = Some(internal_free);
        }

        AllocatorHelper {
            callbacks: allocation_callbacks,
            allocator: allocator,
        }
    }

    #[inline]
    pub fn callbacks(&self) -> *const vks::vk::VkAllocationCallbacks {
        &self.callbacks
    }
}
