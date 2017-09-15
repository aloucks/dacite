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

//! See extension [`VK_KHR_xlib_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_xlib_surface)

use vks;
use xlib_types;

dacite_bitflags! {
    /// See [`VkXlibSurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkXlibSurfaceCreateFlagsKHR)
    pub struct XlibSurfaceCreateFlagsKhr: vks::khr_xlib_surface::VkXlibSurfaceCreateFlagsKHR;
    pub enum XlibSurfaceCreateFlagBitsKhr: vks::khr_xlib_surface::VkXlibSurfaceCreateFlagBitsKHR;
    max_enum: vks::khr_xlib_surface::VK_XLIB_SURFACE_CREATE_FLAG_BITS_MAX_ENUM_KHR;

    flags {}
    no_bits {}
}

gen_chain_struct! {
    name: XlibSurfaceCreateInfoChainKhr [XlibSurfaceCreateInfoChainKhrWrapper],
    query: XlibSurfaceCreateInfoChainQueryKhr [XlibSurfaceCreateInfoChainQueryKhrWrapper],
    vks: vks::khr_xlib_surface::VkXlibSurfaceCreateInfoKHR,
    input: true,
    output: false,
}

/// See [`VkXlibSurfaceCreateInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkXlibSurfaceCreateInfoKHR)
#[derive(Debug, Clone, PartialEq)]
pub struct XlibSurfaceCreateInfoKhr {
    pub flags: XlibSurfaceCreateFlagsKhr,
    pub dpy: *mut xlib_types::Display,
    pub window: xlib_types::Window,
    pub chain: Option<XlibSurfaceCreateInfoChainKhr>,
}

#[derive(Debug)]
pub(crate) struct VkXlibSurfaceCreateInfoKHRWrapper {
    pub vks_struct: vks::khr_xlib_surface::VkXlibSurfaceCreateInfoKHR,
    chain: Option<XlibSurfaceCreateInfoChainKhrWrapper>,
}

impl VkXlibSurfaceCreateInfoKHRWrapper {
    pub fn new(create_info: &XlibSurfaceCreateInfoKhr, with_chain: bool) -> Self {
        let (pnext, chain) = XlibSurfaceCreateInfoChainKhrWrapper::new_optional(&create_info.chain, with_chain);

        VkXlibSurfaceCreateInfoKHRWrapper {
            vks_struct: vks::khr_xlib_surface::VkXlibSurfaceCreateInfoKHR {
                sType: vks::core::VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR,
                pNext: pnext,
                flags: create_info.flags.bits(),
                dpy: create_info.dpy,
                window: create_info.window,
            },
            chain: chain,
        }
    }
}
