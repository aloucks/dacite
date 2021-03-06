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
use core::{self, Instance};
use ext_debug_report;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ptr;
use std::sync::Arc;
use vks;

/// See [`VkDebugReportCallbackEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportCallbackEXT)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DebugReportCallbackExt(Arc<Inner>);

impl VulkanObject for DebugReportCallbackExt {
    type NativeVulkanObject = vks::ext_debug_report::VkDebugReportCallbackEXT;

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

pub struct FromNativeDebugReportCallbackExtParameters {
    /// `true`, if this `DebugReportCallbackExt` should destroy the underlying Vulkan object, when it is dropped.
    pub owned: bool,

    /// The `Instance`, from which this `DebugReportCallbackExt` was created.
    pub instance: Instance,

    /// An `Allocator` compatible with the one used to create this `DebugReportCallbackExt`.
    ///
    /// This parameter is ignored, if `owned` is `false`.
    pub allocator: Option<Box<core::Allocator>>,
}

impl FromNativeDebugReportCallbackExtParameters {
    #[inline]
    pub fn new(owned: bool, instance: Instance, allocator: Option<Box<core::Allocator>>) -> Self {
        FromNativeDebugReportCallbackExtParameters {
            owned: owned,
            instance: instance,
            allocator: allocator,
        }
    }
}

impl FromNativeObject for DebugReportCallbackExt {
    type Parameters = FromNativeDebugReportCallbackExtParameters;

    unsafe fn from_native_object(object: Self::NativeVulkanObject, params: Self::Parameters) -> Self {
        DebugReportCallbackExt::new(object, params.owned, params.instance, params.allocator.map(AllocatorHelper::new), None)
    }
}

impl DebugReportCallbackExt {
    pub(crate) fn new(handle: vks::ext_debug_report::VkDebugReportCallbackEXT, owned: bool, instance: Instance, allocator: Option<AllocatorHelper>, callback_helper: Option<ext_debug_report::CallbackHelper>) -> Self {
        DebugReportCallbackExt(Arc::new(Inner {
            handle: handle,
            owned: owned,
            instance: instance,
            allocator: allocator,
            callback_helper: callback_helper,
        }))
    }

    #[inline]
    pub(crate) fn handle(&self) -> vks::ext_debug_report::VkDebugReportCallbackEXT {
        self.0.handle
    }
}

#[derive(Debug)]
struct Inner {
    handle: vks::ext_debug_report::VkDebugReportCallbackEXT,
    owned: bool,
    instance: Instance,
    allocator: Option<AllocatorHelper>,
    callback_helper: Option<ext_debug_report::CallbackHelper>,
}

impl Drop for Inner {
    fn drop(&mut self) {
        if self.owned {
            let allocator = match self.allocator {
                Some(ref allocator) => allocator.callbacks(),
                None => ptr::null(),
            };

            unsafe {
                self.instance.loader().ext_debug_report.vkDestroyDebugReportCallbackEXT(self.instance.handle(), self.handle, allocator);
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
