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

    /// Get the object id.
    ///
    /// This function returns the same value as `as_native_vulkan_object()`, but conveniently cast
    /// to a `u64`. This is useful for extensions like `VK_EXT_debug_report` and
    /// `VK_EXT_debug_marker`, which use these to refer Vulkan objects.
    ///
    /// Object ids (or more correctly, handles) come in two variants, *dispatchable* and
    /// *non-dispatchable*. While dispatchable objects are actually pointers, and thus unique,
    /// the same is not true for non-dispatchable objects. For instance, two `Semaphore`s, created
    /// independently of each other, might in fact have the same handle.
    ///
    /// Additionally, handles of non-dispatchable objects are only ever meaningful, if their type
    /// is known (whether it is i.e. a `Semaphore` or some other type). This must be taken into
    /// account, if handles are used to identify Vulkan objects.
    ///
    /// Refer to the Vulkan specification ([Object Model]) for more information.
    ///
    /// [Object Model]:
    /// https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#fundamentals-objectmodel-overview
    fn id(&self) -> u64;

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

/// Trait for creating dacite Vulkan objects from native FFI objects.
///
/// This trait is provided for interoperability with other Vulkan-related libraries, if you need to
/// create dacite Vulkan objects from existing native FFI objects.
///
/// Not all dacite Vulkan objects implement this trait, which is why this is a separate trait
/// instead of being integrated in the `VulkanObject` trait.
///
/// __Caution__: Many implementors can optionally own the underlying native object (specified
/// through an `owned` parameter). This means, that the Vulkan object will be destroyed, when this
/// object is dropped. You must not create multiple dacite objects, which own the same Vulkan
/// object.
pub trait FromNativeObject: VulkanObject {
    type Parameters;

    unsafe fn from_native_object(object: Self::NativeVulkanObject, params: Self::Parameters) -> Self;
}
