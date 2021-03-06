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

//! See extension [`VK_KHR_xcb_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_xcb_surface)

use vks;
use xcb_types;

dacite_bitflags! {
    /// See [`VkXcbSurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkXcbSurfaceCreateFlagsKHR)
    pub struct XcbSurfaceCreateFlagsKhr: vks::khr_xcb_surface::VkXcbSurfaceCreateFlagsKHR;
    pub enum XcbSurfaceCreateFlagBitsKhr: vks::khr_xcb_surface::VkXcbSurfaceCreateFlagBitsKHR;
    max_enum: vks::khr_xcb_surface::VK_XCB_SURFACE_CREATE_FLAG_BITS_MAX_ENUM_KHR;

    flags {}
    no_bits {}
}

gen_chain_struct! {
    name: XcbSurfaceCreateInfoChainKhr [XcbSurfaceCreateInfoChainKhrWrapper],
    query: XcbSurfaceCreateInfoChainQueryKhr [XcbSurfaceCreateInfoChainQueryKhrWrapper],
    vks: vks::khr_xcb_surface::VkXcbSurfaceCreateInfoKHR,
    input: true,
    output: false,
}

/// See [`VkXcbSurfaceCreateInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkXcbSurfaceCreateInfoKHR)
#[derive(Debug, Clone, PartialEq)]
pub struct XcbSurfaceCreateInfoKhr {
    pub flags: XcbSurfaceCreateFlagsKhr,
    pub connection: *mut xcb_types::xcb_connection_t,
    pub window: xcb_types::xcb_window_t,
    pub chain: Option<XcbSurfaceCreateInfoChainKhr>,
}

#[derive(Debug)]
pub(crate) struct VkXcbSurfaceCreateInfoKHRWrapper {
    pub vks_struct: vks::khr_xcb_surface::VkXcbSurfaceCreateInfoKHR,
    chain: Option<XcbSurfaceCreateInfoChainKhrWrapper>,
}

impl VkXcbSurfaceCreateInfoKHRWrapper {
    pub fn new(create_info: &XcbSurfaceCreateInfoKhr, with_chain: bool) -> Self {
        let (pnext, chain) = XcbSurfaceCreateInfoChainKhrWrapper::new_optional(&create_info.chain, with_chain);

        VkXcbSurfaceCreateInfoKHRWrapper {
            vks_struct: vks::khr_xcb_surface::VkXcbSurfaceCreateInfoKHR {
                sType: vks::vk::VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR,
                pNext: pnext,
                flags: create_info.flags.bits(),
                connection: create_info.connection,
                window: create_info.window,
            },
            chain: chain,
        }
    }
}
