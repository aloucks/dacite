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

pub use self::surface::{SurfaceKhr, FromNativeSurfaceKhrParameters};

/// See [`VkColorSpaceKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorSpaceKHR)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ColorSpaceKhr {
    SRGBNonLinear,
    Unknown(vks::khr_surface::VkColorSpaceKHR),
}

impl From<vks::khr_surface::VkColorSpaceKHR> for ColorSpaceKhr {
    fn from(color_space: vks::khr_surface::VkColorSpaceKHR) -> Self {
        match color_space {
            vks::khr_surface::VK_COLORSPACE_SRGB_NONLINEAR_KHR => ColorSpaceKhr::SRGBNonLinear,
            _ => ColorSpaceKhr::Unknown(color_space),
        }
    }
}

impl From<ColorSpaceKhr> for vks::khr_surface::VkColorSpaceKHR {
    fn from(color_space: ColorSpaceKhr) -> Self {
        match color_space {
            ColorSpaceKhr::SRGBNonLinear => vks::khr_surface::VK_COLORSPACE_SRGB_NONLINEAR_KHR,
            ColorSpaceKhr::Unknown(color_space) => color_space,
        }
    }
}

/// See [`VkPresentModeKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPresentModeKHR)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PresentModeKhr {
    Immediate,
    Mailbox,
    Fifo,
    FifoRelaxed,
    Unknown(vks::khr_surface::VkPresentModeKHR),
}

impl From<vks::khr_surface::VkPresentModeKHR> for PresentModeKhr {
    fn from(mode: vks::khr_surface::VkPresentModeKHR) -> Self {
        match mode {
            vks::khr_surface::VK_PRESENT_MODE_IMMEDIATE_KHR => PresentModeKhr::Immediate,
            vks::khr_surface::VK_PRESENT_MODE_MAILBOX_KHR => PresentModeKhr::Mailbox,
            vks::khr_surface::VK_PRESENT_MODE_FIFO_KHR => PresentModeKhr::Fifo,
            vks::khr_surface::VK_PRESENT_MODE_FIFO_RELAXED_KHR => PresentModeKhr::FifoRelaxed,
            _ => PresentModeKhr::Unknown(mode),
        }
    }
}

impl From<PresentModeKhr> for vks::khr_surface::VkPresentModeKHR {
    fn from(mode: PresentModeKhr) -> Self {
        match mode {
            PresentModeKhr::Immediate => vks::khr_surface::VK_PRESENT_MODE_IMMEDIATE_KHR,
            PresentModeKhr::Mailbox => vks::khr_surface::VK_PRESENT_MODE_MAILBOX_KHR,
            PresentModeKhr::Fifo => vks::khr_surface::VK_PRESENT_MODE_FIFO_KHR,
            PresentModeKhr::FifoRelaxed => vks::khr_surface::VK_PRESENT_MODE_FIFO_RELAXED_KHR,
            PresentModeKhr::Unknown(mode) => mode,
        }
    }
}

dacite_bitflags! {
    /// See [`VkSurfaceTransformFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceTransformFlagBitsKHR)
    pub struct SurfaceTransformFlagsKhr: vks::khr_surface::VkSurfaceTransformFlagsKHR;
    pub enum SurfaceTransformFlagBitsKhr: vks::khr_surface::VkSurfaceTransformFlagBitsKHR;
    max_enum: vks::khr_surface::VK_SURFACE_TRANSFORM_FLAG_BITS_MAX_ENUM_KHR;

    flags {
        const IDENTITY [Identity] = vks::khr_surface::VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR;
        const ROTATE_90 [Rotate90] = vks::khr_surface::VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR;
        const ROTATE_180 [Rotate180] = vks::khr_surface::VK_SURFACE_TRANSFORM_ROTATE_180_BIT_KHR;
        const ROTATE_270 [Rotate270] = vks::khr_surface::VK_SURFACE_TRANSFORM_ROTATE_270_BIT_KHR;
        const HORIZONTAL_MIRROR [HorizontalMirror] = vks::khr_surface::VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR;
        const HORIZONTAL_MIRROR_ROTATE_90 [HorizontalMirrorRotate90] = vks::khr_surface::VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR;
        const HORIZONTAL_MIRROR_ROTATE_180 [HorizontalMirrorRotate180] = vks::khr_surface::VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR;
        const HORIZONTAL_MIRROR_ROTATE_270 [HorizontalMirrorRotate270] = vks::khr_surface::VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR;
        const INHERIT [Inherit] = vks::khr_surface::VK_SURFACE_TRANSFORM_INHERIT_BIT_KHR;
    }

    no_bits {}
}

dacite_bitflags! {
    /// See [`VkCompositeAlphaFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCompositeAlphaFlagBitsKHR)
    pub struct CompositeAlphaFlagsKhr: vks::khr_surface::VkCompositeAlphaFlagsKHR;
    pub enum CompositeAlphaFlagBitsKhr: vks::khr_surface::VkCompositeAlphaFlagBitsKHR;
    max_enum: vks::khr_surface::VK_COMPOSITE_ALPHA_FLAG_BITS_MAX_ENUM_KHR;

    flags {
        const OPAQUE [Opaque] = vks::khr_surface::VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR;
        const PRE_MULTIPLIED [PreMultiplied] = vks::khr_surface::VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR;
        const POST_MULTIPLIED [PostMultiplied] = vks::khr_surface::VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR;
        const INHERIT [Inherit] = vks::khr_surface::VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR;
    }

    no_bits {}
}

/// See [`VkSurfaceCapabilitiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceCapabilitiesKHR)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SurfaceCapabilitiesKhr {
    pub min_image_count: u32,
    pub max_image_count: Option<u32>,
    pub current_extent: Option<core::Extent2D>,
    pub min_image_extent: core::Extent2D,
    pub max_image_extent: core::Extent2D,
    pub max_image_array_layers: u32,
    pub supported_transforms: SurfaceTransformFlagsKhr,
    pub current_transform: SurfaceTransformFlagBitsKhr,
    pub supported_composite_alpha: CompositeAlphaFlagsKhr,
    pub supported_usage_flags: core::ImageUsageFlags,
}

impl<'a> From<&'a vks::khr_surface::VkSurfaceCapabilitiesKHR> for SurfaceCapabilitiesKhr {
    fn from(capabilities: &'a vks::khr_surface::VkSurfaceCapabilitiesKHR) -> Self {
        let max_image_count = if capabilities.maxImageCount > 0 {
            Some(capabilities.maxImageCount)
        }
        else {
            None
        };

        let current_extent = if (capabilities.currentExtent.width != ::std::u32::MAX) || (capabilities.currentExtent.height != ::std::u32::MAX) {
            Some((&capabilities.currentExtent).into())
        }
        else {
            None
        };

        SurfaceCapabilitiesKhr {
            min_image_count: capabilities.minImageCount,
            max_image_count: max_image_count,
            current_extent: current_extent,
            min_image_extent: (&capabilities.minImageExtent).into(),
            max_image_extent: (&capabilities.maxImageExtent).into(),
            max_image_array_layers: capabilities.maxImageArrayLayers,
            supported_transforms: SurfaceTransformFlagsKhr::from_bits_truncate(capabilities.supportedTransforms),
            current_transform: SurfaceTransformFlagBitsKhr::from_bits(capabilities.currentTransform).unwrap(),
            supported_composite_alpha: CompositeAlphaFlagsKhr::from_bits_truncate(capabilities.supportedCompositeAlpha),
            supported_usage_flags: core::ImageUsageFlags::from_bits_truncate(capabilities.supportedUsageFlags),
        }
    }
}

/// See [`VkSurfaceFormatKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSurfaceFormatKHR)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SurfaceFormatKhr {
    pub format: core::Format,
    pub color_space: ColorSpaceKhr,
}

impl<'a> From<&'a vks::khr_surface::VkSurfaceFormatKHR> for SurfaceFormatKhr {
    fn from(format: &'a vks::khr_surface::VkSurfaceFormatKHR) -> Self {
        SurfaceFormatKhr {
            format: format.format.into(),
            color_space: format.colorSpace.into(),
        }
    }
}
