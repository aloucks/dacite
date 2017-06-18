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

//! See extension [`VK_KHR_get_physical_device_properties2`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_get_physical_device_properties2)

use core;
use vks;

gen_chain_struct! {
    name: PhysicalDeviceFeatures2ChainKhr [PhysicalDeviceFeatures2ChainKhrWrapper],
    query: PhysicalDeviceFeatures2ChainQueryKhr [PhysicalDeviceFeatures2ChainQueryKhrWrapper],
    vks: VkPhysicalDeviceFeatures2KHR,
    input: true,
    output: true,
}

/// See [`VkPhysicalDeviceFeatures2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceFeatures2KHR)
#[derive(Debug, Clone, PartialEq)]
pub struct PhysicalDeviceFeatures2Khr {
    pub features: core::PhysicalDeviceFeatures,
    pub chain: Option<PhysicalDeviceFeatures2ChainKhr>,
}

impl PhysicalDeviceFeatures2Khr {
    pub(crate) unsafe fn from_vks(features: &vks::VkPhysicalDeviceFeatures2KHR, with_chain: bool) -> Self {
        PhysicalDeviceFeatures2Khr {
            features: (&features.features).into(),
            chain: PhysicalDeviceFeatures2ChainKhr::from_optional_pnext(features.pNext, with_chain),
        }
    }
}

#[derive(Debug)]
pub(crate) struct VkPhysicalDeviceFeatures2KHRWrapper {
    pub vks_struct: vks::VkPhysicalDeviceFeatures2KHR,
    chain: Option<PhysicalDeviceFeatures2ChainKhrWrapper>,
}

impl VkPhysicalDeviceFeatures2KHRWrapper {
    pub fn new(features: &PhysicalDeviceFeatures2Khr, with_chain: bool) -> Self {
        let (pnext, chain) = PhysicalDeviceFeatures2ChainKhrWrapper::new_optional(&features.chain, with_chain);

        VkPhysicalDeviceFeatures2KHRWrapper {
            vks_struct: vks::VkPhysicalDeviceFeatures2KHR {
                sType: vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2_KHR,
                pNext: pnext,
                features: (&features.features).into(),
            },
            chain: chain,
        }
    }
}

gen_chain_struct! {
    name: PhysicalDeviceProperties2ChainKhr [PhysicalDeviceProperties2ChainKhrWrapper],
    query: PhysicalDeviceProperties2ChainQueryKhr [PhysicalDeviceProperties2ChainQueryKhrWrapper],
    vks: VkPhysicalDeviceProperties2KHR,
    input: false,
    output: true,
}

/// See [`VkPhysicalDeviceProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceProperties2KHR)
#[derive(Debug, Clone, PartialEq)]
pub struct PhysicalDeviceProperties2Khr {
    pub properties: core::PhysicalDeviceProperties,
    pub chain: Option<PhysicalDeviceProperties2ChainKhr>,
}

impl PhysicalDeviceProperties2Khr {
    pub(crate) unsafe fn from_vks(properties: &vks::VkPhysicalDeviceProperties2KHR, with_chain: bool) -> Self {
        PhysicalDeviceProperties2Khr {
            properties: (&properties.properties).into(),
            chain: PhysicalDeviceProperties2ChainKhr::from_optional_pnext(properties.pNext, with_chain),
        }
    }
}

gen_chain_struct! {
    name: FormatProperties2ChainKhr [FormatProperties2ChainKhrWrapper],
    query: FormatProperties2ChainQueryKhr [FormatProperties2ChainQueryKhrWrapper],
    vks: VkFormatProperties2KHR,
    input: false,
    output: true,
}

/// See [`VkFormatProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatProperties2KHR)
#[derive(Debug, Clone, PartialEq)]
pub struct FormatProperties2Khr {
    pub format_properties: core::FormatProperties,
    pub chain: Option<FormatProperties2ChainKhr>,
}

impl FormatProperties2Khr {
    pub(crate) unsafe fn from_vks(properties: &vks::VkFormatProperties2KHR, with_chain: bool) -> Self {
        FormatProperties2Khr {
            format_properties: (&properties.formatProperties).into(),
            chain: FormatProperties2ChainKhr::from_optional_pnext(properties.pNext, with_chain),
        }
    }
}

gen_chain_struct! {
    name: ImageFormatProperties2ChainKhr [ImageFormatProperties2ChainKhrWrapper],
    query: ImageFormatProperties2ChainQueryKhr [ImageFormatProperties2ChainQueryKhrWrapper],
    vks: VkImageFormatProperties2KHR,
    input: false,
    output: true,
}

/// See [`VkImageFormatProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageFormatProperties2KHR)
#[derive(Debug, Clone, PartialEq)]
pub struct ImageFormatProperties2Khr {
    pub image_format_properties: core::ImageFormatProperties,
    pub chain: Option<ImageFormatProperties2ChainKhr>,
}

impl ImageFormatProperties2Khr {
    pub(crate) unsafe fn from_vks(properties: &vks::VkImageFormatProperties2KHR, with_chain: bool) -> Self {
        ImageFormatProperties2Khr {
            image_format_properties: (&properties.imageFormatProperties).into(),
            chain: ImageFormatProperties2ChainKhr::from_optional_pnext(properties.pNext, with_chain),
        }
    }
}

gen_chain_struct! {
    name: PhysicalDeviceImageFormatInfo2ChainKhr [PhysicalDeviceImageFormatInfo2ChainKhrWrapper],
    query: PhysicalDeviceImageFormatInfo2ChainQueryKhr [PhysicalDeviceImageFormatInfo2ChainQueryKhrWrapper],
    vks: VkPhysicalDeviceImageFormatInfo2KHR,
    input: true,
    output: false,
}

/// See [`VkPhysicalDeviceImageFormatInfo2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceImageFormatInfo2KHR)
#[derive(Debug, Clone, PartialEq)]
pub struct PhysicalDeviceImageFormatInfo2Khr {
    pub format: core::Format,
    pub image_type: core::ImageType,
    pub tiling: core::ImageTiling,
    pub usage: core::ImageUsageFlags,
    pub flags: core::ImageCreateFlags,
    pub chain: Option<PhysicalDeviceImageFormatInfo2ChainKhr>,
}

#[derive(Debug)]
pub(crate) struct VkPhysicalDeviceImageFormatInfo2KHRWrapper {
    pub vks_struct: vks::VkPhysicalDeviceImageFormatInfo2KHR,
    chain: Option<PhysicalDeviceImageFormatInfo2ChainKhrWrapper>,
}

impl VkPhysicalDeviceImageFormatInfo2KHRWrapper {
    pub fn new(info: &PhysicalDeviceImageFormatInfo2Khr, with_chain: bool) -> Self {
        let (pnext, chain) = PhysicalDeviceImageFormatInfo2ChainKhrWrapper::new_optional(&info.chain, with_chain);

        VkPhysicalDeviceImageFormatInfo2KHRWrapper  {
            vks_struct: vks::VkPhysicalDeviceImageFormatInfo2KHR {
                sType: vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR,
                pNext: pnext,
                format: info.format.into(),
                type_: info.image_type.into(),
                tiling: info.tiling.into(),
                usage: info.usage.bits(),
                flags: info.flags.bits(),
            },
            chain: chain,
        }
    }
}

gen_chain_struct! {
    name: QueueFamilyProperties2ChainKhr [QueueFamilyProperties2ChainKhrWrapper],
    query: QueueFamilyProperties2ChainQueryKhr [QueueFamilyProperties2ChainQueryKhrWrapper],
    vks: VkQueueFamilyProperties2KHR,
    input: false,
    output: true,
}

/// See [`VkQueueFamilyProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueueFamilyProperties2KHR)
#[derive(Debug, Clone, PartialEq)]
pub struct QueueFamilyProperties2Khr {
    pub queue_family_properties: core::QueueFamilyProperties,
    pub chain: Option<QueueFamilyProperties2ChainKhr>,
}

impl QueueFamilyProperties2Khr {
    pub(crate) unsafe fn from_vks(properties: &vks::VkQueueFamilyProperties2KHR, with_chain: bool) -> Self {
        QueueFamilyProperties2Khr {
            queue_family_properties: (&properties.queueFamilyProperties).into(),
            chain: QueueFamilyProperties2ChainKhr::from_optional_pnext(properties.pNext, with_chain),
        }
    }
}

gen_chain_struct! {
    name: PhysicalDeviceMemoryProperties2ChainKhr [PhysicalDeviceMemoryProperties2ChainKhrWrapper],
    query: PhysicalDeviceMemoryProperties2ChainQueryKhr [PhysicalDeviceMemoryProperties2ChainQueryKhrWrapper],
    vks: VkPhysicalDeviceMemoryProperties2KHR,
    input: false,
    output: true,
}

/// See [`VkPhysicalDeviceMemoryProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceMemoryProperties2KHR)
#[derive(Debug, Clone, PartialEq)]
pub struct PhysicalDeviceMemoryProperties2Khr {
    pub memory_properties: core::PhysicalDeviceMemoryProperties,
    pub chain: Option<PhysicalDeviceMemoryProperties2ChainKhr>,
}

impl PhysicalDeviceMemoryProperties2Khr {
    pub(crate) unsafe fn from_vks(properties: &vks::VkPhysicalDeviceMemoryProperties2KHR, with_chain: bool) -> Self {
        PhysicalDeviceMemoryProperties2Khr {
            memory_properties: (&properties.memoryProperties).into(),
            chain: PhysicalDeviceMemoryProperties2ChainKhr::from_optional_pnext(properties.pNext, with_chain),
        }
    }
}

gen_chain_struct! {
    name: SparseImageFormatProperties2ChainKhr [SparseImageFormatProperties2ChainKhrWrapper],
    query: SparseImageFormatProperties2ChainQueryKhr [SparseImageFormatProperties2ChainQueryKhrWrapper],
    vks: VkSparseImageFormatProperties2KHR,
    input: false,
    output: true,
}

/// See [`VkSparseImageFormatProperties2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseImageFormatProperties2KHR)
#[derive(Debug, Clone, PartialEq)]
pub struct SparseImageFormatProperties2Khr {
    pub properties: core::SparseImageFormatProperties,
    pub chain: Option<SparseImageFormatProperties2ChainKhr>,
}

impl SparseImageFormatProperties2Khr {
    pub(crate) unsafe fn from_vks(properties: &vks::VkSparseImageFormatProperties2KHR, with_chain: bool) -> Self {
        SparseImageFormatProperties2Khr {
            properties: (&properties.properties).into(),
            chain: SparseImageFormatProperties2ChainKhr::from_optional_pnext(properties.pNext, with_chain),
        }
    }
}

gen_chain_struct! {
    name: PhysicalDeviceSparseImageFormatInfo2ChainKhr [PhysicalDeviceSparseImageFormatInfo2ChainKhrWrapper],
    query: PhysicalDeviceSparseImageFormatInfo2ChainQueryKhr [PhysicalDeviceSparseImageFormatInfo2ChainQueryKhrWrapper],
    vks: VkPhysicalDeviceSparseImageFormatInfo2KHR,
    input: true,
    output: false,
}

/// See [`VkPhysicalDeviceSparseImageFormatInfo2KHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceSparseImageFormatInfo2KHR)
#[derive(Debug, Clone, PartialEq)]
pub struct PhysicalDeviceSparseImageFormatInfo2Khr {
    pub format: core::Format,
    pub image_type: core::ImageType,
    pub samples: core::SampleCountFlagBits,
    pub usage: core::ImageUsageFlags,
    pub tiling: core::ImageTiling,
    pub chain: Option<PhysicalDeviceSparseImageFormatInfo2ChainKhr>,
}

#[derive(Debug)]
pub(crate) struct VkPhysicalDeviceSparseImageFormatInfo2KHRWrapper {
    pub vks_struct: vks::VkPhysicalDeviceSparseImageFormatInfo2KHR,
    chain: Option<PhysicalDeviceSparseImageFormatInfo2ChainKhrWrapper>,
}

impl VkPhysicalDeviceSparseImageFormatInfo2KHRWrapper {
    pub fn new(info: &PhysicalDeviceSparseImageFormatInfo2Khr, with_chain: bool) -> Self {
        let (pnext, chain) = PhysicalDeviceSparseImageFormatInfo2ChainKhrWrapper::new_optional(&info.chain, with_chain);

        VkPhysicalDeviceSparseImageFormatInfo2KHRWrapper  {
            vks_struct: vks::VkPhysicalDeviceSparseImageFormatInfo2KHR {
                sType: vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR,
                pNext: pnext,
                format: info.format.into(),
                type_: info.image_type.into(),
                samples: info.samples.bits(),
                usage: info.usage.bits(),
                tiling: info.tiling.into(),
            },
            chain: chain,
        }
    }
}
