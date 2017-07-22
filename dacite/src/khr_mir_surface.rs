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

//! See extension [`VK_KHR_mir_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_mir_surface)

use mir_types;
use vks;

bitflags! {
    /// See [`VkMirSurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMirSurfaceCreateFlagsKHR)
    #[derive(Default)]
    pub struct MirSurfaceCreateFlagsKhr: vks::khr_mir_surface::VkMirSurfaceCreateFlagsKHR {
        /// See [`VkMirSurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMirSurfaceCreateFlagsKHR)
        const MIR_SURFACE_CREATE_FLAG_BITS_MAX_ENUM_KHR = vks::khr_mir_surface::VK_MIR_SURFACE_CREATE_FLAG_BITS_MAX_ENUM_KHR;
    }
}

/// See [`VkMirSurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMirSurfaceCreateFlagsKHR)
pub type MirSurfaceCreateFlagBitsKhr = MirSurfaceCreateFlagsKhr;

gen_chain_struct! {
    name: MirSurfaceCreateInfoChainKhr [MirSurfaceCreateInfoChainKhrWrapper],
    query: MirSurfaceCreateInfoChainQueryKhr [MirSurfaceCreateInfoChainQueryKhrWrapper],
    vks: vks::khr_mir_surface::VkMirSurfaceCreateInfoKHR,
    input: true,
    output: false,
}

/// See [`VkMirSurfaceCreateInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMirSurfaceCreateInfoKHR)
#[derive(Debug, Clone, PartialEq)]
pub struct MirSurfaceCreateInfoKhr {
    pub flags: MirSurfaceCreateFlagsKhr,
    pub connection: *mut mir_types::MirConnection,
    pub mir_surface: *mut mir_types::MirSurface,
    pub chain: Option<MirSurfaceCreateInfoChainKhr>,
}

#[derive(Debug)]
pub(crate) struct VkMirSurfaceCreateInfoKHRWrapper {
    pub vks_struct: vks::khr_mir_surface::VkMirSurfaceCreateInfoKHR,
    chain: Option<MirSurfaceCreateInfoChainKhrWrapper>,
}

impl VkMirSurfaceCreateInfoKHRWrapper {
    pub fn new(create_info: &MirSurfaceCreateInfoKhr, with_chain: bool) -> Self {
        let (pnext, chain) = MirSurfaceCreateInfoChainKhrWrapper::new_optional(&create_info.chain, with_chain);

        VkMirSurfaceCreateInfoKHRWrapper {
            vks_struct: vks::khr_mir_surface::VkMirSurfaceCreateInfoKHR {
                sType: vks::core::VK_STRUCTURE_TYPE_MIR_SURFACE_CREATE_INFO_KHR,
                pNext: pnext,
                flags: create_info.flags.bits(),
                connection: create_info.connection,
                mirSurface: create_info.mir_surface,
            },
            chain: chain,
        }
    }
}
