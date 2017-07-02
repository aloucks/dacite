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

//! See extension [`VK_NV_dedicated_allocation`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_dedicated_allocation)

use core;
use utils;
use vks;

gen_chain_struct! {
    name: DedicatedAllocationImageCreateInfoChainNv [DedicatedAllocationImageCreateInfoChainNvWrapper],
    query: DedicatedAllocationImageCreateInfoChainQueryNv [DedicatedAllocationImageCreateInfoChainQueryNvWrapper],
    vks: VkDedicatedAllocationImageCreateInfoNV,
    input: true,
    output: false,
}

/// See [`VkDedicatedAllocationImageCreateInfoNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDedicatedAllocationImageCreateInfoNV)
#[derive(Debug, Clone, PartialEq)]
pub struct DedicatedAllocationImageCreateInfoNv {
    pub dedicated_allocation: bool,
    pub chain: Option<DedicatedAllocationImageCreateInfoChainNv>,
}

#[derive(Debug)]
pub(crate) struct VkDedicatedAllocationImageCreateInfoNVWrapper {
    pub vks_struct: vks::VkDedicatedAllocationImageCreateInfoNV,
    chain: Option<DedicatedAllocationImageCreateInfoChainNvWrapper>,
}

impl VkDedicatedAllocationImageCreateInfoNVWrapper {
    pub fn new(create_info: &DedicatedAllocationImageCreateInfoNv, with_chain: bool) -> Self {
        let (pnext, chain) = DedicatedAllocationImageCreateInfoChainNvWrapper::new_optional(&create_info.chain, with_chain);

        VkDedicatedAllocationImageCreateInfoNVWrapper {
            vks_struct: vks::VkDedicatedAllocationImageCreateInfoNV {
                sType: vks::VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV,
                pNext: pnext,
                dedicatedAllocation: utils::to_vk_bool(create_info.dedicated_allocation),
            },
            chain: chain,
        }
    }
}

gen_chain_struct! {
    name: DedicatedAllocationBufferCreateInfoChainNv [DedicatedAllocationBufferCreateInfoChainNvWrapper],
    query: DedicatedAllocationBufferCreateInfoChainQueryNv [DedicatedAllocationBufferCreateInfoChainQueryNvWrapper],
    vks: VkDedicatedAllocationBufferCreateInfoNV,
    input: true,
    output: false,
}

/// See [`VkDedicatedAllocationBufferCreateInfoNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDedicatedAllocationBufferCreateInfoNV)
#[derive(Debug, Clone, PartialEq)]
pub struct DedicatedAllocationBufferCreateInfoNv {
    pub dedicated_allocation: bool,
    pub chain: Option<DedicatedAllocationBufferCreateInfoChainNv>,
}

#[derive(Debug)]
pub(crate) struct VkDedicatedAllocationBufferCreateInfoNVWrapper {
    pub vks_struct: vks::VkDedicatedAllocationBufferCreateInfoNV,
    chain: Option<DedicatedAllocationBufferCreateInfoChainNvWrapper>,
}

impl VkDedicatedAllocationBufferCreateInfoNVWrapper {
    pub fn new(create_info: &DedicatedAllocationBufferCreateInfoNv, with_chain: bool) -> Self {
        let (pnext, chain) = DedicatedAllocationBufferCreateInfoChainNvWrapper::new_optional(&create_info.chain, with_chain);

        VkDedicatedAllocationBufferCreateInfoNVWrapper {
            vks_struct: vks::VkDedicatedAllocationBufferCreateInfoNV {
                sType: vks::VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV,
                pNext: pnext,
                dedicatedAllocation: utils::to_vk_bool(create_info.dedicated_allocation),
            },
            chain: chain,
        }
    }
}

gen_chain_struct! {
    name: DedicatedAllocationMemoryAllocateInfoChainNv [DedicatedAllocationMemoryAllocateInfoChainNvWrapper],
    query: DedicatedAllocationMemoryAllocateInfoChainQueryNv [DedicatedAllocationMemoryAllocateInfoChainQueryNvWrapper],
    vks: VkDedicatedAllocationMemoryAllocateInfoNV,
    input: true,
    output: false,
}

/// See [`VkDedicatedAllocationMemoryAllocateInfoNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDedicatedAllocationMemoryAllocateInfoNV)
#[derive(Debug, Clone, PartialEq)]
pub struct DedicatedAllocationMemoryAllocateInfoNv {
    pub image: Option<core::Image>,
    pub buffer: Option<core::Buffer>,
    pub chain: Option<DedicatedAllocationMemoryAllocateInfoChainNv>,
}

#[derive(Debug)]
pub(crate) struct VkDedicatedAllocationMemoryAllocateInfoNVWrapper {
    pub vks_struct: vks::VkDedicatedAllocationMemoryAllocateInfoNV,
    image: Option<core::Image>,
    buffer: Option<core::Buffer>,
    chain: Option<DedicatedAllocationMemoryAllocateInfoChainNvWrapper>,
}

impl VkDedicatedAllocationMemoryAllocateInfoNVWrapper {
    pub fn new(info: &DedicatedAllocationMemoryAllocateInfoNv, with_chain: bool) -> Self {
        let (pnext, chain) = DedicatedAllocationMemoryAllocateInfoChainNvWrapper::new_optional(&info.chain, with_chain);

        VkDedicatedAllocationMemoryAllocateInfoNVWrapper {
            vks_struct: vks::VkDedicatedAllocationMemoryAllocateInfoNV {
                sType: vks::VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV,
                pNext: pnext,
                image: info.image.as_ref().map_or(Default::default(), core::Image::handle),
                buffer: info.buffer.as_ref().map_or(Default::default(), core::Buffer::handle),
            },
            chain: chain,
            image: info.image.clone(),
            buffer: info.buffer.clone(),
        }
    }
}
