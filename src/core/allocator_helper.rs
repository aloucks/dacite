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
use std::mem;
use vks;

unsafe extern "system" fn alloc(user_data: *mut c_void, size: usize, alignment: usize, allocation_scope: vks::VkSystemAllocationScope) -> *mut c_void {
    let allocator = &mut **(user_data as *mut *mut core::Allocator);
    allocator.alloc(size, alignment, allocation_scope.into())
}

unsafe extern "system" fn realloc(user_data: *mut c_void, original: *mut c_void, size: usize, alignment: usize, allocation_scope: vks::VkSystemAllocationScope) -> *mut c_void {
    let allocator = &mut **(user_data as *mut *mut core::Allocator);
    allocator.realloc(original, size, alignment, allocation_scope.into())
}

unsafe extern "system" fn free(user_data: *mut c_void, memory: *mut c_void) {
    let allocator = &mut **(user_data as *mut *mut core::Allocator);
    allocator.free(memory);
}

unsafe extern "system" fn internal_alloc(user_data: *mut c_void, size: usize, allocation_type: vks::VkInternalAllocationType, allocation_scope: vks::VkSystemAllocationScope) {
    let allocator = &mut **(user_data as *mut *mut core::Allocator);
    allocator.internal_alloc(size, allocation_type.into(), allocation_scope.into())
}

unsafe extern "system" fn internal_free(user_data: *mut c_void, size: usize, allocation_type: vks::VkInternalAllocationType, allocation_scope: vks::VkSystemAllocationScope) {
    let allocator = &mut **(user_data as *mut *mut core::Allocator);
    allocator.internal_free(size, allocation_type.into(), allocation_scope.into())
}

#[derive(Debug)]
pub struct AllocatorHelper {
    pub callbacks: vks::VkAllocationCallbacks,
    allocator: *mut *mut core::Allocator,
}

impl Drop for AllocatorHelper {
    fn drop(&mut self) {
        unsafe {
            Box::from_raw(*Box::from_raw(self.allocator));
        }
    }
}

impl AllocatorHelper {
    pub fn new(allocator: Box<core::Allocator>) -> Self {
        let has_internal_alloc = allocator.has_internal_alloc();
        let allocator = Box::into_raw(Box::new(Box::into_raw(allocator)));

        let mut allocation_callbacks = vks::VkAllocationCallbacks {
            pUserData: allocator as *mut c_void,
            pfnAllocation: alloc,
            pfnReallocation: realloc,
            pfnFree: free,
            pfnInternalAllocation: unsafe { mem::transmute(0usize) },
            pfnInternalFree: unsafe { mem::transmute(0usize) },
        };

        if has_internal_alloc {
            allocation_callbacks.pfnInternalAllocation = internal_alloc;
            allocation_callbacks.pfnInternalFree = internal_free;
        }

        AllocatorHelper {
            callbacks: allocation_callbacks,
            allocator: allocator,
        }
    }
}
