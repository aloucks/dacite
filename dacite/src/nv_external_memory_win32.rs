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

//! See extension [`VK_NV_external_memory_win32`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_external_memory_win32)

use nv_external_memory_capabilities;
use vks;
use win32_wrapper;

gen_chain_struct! {
    name: ImportMemoryWin32HandleInfoChainNv [ImportMemoryWin32HandleInfoChainNvWrapper],
    query: ImportMemoryWin32HandleInfoChainQueryNv [ImportMemoryWin32HandleInfoChainQueryNvWrapper],
    vks: VkImportMemoryWin32HandleInfoNV,
    input: true,
    output: false,
}

/// See [`VkImportMemoryWin32HandleInfoNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImportMemoryWin32HandleInfoNV)
#[derive(Debug, Clone, PartialEq)]
pub struct ImportMemoryWin32HandleInfoNv {
    pub handle_type: nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNv,
    pub handle: win32_wrapper::HANDLE,
    pub chain: Option<ImportMemoryWin32HandleInfoChainNv>,
}

#[derive(Debug)]
pub(crate) struct VkImportMemoryWin32HandleInfoNVWrapper {
    pub vks_struct: vks::VkImportMemoryWin32HandleInfoNV,
    chain: Option<ImportMemoryWin32HandleInfoChainNvWrapper>,
}

impl VkImportMemoryWin32HandleInfoNVWrapper {
    pub fn new(info: &ImportMemoryWin32HandleInfoNv, with_chain: bool) -> Self {
        let (pnext, chain) = ImportMemoryWin32HandleInfoChainNvWrapper::new_optional(&info.chain, with_chain);

        VkImportMemoryWin32HandleInfoNVWrapper {
            vks_struct: vks::VkImportMemoryWin32HandleInfoNV {
                sType: vks::VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_NV,
                pNext: pnext,
                handleType: info.handle_type.bits(),
                handle: info.handle,
            },
            chain: chain,
        }
    }
}

gen_chain_struct! {
    name: ExportMemoryWin32HandleInfoChainNv [ExportMemoryWin32HandleInfoChainNvWrapper],
    query: ExportMemoryWin32HandleInfoChainQueryNv [ExportMemoryWin32HandleInfoChainQueryNvWrapper],
    vks: VkExportMemoryWin32HandleInfoNV,
    input: true,
    output: false,
}

/// See [`VkExportMemoryWin32HandleInfoNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExportMemoryWin32HandleInfoNV)
#[derive(Debug, Clone, PartialEq)]
pub struct ExportMemoryWin32HandleInfoNv {
    pub attributes: *const win32_wrapper::SECURITY_ATTRIBUTES,
    pub dw_access: win32_wrapper::DWORD,
    pub chain: Option<ExportMemoryWin32HandleInfoChainNv>,
}

#[derive(Debug)]
pub(crate) struct VkExportMemoryWin32HandleInfoNVWrapper {
    pub vks_struct: vks::VkExportMemoryWin32HandleInfoNV,
    chain: Option<ExportMemoryWin32HandleInfoChainNvWrapper>,
}

impl VkExportMemoryWin32HandleInfoNVWrapper {
    pub fn new(info: &ExportMemoryWin32HandleInfoNv, with_chain: bool) -> Self {
        let (pnext, chain) = ExportMemoryWin32HandleInfoChainNvWrapper::new_optional(&info.chain, with_chain);

        VkExportMemoryWin32HandleInfoNVWrapper {
            vks_struct: vks::VkExportMemoryWin32HandleInfoNV {
                sType: vks::VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_NV,
                pNext: pnext,
                pAttributes: info.attributes,
                dwAccess: info.dw_access,
            },
            chain: chain,
        }
    }
}
