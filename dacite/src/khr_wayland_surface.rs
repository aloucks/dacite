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

//! See extension [`VK_KHR_wayland_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_wayland_surface)

use vks;
use wayland_types;

dacite_bitflags! {
    /// See [`VkWaylandSurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkWaylandSurfaceCreateFlagsKHR)
    pub struct WaylandSurfaceCreateFlagsKhr: vks::khr_wayland_surface::VkWaylandSurfaceCreateFlagsKHR;
    pub enum WaylandSurfaceCreateFlagBitsKhr: vks::khr_wayland_surface::VkWaylandSurfaceCreateFlagBitsKHR;
    max_enum: vks::khr_wayland_surface::VK_WAYLAND_SURFACE_CREATE_FLAG_BITS_MAX_ENUM_KHR;

    flags {}
    no_bits {}
}

gen_chain_struct! {
    name: WaylandSurfaceCreateInfoChainKhr [WaylandSurfaceCreateInfoChainKhrWrapper],
    query: WaylandSurfaceCreateInfoChainQueryKhr [WaylandSurfaceCreateInfoChainQueryKhrWrapper],
    vks: vks::khr_wayland_surface::VkWaylandSurfaceCreateInfoKHR,
    input: true,
    output: false,
}

/// See [`VkWaylandSurfaceCreateInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkWaylandSurfaceCreateInfoKHR)
#[derive(Debug, Clone, PartialEq)]
pub struct WaylandSurfaceCreateInfoKhr {
    pub flags: WaylandSurfaceCreateFlagsKhr,
    pub display: *mut wayland_types::wl_display,
    pub surface: *mut wayland_types::wl_surface,
    pub chain: Option<WaylandSurfaceCreateInfoChainKhr>,
}

#[derive(Debug)]
pub(crate) struct VkWaylandSurfaceCreateInfoKHRWrapper {
    pub vks_struct: vks::khr_wayland_surface::VkWaylandSurfaceCreateInfoKHR,
    chain: Option<WaylandSurfaceCreateInfoChainKhrWrapper>,
}

impl VkWaylandSurfaceCreateInfoKHRWrapper {
    pub fn new(create_info: &WaylandSurfaceCreateInfoKhr, with_chain: bool) -> Self {
        let (pnext, chain) = WaylandSurfaceCreateInfoChainKhrWrapper::new_optional(&create_info.chain, with_chain);

        VkWaylandSurfaceCreateInfoKHRWrapper {
            vks_struct: vks::khr_wayland_surface::VkWaylandSurfaceCreateInfoKHR {
                sType: vks::core::VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR,
                pNext: pnext,
                flags: create_info.flags.bits(),
                display: create_info.display,
                surface: create_info.surface,
            },
            chain: chain,
        }
    }
}
