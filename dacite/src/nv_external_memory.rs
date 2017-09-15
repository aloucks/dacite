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

//! See extension [`VK_NV_external_memory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_external_memory)

use nv_external_memory_capabilities;
use vks;

gen_chain_struct! {
    name: ExternalMemoryImageCreateInfoChainNv [ExternalMemoryImageCreateInfoChainNvWrapper],
    query: ExternalMemoryImageCreateInfoChainQueryNv [ExternalMemoryImageCreateInfoChainQueryNvWrapper],
    vks: vks::nv_external_memory::VkExternalMemoryImageCreateInfoNV,
    input: true,
    output: false,
}

/// See [`VkExternalMemoryImageCreateInfoNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryImageCreateInfoNV)
#[derive(Debug, Clone, PartialEq)]
pub struct ExternalMemoryImageCreateInfoNv {
    pub handle_types: nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNv,
    pub chain: Option<ExternalMemoryImageCreateInfoChainNv>,
}

#[derive(Debug)]
pub(crate) struct VkExternalMemoryImageCreateInfoNVWrapper {
    pub vks_struct: vks::nv_external_memory::VkExternalMemoryImageCreateInfoNV,
    chain: Option<ExternalMemoryImageCreateInfoChainNvWrapper>,
}

impl VkExternalMemoryImageCreateInfoNVWrapper {
    pub fn new(create_info: &ExternalMemoryImageCreateInfoNv, with_chain: bool) -> Self {
        let (pnext, chain) = ExternalMemoryImageCreateInfoChainNvWrapper::new_optional(&create_info.chain, with_chain);

        VkExternalMemoryImageCreateInfoNVWrapper {
            vks_struct: vks::nv_external_memory::VkExternalMemoryImageCreateInfoNV {
                sType: vks::core::VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV,
                pNext: pnext,
                handleTypes: create_info.handle_types.bits(),
            },
            chain: chain,
        }
    }
}

gen_chain_struct! {
    name: ExportMemoryAllocateInfoChainNv [ExportMemoryAllocateInfoChainNvWrapper],
    query: ExportMemoryAllocateInfoChainQueryNv [ExportMemoryAllocateInfoChainQueryNvWrapper],
    vks: vks::nv_external_memory::VkExportMemoryAllocateInfoNV,
    input: true,
    output: false,
}

/// See [`VkExportMemoryAllocateInfoNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExportMemoryAllocateInfoNV)
#[derive(Debug, Clone, PartialEq)]
pub struct ExportMemoryAllocateInfoNv {
    pub handle_types: nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagBitsNv,
    pub chain: Option<ExportMemoryAllocateInfoChainNv>,
}

#[derive(Debug)]
pub(crate) struct VkExportMemoryAllocateInfoNVWrapper {
    pub vks_struct: vks::nv_external_memory::VkExportMemoryAllocateInfoNV,
    chain: Option<ExportMemoryAllocateInfoChainNvWrapper>,
}

impl VkExportMemoryAllocateInfoNVWrapper {
    pub fn new(info: &ExportMemoryAllocateInfoNv, with_chain: bool) -> Self {
        let (pnext, chain) = ExportMemoryAllocateInfoChainNvWrapper::new_optional(&info.chain, with_chain);

        VkExportMemoryAllocateInfoNVWrapper {
            vks_struct: vks::nv_external_memory::VkExportMemoryAllocateInfoNV {
                sType: vks::core::VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_NV,
                pNext: pnext,
                handleTypes: info.handle_types.bit(),
            },
            chain: chain,
        }
    }
}
