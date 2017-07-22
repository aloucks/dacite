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

bitflags! {
    /// See [`VkExternalMemoryHandleTypeFlagBitsNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryHandleTypeFlagBitsNV)
    #[derive(Default)]
    pub struct ExternalMemoryHandleTypeFlagsNv: vks::nv_external_memory_capabilities::VkExternalMemoryHandleTypeFlagsNV {
        /// See [`VkExternalMemoryHandleTypeFlagBitsNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryHandleTypeFlagBitsNV)
        const EXTERNAL_MEMORY_HANDLE_TYPE_FLAG_BITS_MAX_ENUM_NV = vks::nv_external_memory_capabilities::VK_EXTERNAL_MEMORY_HANDLE_TYPE_FLAG_BITS_MAX_ENUM_NV;

        /// See [`VkExternalMemoryHandleTypeFlagBitsNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryHandleTypeFlagBitsNV)
        const EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_NV = vks::nv_external_memory_capabilities::VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_NV;

        /// See [`VkExternalMemoryHandleTypeFlagBitsNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryHandleTypeFlagBitsNV)
        const EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_NV = vks::nv_external_memory_capabilities::VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_NV;

        /// See [`VkExternalMemoryHandleTypeFlagBitsNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryHandleTypeFlagBitsNV)
        const EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_BIT_NV = vks::nv_external_memory_capabilities::VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_BIT_NV;

        /// See [`VkExternalMemoryHandleTypeFlagBitsNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryHandleTypeFlagBitsNV)
        const EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_KMT_BIT_NV = vks::nv_external_memory_capabilities::VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_KMT_BIT_NV;
    }
}

/// See [`VkExternalMemoryHandleTypeFlagBitsNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryHandleTypeFlagBitsNV)
pub type ExternalMemoryHandleTypeFlagBitsNv = ExternalMemoryHandleTypeFlagsNv;

bitflags! {
    /// See [`VkExternalMemoryFeatureFlagBitsNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryFeatureFlagBitsNV)
    #[derive(Default)]
    pub struct ExternalMemoryFeatureFlagsNv: vks::nv_external_memory_capabilities::VkExternalMemoryFeatureFlagsNV {
        /// See [`VkExternalMemoryFeatureFlagBitsNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryFeatureFlagBitsNV)
        const EXTERNAL_MEMORY_FEATURE_FLAG_BITS_MAX_ENUM_NV = vks::nv_external_memory_capabilities::VK_EXTERNAL_MEMORY_FEATURE_FLAG_BITS_MAX_ENUM_NV;

        /// See [`VkExternalMemoryFeatureFlagBitsNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryFeatureFlagBitsNV)
        const EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_NV = vks::nv_external_memory_capabilities::VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_NV;

        /// See [`VkExternalMemoryFeatureFlagBitsNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryFeatureFlagBitsNV)
        const EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_NV = vks::nv_external_memory_capabilities::VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_NV;

        /// See [`VkExternalMemoryFeatureFlagBitsNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryFeatureFlagBitsNV)
        const EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_NV = vks::nv_external_memory_capabilities::VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_NV;
    }
}

/// See [`VkExternalMemoryFeatureFlagBitsNV`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExternalMemoryFeatureFlagBitsNV)
pub type ExternalMemoryFeatureFlagBitsNv = ExternalMemoryFeatureFlagsNv;

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
