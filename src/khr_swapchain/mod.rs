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

//! See extension [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)

mod swapchain;

use core;
use khr_surface;
use std::ptr;
use utils;
use vks;

#[cfg(feature = "khr_display_swapchain_9")]
use khr_display_swapchain::{DisplayPresentInfoKhr, VkDisplayPresentInfoKHRWrapper};

pub use self::swapchain::SwapchainKhr;

/// See [`VkSwapchainCreateFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSwapchainCreateFlagBitsKHR)
pub type SwapchainCreateFlagsKhr = vks::VkSwapchainCreateFlagsKHR;

/// See [`VkSwapchainCreateFlagBitsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSwapchainCreateFlagBitsKHR)
pub type SwapchainCreateFlagBitsKhr = vks::VkSwapchainCreateFlagBitsKHR;

/// See [`vkAcquireNextImageKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkAcquireNextImageKHR)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcquireNextImageTimeoutKhr {
    None,
    NanoSeconds(u64),
    Infinite,
}

/// See [`vkAcquireNextImageKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkAcquireNextImageKHR)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcquireNextImageResultKhr {
    Index(usize),
    Timeout,
    NotReady,
    Suboptimal(usize),
}

/// See [`vkQueuePresentKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkQueuePresentKHR)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueuePresentResultKhr {
    Ok,
    Suboptimal,
}

chain_struct! {
    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct SwapchainCreateInfoChainKhr {
    }

    #[derive(Debug)]
    struct SwapchainCreateInfoChainKhrWrapper;
}

/// See [`VkSwapchainCreateInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSwapchainCreateInfoKHR)
#[derive(Debug, Clone, PartialEq)]
pub struct SwapchainCreateInfoKhr {
    pub flags: SwapchainCreateFlagsKhr,
    pub surface: khr_surface::SurfaceKhr,
    pub min_image_count: u32,
    pub image_format: core::Format,
    pub image_color_space: khr_surface::ColorSpaceKhr,
    pub image_extent: core::Extent2D,
    pub image_array_layers: u32,
    pub image_usage: core::ImageUsageFlags,
    pub image_sharing_mode: core::SharingMode,
    pub queue_family_indices: Vec<u32>,
    pub pre_transform: khr_surface::SurfaceTransformFlagBitsKhr,
    pub composite_alpha: khr_surface::CompositeAlphaFlagBitsKhr,
    pub present_mode: khr_surface::PresentModeKhr,
    pub clipped: bool,
    pub old_swapchain: Option<SwapchainKhr>,
    pub chain: Option<SwapchainCreateInfoChainKhr>,
}

#[derive(Debug)]
pub(crate) struct VkSwapchainCreateInfoKHRWrapper {
    pub vks_struct: vks::VkSwapchainCreateInfoKHR,
    surface: khr_surface::SurfaceKhr,
    queue_family_indices: Vec<u32>,
    old_swapchain: Option<SwapchainKhr>,
    chain: Option<SwapchainCreateInfoChainKhrWrapper>,
}

impl VkSwapchainCreateInfoKHRWrapper {
    pub fn new(create_info: &SwapchainCreateInfoKhr, with_chain: bool) -> Self {
        let queue_family_indices = create_info.queue_family_indices.clone();
        let old_swapchain_handle = create_info.old_swapchain.as_ref().map_or(ptr::null_mut(), |s| s.handle());
        let (pnext, chain) = SwapchainCreateInfoChainKhrWrapper::new_optional(&create_info.chain, with_chain);

        VkSwapchainCreateInfoKHRWrapper {
            vks_struct: vks::VkSwapchainCreateInfoKHR {
                sType: vks::VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
                pNext: pnext,
                flags: create_info.flags,
                surface: create_info.surface.handle(),
                minImageCount: create_info.min_image_count,
                imageFormat: create_info.image_format.into(),
                imageColorSpace: create_info.image_color_space.into(),
                imageExtent: (&create_info.image_extent).into(),
                imageArrayLayers: create_info.image_array_layers,
                imageUsage: create_info.image_usage,
                imageSharingMode: create_info.image_sharing_mode.into(),
                queueFamilyIndexCount: queue_family_indices.len() as u32,
                pQueueFamilyIndices: queue_family_indices.as_ptr(),
                preTransform: create_info.pre_transform,
                compositeAlpha: create_info.composite_alpha,
                presentMode: create_info.present_mode.into(),
                clipped: utils::to_vk_bool(create_info.clipped),
                oldSwapchain: old_swapchain_handle,
            },
            surface: create_info.surface.clone(),
            queue_family_indices: queue_family_indices,
            old_swapchain: create_info.old_swapchain.clone(),
            chain: chain,
        }
    }
}

chain_struct! {
    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct PresentInfoChainKhr {
        #[cfg(feature = "khr_display_swapchain_9")]
        field display_present_info_khr: DisplayPresentInfoKhr {
            fn: add_display_present_info_khr,
            stype: vks::VK_STRUCTURE_TYPE_DISPLAY_PRESENT_INFO_KHR,
            wrapper: VkDisplayPresentInfoKHRWrapper,
        },
    }

    #[derive(Debug)]
    struct PresentInfoChainKhrWrapper;
}

/// See [`VkPresentInfoKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPresentInfoKHR)
#[derive(Debug, Clone, PartialEq)]
pub struct PresentInfoKhr {
    pub wait_semaphores: Option<Vec<core::Semaphore>>,
    pub swapchains: Vec<SwapchainKhr>,
    pub image_indices: Vec<u32>,
    pub results: Option<Vec<Result<QueuePresentResultKhr, core::Error>>>,
    pub chain: Option<PresentInfoChainKhr>,
}

#[derive(Debug)]
pub(crate) struct VkPresentInfoKHRWrapper {
    pub vks_struct: vks::VkPresentInfoKHR,
    pub results: Option<Vec<vks::VkResult>>,
    wait_semaphores: Option<Vec<core::Semaphore>>,
    vk_wait_semaphores: Option<Vec<vks::VkSemaphore>>,
    swapchains: Vec<SwapchainKhr>,
    vk_swapchains: Vec<vks::VkSwapchainKHR>,
    image_indices: Vec<u32>,
    chain: Option<PresentInfoChainKhrWrapper>,
}

impl VkPresentInfoKHRWrapper {
    pub fn new(info: &PresentInfoKhr, with_chain: bool) -> Self {
        let (wait_semaphores_count, wait_semaphores_ptr, wait_semaphores, vk_wait_semaphores) = match info.wait_semaphores {
            Some(ref wait_semaphores) => {
                let wait_semaphores = wait_semaphores.clone();
                let vk_wait_semaphores: Vec<_> = wait_semaphores.iter().map(core::Semaphore::handle).collect();
                (wait_semaphores.len() as u32, vk_wait_semaphores.as_ptr(), Some(wait_semaphores), Some(vk_wait_semaphores))
            }

            None => (0, ptr::null(), None, None),
        };

        let swapchains = info.swapchains.clone();
        let vk_swapchains: Vec<_> = swapchains.iter().map(SwapchainKhr::handle).collect();
        let image_indices = info.image_indices.clone();

        let (results_ptr, results) = if info.results.is_some() {
            let mut results = Vec::with_capacity(swapchains.len());
            unsafe { results.set_len(swapchains.len()); }
            (results.as_mut_ptr(), Some(results))
        }
        else {
            (ptr::null_mut(), None)
        };

        let (pnext, chain) = PresentInfoChainKhrWrapper::new_optional(&info.chain, with_chain);

        VkPresentInfoKHRWrapper {
            vks_struct: vks::VkPresentInfoKHR {
                sType: vks::VK_STRUCTURE_TYPE_PRESENT_INFO_KHR,
                pNext: pnext,
                waitSemaphoreCount: wait_semaphores_count,
                pWaitSemaphores: wait_semaphores_ptr,
                swapchainCount: swapchains.len() as u32,
                pSwapchains: vk_swapchains.as_ptr(),
                pImageIndices: image_indices.as_ptr(),
                pResults: results_ptr,
            },
            results: results,
            wait_semaphores: wait_semaphores,
            vk_wait_semaphores: vk_wait_semaphores,
            swapchains: swapchains,
            vk_swapchains: vk_swapchains,
            image_indices: image_indices,
            chain: chain,
        }
    }
}
