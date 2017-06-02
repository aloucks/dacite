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

//! See extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)

mod callback_helper;
mod debug_report_callback;

use self::callback_helper::CallbackHelper;
use std::fmt;
use std::sync::Arc;
use vks;

pub use self::debug_report_callback::DebugReportCallbackExt;

/// See [`VkDebugReportObjectTypeEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportObjectTypeEXT)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DebugReportObjectTypeExt {
    Unknown,
    Instance,
    PhysicalDevice,
    Device,
    Queue,
    Semaphore,
    CommandBuffer,
    Fence,
    DeviceMemory,
    Buffer,
    Image,
    Event,
    QueryPool,
    BufferView,
    ImageView,
    ShaderModule,
    PipelineCache,
    PipelineLayout,
    RenderPass,
    Pipeline,
    DescriptorSetLayout,
    Sampler,
    DescriptorPool,
    DescriptorSet,
    Framebuffer,
    CommandPool,
    DebugReport,

    /// See extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
    SurfaceKhr,

    /// See extension [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)
    SwapchainKhr,

    UnknownValue(vks::VkDebugReportObjectTypeEXT),
}

impl From<vks::VkDebugReportObjectTypeEXT> for DebugReportObjectTypeExt {
    fn from(ty: vks::VkDebugReportObjectTypeEXT) -> Self {
        match ty {
            vks::VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT => DebugReportObjectTypeExt::Unknown,
            vks::VK_DEBUG_REPORT_OBJECT_TYPE_INSTANCE_EXT => DebugReportObjectTypeExt::Instance,
            vks::VK_DEBUG_REPORT_OBJECT_TYPE_PHYSICAL_DEVICE_EXT => DebugReportObjectTypeExt::PhysicalDevice,
            vks::VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_EXT => DebugReportObjectTypeExt::Device,
            vks::VK_DEBUG_REPORT_OBJECT_TYPE_QUEUE_EXT => DebugReportObjectTypeExt::Queue,
            vks::VK_DEBUG_REPORT_OBJECT_TYPE_SEMAPHORE_EXT => DebugReportObjectTypeExt::Semaphore,
            vks::VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_BUFFER_EXT => DebugReportObjectTypeExt::CommandBuffer,
            vks::VK_DEBUG_REPORT_OBJECT_TYPE_FENCE_EXT => DebugReportObjectTypeExt::Fence,
            vks::VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_MEMORY_EXT => DebugReportObjectTypeExt::DeviceMemory,
            vks::VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_EXT => DebugReportObjectTypeExt::Buffer,
            vks::VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_EXT => DebugReportObjectTypeExt::Image,
            vks::VK_DEBUG_REPORT_OBJECT_TYPE_EVENT_EXT => DebugReportObjectTypeExt::Event,
            vks::VK_DEBUG_REPORT_OBJECT_TYPE_QUERY_POOL_EXT => DebugReportObjectTypeExt::QueryPool,
            vks::VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_VIEW_EXT => DebugReportObjectTypeExt::BufferView,
            vks::VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_VIEW_EXT => DebugReportObjectTypeExt::ImageView,
            vks::VK_DEBUG_REPORT_OBJECT_TYPE_SHADER_MODULE_EXT => DebugReportObjectTypeExt::ShaderModule,
            vks::VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_CACHE_EXT => DebugReportObjectTypeExt::PipelineCache,
            vks::VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_LAYOUT_EXT => DebugReportObjectTypeExt::PipelineLayout,
            vks::VK_DEBUG_REPORT_OBJECT_TYPE_RENDER_PASS_EXT => DebugReportObjectTypeExt::RenderPass,
            vks::VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_EXT => DebugReportObjectTypeExt::Pipeline,
            vks::VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT_EXT => DebugReportObjectTypeExt::DescriptorSetLayout,
            vks::VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_EXT => DebugReportObjectTypeExt::Sampler,
            vks::VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_POOL_EXT => DebugReportObjectTypeExt::DescriptorPool,
            vks::VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_EXT => DebugReportObjectTypeExt::DescriptorSet,
            vks::VK_DEBUG_REPORT_OBJECT_TYPE_FRAMEBUFFER_EXT => DebugReportObjectTypeExt::Framebuffer,
            vks::VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_POOL_EXT => DebugReportObjectTypeExt::CommandPool,
            vks::VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_EXT => DebugReportObjectTypeExt::DebugReport,
            vks::VK_DEBUG_REPORT_OBJECT_TYPE_SURFACE_KHR_EXT => DebugReportObjectTypeExt::SurfaceKhr,
            vks::VK_DEBUG_REPORT_OBJECT_TYPE_SWAPCHAIN_KHR_EXT => DebugReportObjectTypeExt::SwapchainKhr,
            _ => DebugReportObjectTypeExt::UnknownValue(ty),
        }
    }
}

impl From<DebugReportObjectTypeExt> for vks::VkDebugReportObjectTypeEXT {
    fn from(ty: DebugReportObjectTypeExt) -> Self {
        match ty {
            DebugReportObjectTypeExt::Unknown => vks::VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT,
            DebugReportObjectTypeExt::Instance => vks::VK_DEBUG_REPORT_OBJECT_TYPE_INSTANCE_EXT,
            DebugReportObjectTypeExt::PhysicalDevice => vks::VK_DEBUG_REPORT_OBJECT_TYPE_PHYSICAL_DEVICE_EXT,
            DebugReportObjectTypeExt::Device => vks::VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_EXT,
            DebugReportObjectTypeExt::Queue => vks::VK_DEBUG_REPORT_OBJECT_TYPE_QUEUE_EXT,
            DebugReportObjectTypeExt::Semaphore => vks::VK_DEBUG_REPORT_OBJECT_TYPE_SEMAPHORE_EXT,
            DebugReportObjectTypeExt::CommandBuffer => vks::VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_BUFFER_EXT,
            DebugReportObjectTypeExt::Fence => vks::VK_DEBUG_REPORT_OBJECT_TYPE_FENCE_EXT,
            DebugReportObjectTypeExt::DeviceMemory => vks::VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_MEMORY_EXT,
            DebugReportObjectTypeExt::Buffer => vks::VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_EXT,
            DebugReportObjectTypeExt::Image => vks::VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_EXT,
            DebugReportObjectTypeExt::Event => vks::VK_DEBUG_REPORT_OBJECT_TYPE_EVENT_EXT,
            DebugReportObjectTypeExt::QueryPool => vks::VK_DEBUG_REPORT_OBJECT_TYPE_QUERY_POOL_EXT,
            DebugReportObjectTypeExt::BufferView => vks::VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_VIEW_EXT,
            DebugReportObjectTypeExt::ImageView => vks::VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_VIEW_EXT,
            DebugReportObjectTypeExt::ShaderModule => vks::VK_DEBUG_REPORT_OBJECT_TYPE_SHADER_MODULE_EXT,
            DebugReportObjectTypeExt::PipelineCache => vks::VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_CACHE_EXT,
            DebugReportObjectTypeExt::PipelineLayout => vks::VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_LAYOUT_EXT,
            DebugReportObjectTypeExt::RenderPass => vks::VK_DEBUG_REPORT_OBJECT_TYPE_RENDER_PASS_EXT,
            DebugReportObjectTypeExt::Pipeline => vks::VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_EXT,
            DebugReportObjectTypeExt::DescriptorSetLayout => vks::VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT_EXT,
            DebugReportObjectTypeExt::Sampler => vks::VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_EXT,
            DebugReportObjectTypeExt::DescriptorPool => vks::VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_POOL_EXT,
            DebugReportObjectTypeExt::DescriptorSet => vks::VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_EXT,
            DebugReportObjectTypeExt::Framebuffer => vks::VK_DEBUG_REPORT_OBJECT_TYPE_FRAMEBUFFER_EXT,
            DebugReportObjectTypeExt::CommandPool => vks::VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_POOL_EXT,
            DebugReportObjectTypeExt::DebugReport => vks::VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_EXT,
            DebugReportObjectTypeExt::SurfaceKhr => vks::VK_DEBUG_REPORT_OBJECT_TYPE_SURFACE_KHR_EXT,
            DebugReportObjectTypeExt::SwapchainKhr => vks::VK_DEBUG_REPORT_OBJECT_TYPE_SWAPCHAIN_KHR_EXT,
            DebugReportObjectTypeExt::UnknownValue(ty) => ty,
        }
    }
}

/// See [`VkDebugReportFlagBitsEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportFlagBitsEXT)
pub type DebugReportFlagsExt = vks::VkDebugReportFlagsEXT;

/// See [`VkDebugReportFlagBitsEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportFlagBitsEXT)
pub type DebugReportFlagBitsExt = vks::VkDebugReportFlagBitsEXT;

/// See [`VkDebugReportFlagBitsEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportFlagBitsEXT)
pub const DEBUG_REPORT_INFORMATION_BIT_EXT: DebugReportFlagBitsExt = vks::VK_DEBUG_REPORT_INFORMATION_BIT_EXT;

/// See [`VkDebugReportFlagBitsEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportFlagBitsEXT)
pub const DEBUG_REPORT_WARNING_BIT_EXT: DebugReportFlagBitsExt = vks::VK_DEBUG_REPORT_WARNING_BIT_EXT;

/// See [`VkDebugReportFlagBitsEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportFlagBitsEXT)
pub const DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT: DebugReportFlagBitsExt = vks::VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT;

/// See [`VkDebugReportFlagBitsEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportFlagBitsEXT)
pub const DEBUG_REPORT_ERROR_BIT_EXT: DebugReportFlagBitsExt = vks::VK_DEBUG_REPORT_ERROR_BIT_EXT;

/// See [`VkDebugReportFlagBitsEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportFlagBitsEXT)
pub const DEBUG_REPORT_DEBUG_BIT_EXT: DebugReportFlagBitsExt = vks::VK_DEBUG_REPORT_DEBUG_BIT_EXT;

/// See [`PFN_vkDebugReportCallbackEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#PFN_vkDebugReportCallbackEXT)
pub trait DebugReportCallbacksExt: Send + Sync + fmt::Debug {
    fn callback(&self, flags: DebugReportFlagsExt, object_type: DebugReportObjectTypeExt, object: u64, location: usize, message_code: i32, layer_prefix: Option<&str>, message: Option<&str>) -> bool;
}

chain_struct! {
    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct DebugReportCallbackCreateInfoChainExt {
    }

    #[derive(Debug)]
    struct DebugReportCallbackCreateInfoChainWrapperExt;
}

/// See [`VkDebugReportCallbackCreateInfoEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportCallbackCreateInfoEXT)
#[derive(Debug, Clone)]
pub struct DebugReportCallbackCreateInfoExt {
    pub flags: DebugReportFlagsExt,
    pub callback: Arc<DebugReportCallbacksExt>,
    pub chain: Option<DebugReportCallbackCreateInfoChainExt>,
}

impl PartialEq for DebugReportCallbackCreateInfoExt {
    fn eq(&self, other: &Self) -> bool {
        (self.flags == other.flags) &&
        Arc::ptr_eq(&self.callback, &other.callback) &&
        (self.chain == other.chain)
    }
}

#[derive(Debug)]
pub(crate) struct VkDebugReportCallbackCreateInfoEXTWrapper {
    pub vks_struct: vks::VkDebugReportCallbackCreateInfoEXT,
    pub callback_helper: CallbackHelper,
    chain: Option<DebugReportCallbackCreateInfoChainWrapperExt>,
}

impl VkDebugReportCallbackCreateInfoEXTWrapper {
    pub fn new(create_info: &DebugReportCallbackCreateInfoExt, with_chain: bool) -> Self {
        let callback_helper = CallbackHelper::new(create_info.callback.clone());
        let (pnext, chain) = DebugReportCallbackCreateInfoChainWrapperExt::new_optional(&create_info.chain, with_chain);

        VkDebugReportCallbackCreateInfoEXTWrapper {
            vks_struct: vks::VkDebugReportCallbackCreateInfoEXT {
                sType: vks::VK_STRUCTURE_TYPE_DEBUG_REPORT_CREATE_INFO_EXT,
                pNext: pnext,
                flags: create_info.flags,
                pfnCallback: callback_helper.vks_callback,
                pUserData: callback_helper.user_data,
            },
            callback_helper: callback_helper,
            chain: chain,
        }
    }
}
