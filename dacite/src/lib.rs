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

#[macro_use]
mod chain;

mod utils;
mod vulkan_object;

pub mod core;
pub mod ext_debug_report;
pub mod khr_android_surface;
pub mod khr_display;
pub mod khr_display_swapchain;
pub mod khr_get_physical_device_properties2;
pub mod khr_mir_surface;
pub mod khr_surface;
pub mod khr_swapchain;
pub mod khr_wayland_surface;
pub mod khr_win32_surface;
pub mod khr_xcb_surface;
pub mod khr_xlib_surface;

pub use vks::android_wrapper;
pub use vks::mir_wrapper;
pub use vks::wayland_wrapper;
pub use vks::win32_wrapper;
pub use vks::xcb_wrapper;
pub use vks::xlib_wrapper;

pub use vulkan_object::{
    TryDestroyError,
    TryDestroyErrorKind,
    VulkanObject,
};

pub const DACITE_API_VERSION: core::Version = core::Version {
    major: DACITE_API_VERSION_MAJOR,
    minor: DACITE_API_VERSION_MINOR,
    patch: DACITE_API_VERSION_PATCH,
};

pub const DACITE_API_VERSION_MAJOR: u32 = 1;
pub const DACITE_API_VERSION_MINOR: u32 = 0;
pub const DACITE_API_VERSION_PATCH: u32 = 6;
