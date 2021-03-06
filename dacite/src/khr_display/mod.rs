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

//! See extension [`VK_KHR_display`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display)

mod display;
mod display_mode;

use core;
use khr_surface;
use utils;
use vks;

pub use self::display::DisplayKhr;
pub use self::display_mode::DisplayModeKhr;

dacite_bitflags! {
    /// See [`VkDisplayPlaneAlphaFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayPlaneAlphaFlagBitsKHR)
    pub struct DisplayPlaneAlphaFlagsKhr: vks::khr_display::VkDisplayPlaneAlphaFlagsKHR;
    pub enum DisplayPlaneAlphaFlagBitsKhr: vks::khr_display::VkDisplayPlaneAlphaFlagBitsKHR;
    max_enum: vks::khr_display::VK_DISPLAY_PLANE_ALPHA_FLAG_BITS_MAX_ENUM_KHR;

    flags {
        const OPAQUE [Opaque] = vks::khr_display::VK_DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR;
        const GLOBAL [Global] = vks::khr_display::VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR;
        const PER_PIXEL [PerPixel] = vks::khr_display::VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR;
        const PER_PIXEL_PREMULTIPLIED [PerPixelPremultiplied] = vks::khr_display::VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR;
    }

    no_bits {}
}

dacite_bitflags! {
    /// See [`VkDisplayModeCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayModeCreateFlagsKHR)
    pub struct DisplayModeCreateFlagsKhr: vks::khr_display::VkDisplayModeCreateFlagsKHR;
    pub enum DisplayModeCreateFlagBitsKhr: vks::khr_display::VkDisplayModeCreateFlagBitsKHR;
    max_enum: vks::khr_display::VK_DISPLAY_MODE_CREATE_FLAG_BITS_MAX_ENUM_KHR;

    flags {}
    no_bits {}
}

dacite_bitflags! {
    /// See [`VkDisplaySurfaceCreateFlagsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplaySurfaceCreateFlagsKHR)
    pub struct DisplaySurfaceCreateFlagsKhr: vks::khr_display::VkDisplaySurfaceCreateFlagsKHR;
    pub enum DisplaySurfaceCreateFlagBitsKhr: vks::khr_display::VkDisplaySurfaceCreateFlagBitsKHR;
    max_enum: vks::khr_display::VK_DISPLAY_SURFACE_CREATE_FLAG_BITS_MAX_ENUM_KHR;

    flags {}
    no_bits {}
}

/// See [`VkDisplayPropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayPropertiesKHR)
#[derive(Debug, Clone, PartialEq)]
pub struct DisplayPropertiesKhr {
    pub display: DisplayKhr,
    pub display_name: Option<String>,
    pub physical_dimensions: core::Extent2D,
    pub physical_resolution: core::Extent2D,
    pub supported_transforms: khr_surface::SurfaceTransformFlagsKhr,
    pub plane_reorder_possible: bool,
    pub persistent_content: bool,
}

impl DisplayPropertiesKhr {
    pub(crate) unsafe fn from_vks(properties: &vks::khr_display::VkDisplayPropertiesKHR, physical_device: core::PhysicalDevice) -> Self {
        DisplayPropertiesKhr {
            display: DisplayKhr::new(properties.display, physical_device),
            display_name: utils::string_from_cstr(properties.displayName),
            physical_dimensions: (&properties.physicalDimensions).into(),
            physical_resolution: (&properties.physicalResolution).into(),
            supported_transforms: khr_surface::SurfaceTransformFlagsKhr::from_bits_truncate(properties.supportedTransforms),
            plane_reorder_possible: utils::from_vk_bool(properties.planeReorderPossible),
            persistent_content: utils::from_vk_bool(properties.persistentContent),
        }
    }
}

/// See [`VkDisplayModeParametersKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayModeParametersKHR)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DisplayModeParametersKhr {
    pub visible_region: core::Extent2D,
    pub refresh_rate: u32,
}

impl<'a> From<&'a vks::khr_display::VkDisplayModeParametersKHR> for DisplayModeParametersKhr {
    fn from(parameters: &'a vks::khr_display::VkDisplayModeParametersKHR) -> Self {
        DisplayModeParametersKhr {
            visible_region: (&parameters.visibleRegion).into(),
            refresh_rate: parameters.refreshRate,
        }
    }
}

impl<'a> From<&'a DisplayModeParametersKhr> for vks::khr_display::VkDisplayModeParametersKHR {
    fn from(parameters: &'a DisplayModeParametersKhr) -> Self {
        vks::khr_display::VkDisplayModeParametersKHR {
            visibleRegion: (&parameters.visible_region).into(),
            refreshRate: parameters.refresh_rate,
        }
    }
}

/// See [`VkDisplayModePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayModePropertiesKHR)
#[derive(Debug, Clone, PartialEq)]
pub struct DisplayModePropertiesKhr {
    pub display_mode: DisplayModeKhr,
    pub parameters: DisplayModeParametersKhr,
}

impl DisplayModePropertiesKhr {
    pub(crate) fn from_vks(properties: &vks::khr_display::VkDisplayModePropertiesKHR, display: DisplayKhr) -> Self {
        DisplayModePropertiesKhr {
            display_mode: DisplayModeKhr::new(properties.displayMode, display),
            parameters: (&properties.parameters).into(),
        }
    }
}

gen_chain_struct! {
    name: DisplayModeCreateInfoChainKhr [DisplayModeCreateInfoChainKhrWrapper],
    query: DisplayModeCreateInfoChainQueryKhr [DisplayModeCreateInfoChainQueryKhrWrapper],
    vks: vks::khr_display::VkDisplayModeCreateInfoKHR,
    input: true,
    output: false,
}

/// See [`VkDisplayModeCreateInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayModeCreateInfoKHR)
#[derive(Debug, Clone, PartialEq)]
pub struct DisplayModeCreateInfoKhr {
    pub flags: DisplayModeCreateFlagsKhr,
    pub parameters: DisplayModeParametersKhr,
    pub chain: Option<DisplayModeCreateInfoChainKhr>,
}

#[derive(Debug)]
struct VkDisplayModeCreateInfoKHRWrapper {
    pub vks_struct: vks::khr_display::VkDisplayModeCreateInfoKHR,
    chain: Option<DisplayModeCreateInfoChainKhrWrapper>,
}

impl VkDisplayModeCreateInfoKHRWrapper {
    fn new(create_info: &DisplayModeCreateInfoKhr, with_chain: bool) -> Self {
        let (pnext, chain) = DisplayModeCreateInfoChainKhrWrapper::new_optional(&create_info.chain, with_chain);

        VkDisplayModeCreateInfoKHRWrapper {
            vks_struct: vks::khr_display::VkDisplayModeCreateInfoKHR {
                sType: vks::vk::VK_STRUCTURE_TYPE_DISPLAY_MODE_CREATE_INFO_KHR,
                pNext: pnext,
                flags: create_info.flags.bits(),
                parameters: (&create_info.parameters).into(),
            },
            chain: chain,
        }
    }
}

/// See [`VkDisplayPlaneCapabilitiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayPlaneCapabilitiesKHR)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DisplayPlaneCapabilitiesKhr {
    pub supported_alpha: DisplayPlaneAlphaFlagsKhr,
    pub min_src_position: core::Offset2D,
    pub max_src_position: core::Offset2D,
    pub min_src_extent: core::Extent2D,
    pub max_src_extent: core::Extent2D,
    pub min_dst_position: core::Offset2D,
    pub max_dst_position: core::Offset2D,
    pub min_dst_extent: core::Extent2D,
    pub max_dst_extent: core::Extent2D,
}

impl<'a> From<&'a vks::khr_display::VkDisplayPlaneCapabilitiesKHR> for DisplayPlaneCapabilitiesKhr {
    fn from(capabilities: &'a vks::khr_display::VkDisplayPlaneCapabilitiesKHR) -> Self {
        DisplayPlaneCapabilitiesKhr {
            supported_alpha: DisplayPlaneAlphaFlagsKhr::from_bits_truncate(capabilities.supportedAlpha),
            min_src_position: (&capabilities.minSrcPosition).into(),
            max_src_position: (&capabilities.maxSrcPosition).into(),
            min_src_extent: (&capabilities.minSrcExtent).into(),
            max_src_extent: (&capabilities.maxSrcExtent).into(),
            min_dst_position: (&capabilities.minDstPosition).into(),
            max_dst_position: (&capabilities.maxDstPosition).into(),
            min_dst_extent: (&capabilities.minDstExtent).into(),
            max_dst_extent: (&capabilities.maxDstExtent).into(),
        }
    }
}

/// See [`VkDisplayPlanePropertiesKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplayPlanePropertiesKHR)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisplayPlanePropertiesKhr {
    pub current_display: Option<DisplayKhr>,
    pub current_stack_index: u32,
}

impl DisplayPlanePropertiesKhr {
    pub(crate) unsafe fn from_vks(properties: &vks::khr_display::VkDisplayPlanePropertiesKHR, physical_device: &core::PhysicalDevice) -> Self {
        let current_display = if properties.currentDisplay != 0 {
            Some(DisplayKhr::new(properties.currentDisplay, physical_device.clone()))
        }
        else {
            None
        };

        DisplayPlanePropertiesKhr {
            current_display: current_display,
            current_stack_index: properties.currentStackIndex,
        }
    }
}

gen_chain_struct! {
    name: DisplaySurfaceCreateInfoChainKhr [DisplaySurfaceCreateInfoChainKhrWrapper],
    query: DisplaySurfaceCreateInfoChainQueryKhr [DisplaySurfaceCreateInfoChainQueryKhrWrapper],
    vks: vks::khr_display::VkDisplaySurfaceCreateInfoKHR,
    input: true,
    output: false,
}

/// See [`VkDisplaySurfaceCreateInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDisplaySurfaceCreateInfoKHR)
#[derive(Debug, Clone, PartialEq)]
pub struct DisplaySurfaceCreateInfoKhr {
    pub flags: DisplaySurfaceCreateFlagsKhr,
    pub display_mode: DisplayModeKhr,
    pub plane_index: u32,
    pub plane_stack_index: u32,
    pub transform: khr_surface::SurfaceTransformFlagBitsKhr,
    pub global_alpha: f32,
    pub alpha_mode: DisplayPlaneAlphaFlagBitsKhr,
    pub image_extent: core::Extent2D,
    pub chain: Option<DisplaySurfaceCreateInfoChainKhr>,
}

#[derive(Debug)]
pub(crate) struct VkDisplaySurfaceCreateInfoKHRWrapper {
    pub vks_struct: vks::khr_display::VkDisplaySurfaceCreateInfoKHR,
    chain: Option<DisplaySurfaceCreateInfoChainKhrWrapper>,
}

impl VkDisplaySurfaceCreateInfoKHRWrapper {
    pub fn new(create_info: &DisplaySurfaceCreateInfoKhr, with_chain: bool) -> Self {
        let (pnext, chain) = DisplaySurfaceCreateInfoChainKhrWrapper::new_optional(&create_info.chain, with_chain);

        VkDisplaySurfaceCreateInfoKHRWrapper {
            vks_struct: vks::khr_display::VkDisplaySurfaceCreateInfoKHR {
                sType: vks::vk::VK_STRUCTURE_TYPE_DISPLAY_SURFACE_CREATE_INFO_KHR,
                pNext: pnext,
                flags: create_info.flags.bits(),
                displayMode: create_info.display_mode.handle,
                planeIndex: create_info.plane_index,
                planeStackIndex: create_info.plane_stack_index,
                transform: create_info.transform.bit(),
                globalAlpha: create_info.global_alpha,
                alphaMode: create_info.alpha_mode.bit(),
                imageExtent: (&create_info.image_extent).into(),
            },
            chain: chain,
        }
    }
}
