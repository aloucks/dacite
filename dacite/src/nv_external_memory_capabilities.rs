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

//! See extension [`VK_NV_external_memory_capabilities`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_external_memory_capabilities)

use core;
use vks;

dacite_bitflags! {
    /// See [`VkExternalMemoryHandleTypeFlagBitsNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryHandleTypeFlagBitsNV)
    pub struct ExternalMemoryHandleTypeFlagsNv: vks::nv_external_memory_capabilities::VkExternalMemoryHandleTypeFlagsNV;
    pub enum ExternalMemoryHandleTypeFlagBitsNv: vks::nv_external_memory_capabilities::VkExternalMemoryHandleTypeFlagBitsNV;
    max_enum: vks::nv_external_memory_capabilities::VK_EXTERNAL_MEMORY_HANDLE_TYPE_FLAG_BITS_MAX_ENUM_NV;

    flags {
        const OPAQUE_WIN32 [OpaqueWin32] = vks::nv_external_memory_capabilities::VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_NV;
        const OPAQUE_WIN32_KMT [OpaqueWin32Kmt] = vks::nv_external_memory_capabilities::VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_NV;
        const D3D11_IMAGE [D3D11Image] = vks::nv_external_memory_capabilities::VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_BIT_NV;
        const D3D11_IMAGE_KMT [D3D11ImageKmt] = vks::nv_external_memory_capabilities::VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_KMT_BIT_NV;
    }

    no_bits {}
}

dacite_bitflags! {
    /// See [`VkExternalMemoryFeatureFlagBitsNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryFeatureFlagBitsNV)
    pub struct ExternalMemoryFeatureFlagsNv: vks::nv_external_memory_capabilities::VkExternalMemoryFeatureFlagsNV;
    pub enum ExternalMemoryFeatureFlagBitsNv: vks::nv_external_memory_capabilities::VkExternalMemoryFeatureFlagBitsNV;
    max_enum: vks::nv_external_memory_capabilities::VK_EXTERNAL_MEMORY_FEATURE_FLAG_BITS_MAX_ENUM_NV;

    flags {
        const DEDICATED_ONLY [DedicatedOnly] = vks::nv_external_memory_capabilities::VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_NV;
        const EXPORTABLE [Exportable] = vks::nv_external_memory_capabilities::VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_NV;
        const IMPORTABLE [Importable] = vks::nv_external_memory_capabilities::VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_NV;
    }

    no_bits {}
}

/// See [`VkExternalImageFormatPropertiesNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalImageFormatPropertiesNV)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ExternalImageFormatPropertiesNv {
    pub image_format_properties: core::ImageFormatProperties,
    pub external_memory_features: ExternalMemoryFeatureFlagsNv,
    pub export_from_imported_handle_types: ExternalMemoryHandleTypeFlagsNv,
    pub compatible_handle_types: ExternalMemoryHandleTypeFlagsNv,
}

impl<'a> From<&'a vks::nv_external_memory_capabilities::VkExternalImageFormatPropertiesNV> for ExternalImageFormatPropertiesNv {
    fn from(properties: &'a vks::nv_external_memory_capabilities::VkExternalImageFormatPropertiesNV) -> Self {
        ExternalImageFormatPropertiesNv {
            image_format_properties: (&properties.imageFormatProperties).into(),
            external_memory_features: ExternalMemoryFeatureFlagsNv::from_bits_truncate(properties.externalMemoryFeatures),
            export_from_imported_handle_types: ExternalMemoryHandleTypeFlagsNv::from_bits_truncate(properties.exportFromImportedHandleTypes),
            compatible_handle_types: ExternalMemoryHandleTypeFlagsNv::from_bits_truncate(properties.compatibleHandleTypes),
        }
    }
}
