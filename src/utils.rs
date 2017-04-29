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

use libc::c_char;
use std::ffi::{CStr, CString};
use std::ptr;
use vk_sys;

#[inline]
pub fn from_vk_bool(v: vk_sys::VkBool32) -> bool {
    v != vk_sys::VK_FALSE
}

#[inline]
pub fn to_vk_bool(v: bool) -> vk_sys::VkBool32 {
    if v {
        vk_sys::VK_TRUE
    }
    else {
        vk_sys::VK_FALSE
    }
}

#[inline]
pub fn string_from_cstr(cstr: *const c_char) -> Option<String> {
    if !cstr.is_null() {
        unsafe {
            Some(CStr::from_ptr(cstr).to_str().unwrap().to_owned())
        }
    }
    else {
        None
    }
}

#[inline]
pub fn cstr_from_string(string: Option<String>) -> (Option<CString>, *const c_char) {
    match string {
        Some(string) => {
            let cstr = CString::new(string).unwrap();
            let ptr = cstr.as_ptr();
            (Some(cstr), ptr)
        }

        None => (None, ptr::null()),
    }
}
