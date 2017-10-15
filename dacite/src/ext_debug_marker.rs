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

//! See extension [`VK_EXT_debug_marker`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_marker)

use ext_debug_report;
use libc;
use std::ffi::CString;
use vks;

gen_chain_struct! {
    name: DebugMarkerObjectNameInfoChainExt [DebugMarkerObjectNameInfoChainExtWrapper],
    query: DebugMarkerObjectNameInfoChainQueryExt [DebugMarkerObjectNameInfoChainQueryExtWrapper],
    vks: vks::ext_debug_marker::VkDebugMarkerObjectNameInfoEXT,
    input: true,
    output: false,
}

/// See [`VkDebugMarkerObjectNameInfoEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugMarkerObjectNameInfoEXT)
#[derive(Debug, Clone, PartialEq)]
pub struct DebugMarkerObjectNameInfoExt {
    pub object_type: ext_debug_report::DebugReportObjectTypeExt,
    pub object: u64,
    pub object_name: String,
    pub chain: Option<DebugMarkerObjectNameInfoChainExt>,
}

#[derive(Debug)]
pub(crate) struct VkDebugMarkerObjectNameInfoEXTWrapper {
    pub vks_struct: vks::ext_debug_marker::VkDebugMarkerObjectNameInfoEXT,
    object_name: CString,
    chain: Option<DebugMarkerObjectNameInfoChainExtWrapper>,
}

impl VkDebugMarkerObjectNameInfoEXTWrapper {
    pub fn new(info: &DebugMarkerObjectNameInfoExt, with_chain: bool) -> Self {
        let object_name = CString::new(info.object_name.as_str()).unwrap();
        let (pnext, chain) = DebugMarkerObjectNameInfoChainExtWrapper::new_optional(&info.chain, with_chain);

        VkDebugMarkerObjectNameInfoEXTWrapper {
            vks_struct: vks::ext_debug_marker::VkDebugMarkerObjectNameInfoEXT {
                sType: vks::vk::VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_NAME_INFO_EXT,
                pNext: pnext,
                objectType: info.object_type.into(),
                object: info.object,
                pObjectName: object_name.as_ptr(),
            },
            object_name: object_name,
            chain: chain,
        }
    }
}

gen_chain_struct! {
    name: DebugMarkerObjectTagInfoChainExt [DebugMarkerObjectTagInfoChainExtWrapper],
    query: DebugMarkerObjectTagInfoChainQueryExt [DebugMarkerObjectTagInfoChainQueryExtWrapper],
    vks: vks::ext_debug_marker::VkDebugMarkerObjectTagInfoEXT,
    input: true,
    output: false,
}

/// See [`VkDebugMarkerObjectTagInfoEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugMarkerObjectTagInfoEXT)
#[derive(Debug, Clone, PartialEq)]
pub struct DebugMarkerObjectTagInfoExt {
    pub object_type: ext_debug_report::DebugReportObjectTypeExt,
    pub object: u64,
    pub tag_name: u64,
    pub tag: Vec<u8>,
    pub chain: Option<DebugMarkerObjectTagInfoChainExt>,
}

#[derive(Debug)]
pub(crate) struct VkDebugMarkerObjectTagInfoEXTWrapper {
    pub vks_struct: vks::ext_debug_marker::VkDebugMarkerObjectTagInfoEXT,
    tag: Vec<u8>,
    chain: Option<DebugMarkerObjectTagInfoChainExtWrapper>,
}

impl VkDebugMarkerObjectTagInfoEXTWrapper {
    pub fn new(info: &DebugMarkerObjectTagInfoExt, with_chain: bool) -> Self {
        let tag = info.tag.clone();
        let (pnext, chain) = DebugMarkerObjectTagInfoChainExtWrapper::new_optional(&info.chain, with_chain);

        VkDebugMarkerObjectTagInfoEXTWrapper {
            vks_struct: vks::ext_debug_marker::VkDebugMarkerObjectTagInfoEXT {
                sType: vks::vk::VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_TAG_INFO_EXT,
                pNext: pnext,
                objectType: info.object_type.into(),
                object: info.object,
                tagName: info.tag_name,
                tagSize: tag.len(),
                pTag: tag.as_ptr() as *const libc::c_void,
            },
            tag: tag,
            chain: chain,
        }
    }
}

gen_chain_struct! {
    name: DebugMarkerMarkerInfoChainExt [DebugMarkerMarkerInfoChainExtWrapper],
    query: DebugMarkerMarkerInfoChainQueryExt [DebugMarkerMarkerInfoChainQueryExtWrapper],
    vks: vks::ext_debug_marker::VkDebugMarkerMarkerInfoEXT,
    input: true,
    output: false,
}

/// See [`VkDebugMarkerMarkerInfoEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugMarkerMarkerInfoEXT)
#[derive(Debug, Clone, PartialEq)]
pub struct DebugMarkerMarkerInfoExt {
    pub marker_name: String,
    pub color: [f32; 4],
    pub chain: Option<DebugMarkerMarkerInfoChainExt>,
}

#[derive(Debug)]
pub(crate) struct VkDebugMarkerMarkerInfoEXTWrapper {
    pub vks_struct: vks::ext_debug_marker::VkDebugMarkerMarkerInfoEXT,
    marker_name: CString,
    chain: Option<DebugMarkerMarkerInfoChainExtWrapper>,
}

impl VkDebugMarkerMarkerInfoEXTWrapper {
    pub fn new(info: &DebugMarkerMarkerInfoExt, with_chain: bool) -> Self {
        let marker_name = CString::new(info.marker_name.as_str()).unwrap();
        let (pnext, chain) = DebugMarkerMarkerInfoChainExtWrapper::new_optional(&info.chain, with_chain);

        VkDebugMarkerMarkerInfoEXTWrapper {
            vks_struct: vks::ext_debug_marker::VkDebugMarkerMarkerInfoEXT {
                sType: vks::vk::VK_STRUCTURE_TYPE_DEBUG_MARKER_MARKER_INFO_EXT,
                pNext: pnext,
                pMarkerName: marker_name.as_ptr(),
                color: info.color,
            },
            marker_name: marker_name,
            chain: chain,
        }
    }
}
