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
extern crate vks;

pub trait AsNativeVkObject {
    type NativeVkObject;

    fn as_native_vk_object(&self) -> Self::NativeVkObject;
}

#[cfg(feature = "core_1_0_3")]
mod utils;

#[cfg(feature = "core_1_0_3")]
pub mod core;

#[cfg(feature = "core_1_0_3")]
mod version;

#[cfg(feature = "core_1_0_3")]
pub use version::{DACITE_API_VERSION_MAJOR, DACITE_API_VERSION_MINOR, DACITE_API_VERSION_PATCH};

#[cfg(feature = "core_1_0_3")]
pub type Result<T> = ::std::result::Result<T, core::Error>;
