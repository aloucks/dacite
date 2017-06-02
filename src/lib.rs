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

extern crate libc;
extern crate libloading;
extern crate vks;

#[cfg(feature = "core_1_0_3")]
#[macro_use]
mod chain;

mod vulkan_object;
pub use vulkan_object::{
    TryDestroyError,
    TryDestroyErrorKind,
    VulkanObject,
};

#[cfg(feature = "core_1_0_3")]
mod utils;

#[cfg(feature = "core_1_0_3")]
pub mod core;

#[cfg(feature = "core_1_0_3")]
mod version;

#[cfg(feature = "core_1_0_3")]
pub use version::{
    DACITE_API_VERSION,
    DACITE_API_VERSION_MAJOR,
    DACITE_API_VERSION_MINOR,
    DACITE_API_VERSION_PATCH
};

#[cfg(feature = "khr_xlib_surface_6")]
pub use vks::xlib_wrapper;

#[cfg(feature = "khr_wayland_surface_5")]
pub use vks::wayland_wrapper;

#[cfg(feature = "khr_surface_25")]
pub mod khr_surface;

#[cfg(feature = "ext_debug_report_1")]
pub mod ext_debug_report;

#[cfg(feature = "khr_display_21")]
pub mod khr_display;

#[cfg(feature = "khr_swapchain_67")]
pub mod khr_swapchain;

#[cfg(feature = "khr_display_swapchain_9")]
pub mod khr_display_swapchain;

#[cfg(feature = "khr_xlib_surface_6")]
pub mod khr_xlib_surface;

#[cfg(feature = "khr_wayland_surface_5")]
pub mod khr_wayland_surface;
