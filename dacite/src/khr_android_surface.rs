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

//! See extension [`VK_KHR_android_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_android_surface)

use android_types;
use vks;

dacite_bitflags! {
    /// See [`VkAndroidSurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAndroidSurfaceCreateFlagsKHR)
    pub struct AndroidSurfaceCreateFlagsKhr: vks::khr_android_surface::VkAndroidSurfaceCreateFlagsKHR;
    pub enum AndroidSurfaceCreateFlagBitsKhr: vks::khr_android_surface::VkAndroidSurfaceCreateFlagBitsKHR;
    max_enum: vks::khr_android_surface::VK_ANDROID_SURFACE_CREATE_FLAG_BITS_MAX_ENUM_KHR;

    flags {}
    no_bits {}
}

gen_chain_struct! {
    name: AndroidSurfaceCreateInfoChainKhr [AndroidSurfaceCreateInfoChainKhrWrapper],
    query: AndroidSurfaceCreateInfoChainQueryKhr [AndroidSurfaceCreateInfoChainQueryKhrWrapper],
    vks: vks::khr_android_surface::VkAndroidSurfaceCreateInfoKHR,
    input: true,
    output: false,
}

/// See [`VkAndroidSurfaceCreateInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAndroidSurfaceCreateInfoKHR)
#[derive(Debug, Clone, PartialEq)]
pub struct AndroidSurfaceCreateInfoKhr {
    pub flags: AndroidSurfaceCreateFlagsKhr,
    pub window: *mut android_types::ANativeWindow,
    pub chain: Option<AndroidSurfaceCreateInfoChainKhr>,
}

#[derive(Debug)]
pub(crate) struct VkAndroidSurfaceCreateInfoKHRWrapper {
    pub vks_struct: vks::khr_android_surface::VkAndroidSurfaceCreateInfoKHR,
    chain: Option<AndroidSurfaceCreateInfoChainKhrWrapper>,
}

impl VkAndroidSurfaceCreateInfoKHRWrapper {
    pub fn new(create_info: &AndroidSurfaceCreateInfoKhr, with_chain: bool) -> Self {
        let (pnext, chain) = AndroidSurfaceCreateInfoChainKhrWrapper::new_optional(&create_info.chain, with_chain);

        VkAndroidSurfaceCreateInfoKHRWrapper {
            vks_struct: vks::khr_android_surface::VkAndroidSurfaceCreateInfoKHR {
                sType: vks::vk::VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR,
                pNext: pnext,
                flags: create_info.flags.bits(),
                window: create_info.window,
            },
            chain: chain,
        }
    }
}
