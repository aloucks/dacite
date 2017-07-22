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

//! See extension [`VK_NV_win32_keyed_mutex`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_win32_keyed_mutex)

use core;
use std::ptr;
use vks;

gen_chain_struct! {
    name: Win32KeyedMutexAcquireReleaseInfoChainNv [Win32KeyedMutexAcquireReleaseInfoChainNvWrapper],
    query: Win32KeyedMutexAcquireReleaseInfoChainQueryNv [Win32KeyedMutexAcquireReleaseInfoChainQueryNvWrapper],
    vks: vks::nv_win32_keyed_mutex::VkWin32KeyedMutexAcquireReleaseInfoNV,
    input: true,
    output: false,
}

/// See [`VkWin32KeyedMutexAcquireReleaseInfoNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkWin32KeyedMutexAcquireReleaseInfoNV)
#[derive(Debug, Clone, PartialEq)]
pub struct Win32KeyedMutexAcquireReleaseInfoNv {
    pub acquire_syncs: Vec<core::DeviceMemory>,
    pub acquire_keys: Vec<u64>,
    pub acquire_timeout_milliseconds: Vec<u32>,
    pub release_syncs: Vec<core::DeviceMemory>,
    pub release_keys: Vec<u64>,
    pub chain: Option<Win32KeyedMutexAcquireReleaseInfoChainNv>,
}

#[derive(Debug)]
pub(crate) struct VkWin32KeyedMutexAcquireReleaseInfoNVWrapper {
    pub vks_struct: vks::nv_win32_keyed_mutex::VkWin32KeyedMutexAcquireReleaseInfoNV,
    acquire_syncs: Vec<core::DeviceMemory>,
    vk_acquire_syncs: Vec<vks::core::VkDeviceMemory>,
    acquire_keys: Vec<u64>,
    acquire_timeout_milliseconds: Vec<u32>,
    release_syncs: Vec<core::DeviceMemory>,
    vk_release_syncs: Vec<vks::core::VkDeviceMemory>,
    release_keys: Vec<u64>,
    chain: Option<Win32KeyedMutexAcquireReleaseInfoChainNvWrapper>,
}

impl VkWin32KeyedMutexAcquireReleaseInfoNVWrapper {
    pub fn new(info: &Win32KeyedMutexAcquireReleaseInfoNv, with_chain: bool) -> Self {
        let acquire_syncs = info.acquire_syncs.clone();
        let vk_acquire_syncs: Vec<_> = acquire_syncs.iter().map(core::DeviceMemory::handle).collect();
        let vk_acquire_syncs_ptr = if !vk_acquire_syncs.is_empty() {
            vk_acquire_syncs.as_ptr()
        }
        else {
            ptr::null()
        };

        let acquire_keys = info.acquire_keys.clone();
        let acquire_timeout_milliseconds = info.acquire_timeout_milliseconds.clone();

        let release_syncs = info.acquire_syncs.clone();
        let vk_release_syncs: Vec<_> = release_syncs.iter().map(core::DeviceMemory::handle).collect();
        let vk_release_syncs_ptr = if !vk_release_syncs.is_empty() {
            vk_release_syncs.as_ptr()
        }
        else {
            ptr::null()
        };

        let release_keys = info.release_keys.clone();

        let (pnext, chain) = Win32KeyedMutexAcquireReleaseInfoChainNvWrapper::new_optional(&info.chain, with_chain);

        VkWin32KeyedMutexAcquireReleaseInfoNVWrapper {
            vks_struct: vks::nv_win32_keyed_mutex::VkWin32KeyedMutexAcquireReleaseInfoNV {
                sType: vks::core::VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV,
                pNext: pnext,
                acquireCount: acquire_syncs.len() as u32,
                pAcquireSyncs: vk_acquire_syncs_ptr,
                pAcquireKeys: acquire_keys.as_ptr(),
                pAcquireTimeoutMilliseconds: acquire_timeout_milliseconds.as_ptr(),
                releaseCount: release_syncs.len() as u32,
                pReleaseSyncs: vk_release_syncs_ptr,
                pReleaseKeys: release_keys.as_ptr(),
            },
            acquire_syncs: acquire_syncs,
            vk_acquire_syncs: vk_acquire_syncs,
            acquire_keys: acquire_keys,
            acquire_timeout_milliseconds: acquire_timeout_milliseconds,
            release_syncs: release_syncs,
            vk_release_syncs: vk_release_syncs,
            release_keys: release_keys,
            chain: chain,
        }
    }
}
