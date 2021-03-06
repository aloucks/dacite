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

//! See extension [`VK_KHR_win32_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_win32_surface)

use vks;
use win32_types;

dacite_bitflags! {
    /// See [`VkWin32SurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkWin32SurfaceCreateFlagsKHR)
    pub struct Win32SurfaceCreateFlagsKhr: vks::khr_win32_surface::VkWin32SurfaceCreateFlagsKHR;
    pub enum Win32SurfaceCreateFlagBitsKhr: vks::khr_win32_surface::VkWin32SurfaceCreateFlagBitsKHR;
    max_enum: vks::khr_win32_surface::VK_WIN32_SURFACE_CREATE_FLAG_BITS_MAX_ENUM_KHR;

    flags {}
    no_bits {}
}

gen_chain_struct! {
    name: Win32SurfaceCreateInfoChainKhr [Win32SurfaceCreateInfoChainKhrWrapper],
    query: Win32SurfaceCreateInfoChainQueryKhr [Win32SurfaceCreateInfoChainQueryKhrWrapper],
    vks: vks::khr_win32_surface::VkWin32SurfaceCreateInfoKHR,
    input: true,
    output: false,
}

/// See [`VkWin32SurfaceCreateInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkWin32SurfaceCreateInfoKHR)
#[derive(Debug, Clone, PartialEq)]
pub struct Win32SurfaceCreateInfoKhr {
    pub flags: Win32SurfaceCreateFlagsKhr,
    pub hinstance: win32_types::HINSTANCE,
    pub hwnd: win32_types::HWND,
    pub chain: Option<Win32SurfaceCreateInfoChainKhr>,
}

#[derive(Debug)]
pub(crate) struct VkWin32SurfaceCreateInfoKHRWrapper {
    pub vks_struct: vks::khr_win32_surface::VkWin32SurfaceCreateInfoKHR,
    chain: Option<Win32SurfaceCreateInfoChainKhrWrapper>,
}

impl VkWin32SurfaceCreateInfoKHRWrapper {
    pub fn new(create_info: &Win32SurfaceCreateInfoKhr, with_chain: bool) -> Self {
        let (pnext, chain) = Win32SurfaceCreateInfoChainKhrWrapper::new_optional(&create_info.chain, with_chain);

        VkWin32SurfaceCreateInfoKHRWrapper {
            vks_struct: vks::khr_win32_surface::VkWin32SurfaceCreateInfoKHR {
                sType: vks::vk::VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR,
                pNext: pnext,
                flags: create_info.flags.bits(),
                hinstance: create_info.hinstance,
                hwnd: create_info.hwnd,
            },
            chain: chain,
        }
    }
}
