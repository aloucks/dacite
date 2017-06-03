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

use mir_wrapper;
use vks;

/// See [`VkMirSurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMirSurfaceCreateFlagsKHR)
pub type MirSurfaceCreateFlagsKhr = vks::VkMirSurfaceCreateFlagsKHR;

/// See [`VkMirSurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMirSurfaceCreateFlagsKHR)
pub type MirSurfaceCreateFlagBitsKhr = vks::VkMirSurfaceCreateFlagBitsKHR;

chain_struct! {
    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct MirSurfaceCreateInfoChainKhr {
    }

    #[derive(Debug)]
    struct MirSurfaceCreateInfoChainKhrWrapper;
}

/// See [`VkMirSurfaceCreateInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMirSurfaceCreateInfoKHR)
#[derive(Debug, Clone, PartialEq)]
pub struct MirSurfaceCreateInfoKhr {
    pub flags: MirSurfaceCreateFlagsKhr,
    pub connection: *mut mir_wrapper::MirConnection,
    pub mir_surface: *mut mir_wrapper::MirSurface,
    pub chain: Option<MirSurfaceCreateInfoChainKhr>,
}

#[derive(Debug)]
pub(crate) struct VkMirSurfaceCreateInfoKHRWrapper {
    pub vks_struct: vks::VkMirSurfaceCreateInfoKHR,
    chain: Option<MirSurfaceCreateInfoChainKhrWrapper>,
}

impl VkMirSurfaceCreateInfoKHRWrapper {
    pub fn new(create_info: &MirSurfaceCreateInfoKhr, with_chain: bool) -> Self {
        let (pnext, chain) = MirSurfaceCreateInfoChainKhrWrapper::new_optional(&create_info.chain, with_chain);

        VkMirSurfaceCreateInfoKHRWrapper {
            vks_struct: vks::VkMirSurfaceCreateInfoKHR {
                sType: vks::VK_STRUCTURE_TYPE_MIR_SURFACE_CREATE_INFO_KHR,
                pNext: pnext,
                flags: create_info.flags,
                connection: create_info.connection,
                mirSurface: create_info.mir_surface,
            },
            chain: chain,
        }
    }
}
