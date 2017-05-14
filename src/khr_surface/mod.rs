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

//! See extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)

mod surface;

use core;
use vks;

pub use self::surface::SurfaceKHR;

/// See [`VkSurfaceTransformFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceTransformFlagBitsKHR)
pub type SurfaceTransformFlagsKHR = vks::VkSurfaceTransformFlagsKHR;

/// See [`VkSurfaceTransformFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceTransformFlagBitsKHR)
pub const SURFACE_TRANSFORM_IDENTITY_BIT_KHR: SurfaceTransformFlagsKHR = vks::VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR;

/// See [`VkSurfaceTransformFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceTransformFlagBitsKHR)
pub const SURFACE_TRANSFORM_ROTATE_90_BIT_KHR: SurfaceTransformFlagsKHR = vks::VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR;

/// See [`VkSurfaceTransformFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceTransformFlagBitsKHR)
pub const SURFACE_TRANSFORM_ROTATE_180_BIT_KHR: SurfaceTransformFlagsKHR = vks::VK_SURFACE_TRANSFORM_ROTATE_180_BIT_KHR;

/// See [`VkSurfaceTransformFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceTransformFlagBitsKHR)
pub const SURFACE_TRANSFORM_ROTATE_270_BIT_KHR: SurfaceTransformFlagsKHR = vks::VK_SURFACE_TRANSFORM_ROTATE_270_BIT_KHR;

/// See [`VkSurfaceTransformFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceTransformFlagBitsKHR)
pub const SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR: SurfaceTransformFlagsKHR = vks::VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR;

/// See [`VkSurfaceTransformFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceTransformFlagBitsKHR)
pub const SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR: SurfaceTransformFlagsKHR = vks::VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR;

/// See [`VkSurfaceTransformFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceTransformFlagBitsKHR)
pub const SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR: SurfaceTransformFlagsKHR = vks::VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR;

/// See [`VkSurfaceTransformFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceTransformFlagBitsKHR)
pub const SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR: SurfaceTransformFlagsKHR = vks::VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR;

/// See [`VkSurfaceTransformFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceTransformFlagBitsKHR)
pub const SURFACE_TRANSFORM_INHERIT_BIT_KHR: SurfaceTransformFlagsKHR = vks::VK_SURFACE_TRANSFORM_INHERIT_BIT_KHR;

/// See [`VkSurfaceTransformFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceTransformFlagBitsKHR)
pub type SurfaceTransformFlagBitsKHR = vks::VkSurfaceTransformFlagBitsKHR ;

/// See [`VkCompositeAlphaFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCompositeAlphaFlagBitsKHR)
pub type CompositeAlphaFlagsKHR = vks::VkCompositeAlphaFlagsKHR;

/// See [`VkCompositeAlphaFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCompositeAlphaFlagBitsKHR)
pub const COMPOSITE_ALPHA_OPAQUE_BIT_KHR: CompositeAlphaFlagsKHR = vks::VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR;

/// See [`VkCompositeAlphaFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCompositeAlphaFlagBitsKHR)
pub const COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR: CompositeAlphaFlagsKHR = vks::VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR;

/// See [`VkCompositeAlphaFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCompositeAlphaFlagBitsKHR)
pub const COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR: CompositeAlphaFlagsKHR = vks::VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR;

/// See [`VkCompositeAlphaFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCompositeAlphaFlagBitsKHR)
pub const COMPOSITE_ALPHA_INHERIT_BIT_KHR: CompositeAlphaFlagsKHR = vks::VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR;

/// See [`VkCompositeAlphaFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCompositeAlphaFlagBitsKHR)
pub type VkCompositeAlphaFlagBitsKHR = vks::VkCompositeAlphaFlagBitsKHR;

/// See [`VkColorSpaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorSpaceKHR)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ColorSpaceKHR {
    SRGBNonLinear,
    Unknown(vks::VkColorSpaceKHR),
}

impl From<vks::VkColorSpaceKHR> for ColorSpaceKHR {
    fn from(color_space: vks::VkColorSpaceKHR) -> Self {
        match color_space {
            vks::VK_COLORSPACE_SRGB_NONLINEAR_KHR => ColorSpaceKHR::SRGBNonLinear,
            _ => ColorSpaceKHR::Unknown(color_space),
        }
    }
}

impl From<ColorSpaceKHR> for vks::VkColorSpaceKHR {
    fn from(color_space: ColorSpaceKHR) -> Self {
        match color_space {
            ColorSpaceKHR::SRGBNonLinear => vks::VK_COLORSPACE_SRGB_NONLINEAR_KHR,
            ColorSpaceKHR::Unknown(color_space) => color_space,
        }
    }
}

/// See [`VkPresentModeKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPresentModeKHR)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PresentModeKHR {
    ImmediateKHR,
    MailboxKHR,
    FifoKHR,
    FifoRelaxedKHR,
    Unknown(vks::VkPresentModeKHR),
}

impl From<vks::VkPresentModeKHR> for PresentModeKHR {
    fn from(mode: vks::VkPresentModeKHR) -> Self {
        match mode {
            vks::VK_PRESENT_MODE_IMMEDIATE_KHR => PresentModeKHR::ImmediateKHR,
            vks::VK_PRESENT_MODE_MAILBOX_KHR => PresentModeKHR::MailboxKHR,
            vks::VK_PRESENT_MODE_FIFO_KHR => PresentModeKHR::FifoKHR,
            vks::VK_PRESENT_MODE_FIFO_RELAXED_KHR => PresentModeKHR::FifoRelaxedKHR,
            _ => PresentModeKHR::Unknown(mode),
        }
    }
}

impl From<PresentModeKHR> for vks::VkPresentModeKHR {
    fn from(mode: PresentModeKHR) -> Self {
        match mode {
            PresentModeKHR::ImmediateKHR => vks::VK_PRESENT_MODE_IMMEDIATE_KHR,
            PresentModeKHR::MailboxKHR => vks::VK_PRESENT_MODE_MAILBOX_KHR,
            PresentModeKHR::FifoKHR => vks::VK_PRESENT_MODE_FIFO_KHR,
            PresentModeKHR::FifoRelaxedKHR => vks::VK_PRESENT_MODE_FIFO_RELAXED_KHR,
            PresentModeKHR::Unknown(mode) => mode,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PresentModeKHRIterator(pub(crate) ::std::vec::IntoIter<vks::VkPresentModeKHR>);

impl Iterator for PresentModeKHRIterator {
    type Item = PresentModeKHR;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(From::from)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl DoubleEndedIterator for PresentModeKHRIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(From::from)
    }
}

impl ExactSizeIterator for PresentModeKHRIterator {
    fn len(&self) -> usize {
        self.0.len()
    }
}

/// See [`VkSurfaceCapabilitiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceCapabilitiesKHR)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SurfaceCapabilitiesKHR {
    pub min_image_count: u32,
    pub max_image_count: u32,
    pub current_extent: core::Extent2D,
    pub min_image_extent: core::Extent2D,
    pub max_image_extent: core::Extent2D,
    pub max_image_array_layers: u32,
    pub supported_transforms: SurfaceTransformFlagsKHR,
    pub current_transform: SurfaceTransformFlagBitsKHR,
    pub supported_composite_alpha: CompositeAlphaFlagsKHR,
    pub supported_usage_flags: core::ImageUsageFlags,
}

impl<'a> From<&'a vks::VkSurfaceCapabilitiesKHR> for SurfaceCapabilitiesKHR {
    fn from(capabilities: &'a vks::VkSurfaceCapabilitiesKHR) -> Self {
        SurfaceCapabilitiesKHR {
            min_image_count: capabilities.minImageCount,
            max_image_count: capabilities.maxImageCount,
            current_extent: (&capabilities.currentExtent).into(),
            min_image_extent: (&capabilities.minImageExtent).into(),
            max_image_extent: (&capabilities.maxImageExtent).into(),
            max_image_array_layers: capabilities.maxImageArrayLayers,
            supported_transforms: capabilities.supportedTransforms,
            current_transform: capabilities.currentTransform,
            supported_composite_alpha: capabilities.supportedCompositeAlpha,
            supported_usage_flags: capabilities.supportedUsageFlags,
        }
    }
}

impl<'a> From<&'a SurfaceCapabilitiesKHR> for vks::VkSurfaceCapabilitiesKHR {
    fn from(capabilities: &'a SurfaceCapabilitiesKHR) -> Self {
        vks::VkSurfaceCapabilitiesKHR {
            minImageCount: capabilities.min_image_count,
            maxImageCount: capabilities.max_image_count,
            currentExtent: (&capabilities.current_extent).into(),
            minImageExtent: (&capabilities.min_image_extent).into(),
            maxImageExtent: (&capabilities.max_image_extent).into(),
            maxImageArrayLayers: capabilities.max_image_array_layers,
            supportedTransforms: capabilities.supported_transforms,
            currentTransform: capabilities.current_transform,
            supportedCompositeAlpha: capabilities.supported_composite_alpha,
            supportedUsageFlags: capabilities.supported_usage_flags,
        }
    }
}

/// See [`VkSurfaceFormatKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceFormatKHR)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SurfaceFormatKHR {
    pub format: core::Format,
    pub color_space: ColorSpaceKHR,
}

impl<'a> From<&'a vks::VkSurfaceFormatKHR> for SurfaceFormatKHR {
    fn from(format: &'a vks::VkSurfaceFormatKHR) -> Self {
        SurfaceFormatKHR {
            format: format.format.into(),
            color_space: format.colorSpace.into(),
        }
    }
}

impl<'a> From<&'a SurfaceFormatKHR> for vks::VkSurfaceFormatKHR {
    fn from(format: &'a SurfaceFormatKHR) -> Self {
        vks::VkSurfaceFormatKHR {
            format: format.format.into(),
            colorSpace: format.color_space.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SurfaceFormatKHRIterator(pub(crate) ::std::vec::IntoIter<vks::VkSurfaceFormatKHR>);

impl Iterator for SurfaceFormatKHRIterator {
    type Item = SurfaceFormatKHR;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().as_ref().map(From::from)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl DoubleEndedIterator for SurfaceFormatKHRIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().as_ref().map(From::from)
    }
}

impl ExactSizeIterator for SurfaceFormatKHRIterator {
    fn len(&self) -> usize {
        self.0.len()
    }
}
