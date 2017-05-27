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

//! See extension [`VK_KHR_display_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display_swapchain)

use core;
use utils;
use vks;

chain_struct! {
    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct DisplayPresentInfoKhrChain {
    }

    #[derive(Debug)]
    struct DisplayPresentInfoKhrChainWrapper;
}

/// See [`VkDisplayPresentInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayPresentInfoKHR)
#[derive(Debug, Clone, PartialEq)]
pub struct DisplayPresentInfoKhr {
    pub src_rect: core::Rect2D,
    pub dst_rect: core::Rect2D,
    pub persistent: bool,
    pub chain: Option<DisplayPresentInfoKhrChain>,
}

#[derive(Debug)]
pub(crate) struct VkDisplayPresentInfoKHRWrapper {
    pub vks_struct: vks::VkDisplayPresentInfoKHR,
    chain: Option<DisplayPresentInfoKhrChainWrapper>,
}

impl VkDisplayPresentInfoKHRWrapper {
    pub fn new(info: &DisplayPresentInfoKhr, with_chain: bool) -> Self {
        let (pnext, chain) = DisplayPresentInfoKhrChainWrapper::new_optional(&info.chain, with_chain);

        VkDisplayPresentInfoKHRWrapper {
            vks_struct: vks::VkDisplayPresentInfoKHR {
                sType: vks::VK_STRUCTURE_TYPE_DISPLAY_PRESENT_INFO_KHR,
                pNext: pnext,
                srcRect: (&info.src_rect).into(),
                dstRect: (&info.dst_rect).into(),
                persistent: utils::to_vk_bool(info.persistent),
            },
            chain: chain,
        }
    }
}
