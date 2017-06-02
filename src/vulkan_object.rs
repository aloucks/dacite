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

use std::error::Error;
use std::fmt;

use core;

pub trait VulkanObject: Sized + Send + Sync + Clone + fmt::Debug {
    type NativeVulkanObject;

    fn as_native_vulkan_object(&self) -> Self::NativeVulkanObject;
    fn try_destroy(self) -> Result<(), TryDestroyError<Self>>;
}

pub struct TryDestroyError<T: VulkanObject> {
    object: T,
    kind: TryDestroyErrorKind,
}

impl<T: VulkanObject> fmt::Debug for TryDestroyError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TryDestroyError")
            .field("object", &"VulkanObject")
            .field("kind", &self.kind)
            .finish()
    }
}

impl<T: VulkanObject> fmt::Display for TryDestroyError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            TryDestroyErrorKind::InUse(Some(strong_count)) => write!(f, "This Vulkan object still has a reference counter of {} and can not be destroyed", strong_count),
            _ => write!(f, "{}", self.description()),
        }
    }
}

impl<T: VulkanObject> Error for TryDestroyError<T> {
    fn description(&self) -> &str {
        match self.kind {
            TryDestroyErrorKind::Unsupported => "This Vulkan object can not be destroyed explicitly",
            TryDestroyErrorKind::InUse(_) => "This Vulkan object is referenced elsewhere",
            TryDestroyErrorKind::VulkanError(_) => "A Vulkan error occurred while trying to destroy the object",
        }
    }
}

impl<T: VulkanObject> TryDestroyError<T> {
    pub fn new(object: T, kind: TryDestroyErrorKind) -> Self {
        TryDestroyError {
            object: object,
            kind: kind,
        }
    }

    /// Consumes this error and returns the Vulkan object in its original state.
    pub fn into_vulkan_object(self) -> T {
        self.object
    }

    pub fn kind(&self) -> TryDestroyErrorKind {
        self.kind
    }
}

/// Indicates the kind of error, which occurred while trying to delete a Vulkan object.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TryDestroyErrorKind {
    /// The Vulkan object can not be destroyed explicitly.
    ///
    /// This is the case for objects, which are implicitly destroyed with its parent
    /// (e.g. PhysicalDevice and Queue).
    Unsupported,

    /// The Vulkan object is still in use.
    ///
    /// The optional usize value indicates the current reference counter.
    InUse(Option<usize>),

    /// A Vulkan error occurred at runtime.
    VulkanError(core::Error),
}
