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

pub use self::debug_report_callback::{DebugReportCallbackExt, FromNativeDebugReportCallbackExtParameters};

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

    UnknownValue(vks::ext_debug_report::VkDebugReportObjectTypeEXT),
}

impl From<vks::ext_debug_report::VkDebugReportObjectTypeEXT> for DebugReportObjectTypeExt {
    fn from(ty: vks::ext_debug_report::VkDebugReportObjectTypeEXT) -> Self {
        match ty {
            vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT => DebugReportObjectTypeExt::Unknown,
            vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_INSTANCE_EXT => DebugReportObjectTypeExt::Instance,
            vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_PHYSICAL_DEVICE_EXT => DebugReportObjectTypeExt::PhysicalDevice,
            vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_EXT => DebugReportObjectTypeExt::Device,
            vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_QUEUE_EXT => DebugReportObjectTypeExt::Queue,
            vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_SEMAPHORE_EXT => DebugReportObjectTypeExt::Semaphore,
            vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_BUFFER_EXT => DebugReportObjectTypeExt::CommandBuffer,
            vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_FENCE_EXT => DebugReportObjectTypeExt::Fence,
            vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_MEMORY_EXT => DebugReportObjectTypeExt::DeviceMemory,
            vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_EXT => DebugReportObjectTypeExt::Buffer,
            vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_EXT => DebugReportObjectTypeExt::Image,
            vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_EVENT_EXT => DebugReportObjectTypeExt::Event,
            vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_QUERY_POOL_EXT => DebugReportObjectTypeExt::QueryPool,
            vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_VIEW_EXT => DebugReportObjectTypeExt::BufferView,
            vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_VIEW_EXT => DebugReportObjectTypeExt::ImageView,
            vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_SHADER_MODULE_EXT => DebugReportObjectTypeExt::ShaderModule,
            vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_CACHE_EXT => DebugReportObjectTypeExt::PipelineCache,
            vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_LAYOUT_EXT => DebugReportObjectTypeExt::PipelineLayout,
            vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_RENDER_PASS_EXT => DebugReportObjectTypeExt::RenderPass,
            vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_EXT => DebugReportObjectTypeExt::Pipeline,
            vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT_EXT => DebugReportObjectTypeExt::DescriptorSetLayout,
            vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_EXT => DebugReportObjectTypeExt::Sampler,
            vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_POOL_EXT => DebugReportObjectTypeExt::DescriptorPool,
            vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_EXT => DebugReportObjectTypeExt::DescriptorSet,
            vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_FRAMEBUFFER_EXT => DebugReportObjectTypeExt::Framebuffer,
            vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_POOL_EXT => DebugReportObjectTypeExt::CommandPool,
            vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_EXT => DebugReportObjectTypeExt::DebugReport,
            vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_SURFACE_KHR_EXT => DebugReportObjectTypeExt::SurfaceKhr,
            vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_SWAPCHAIN_KHR_EXT => DebugReportObjectTypeExt::SwapchainKhr,
            _ => DebugReportObjectTypeExt::UnknownValue(ty),
        }
    }
}

impl From<DebugReportObjectTypeExt> for vks::ext_debug_report::VkDebugReportObjectTypeEXT {
    fn from(ty: DebugReportObjectTypeExt) -> Self {
        match ty {
            DebugReportObjectTypeExt::Unknown => vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT,
            DebugReportObjectTypeExt::Instance => vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_INSTANCE_EXT,
            DebugReportObjectTypeExt::PhysicalDevice => vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_PHYSICAL_DEVICE_EXT,
            DebugReportObjectTypeExt::Device => vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_EXT,
            DebugReportObjectTypeExt::Queue => vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_QUEUE_EXT,
            DebugReportObjectTypeExt::Semaphore => vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_SEMAPHORE_EXT,
            DebugReportObjectTypeExt::CommandBuffer => vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_BUFFER_EXT,
            DebugReportObjectTypeExt::Fence => vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_FENCE_EXT,
            DebugReportObjectTypeExt::DeviceMemory => vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_MEMORY_EXT,
            DebugReportObjectTypeExt::Buffer => vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_EXT,
            DebugReportObjectTypeExt::Image => vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_EXT,
            DebugReportObjectTypeExt::Event => vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_EVENT_EXT,
            DebugReportObjectTypeExt::QueryPool => vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_QUERY_POOL_EXT,
            DebugReportObjectTypeExt::BufferView => vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_VIEW_EXT,
            DebugReportObjectTypeExt::ImageView => vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_VIEW_EXT,
            DebugReportObjectTypeExt::ShaderModule => vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_SHADER_MODULE_EXT,
            DebugReportObjectTypeExt::PipelineCache => vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_CACHE_EXT,
            DebugReportObjectTypeExt::PipelineLayout => vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_LAYOUT_EXT,
            DebugReportObjectTypeExt::RenderPass => vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_RENDER_PASS_EXT,
            DebugReportObjectTypeExt::Pipeline => vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_EXT,
            DebugReportObjectTypeExt::DescriptorSetLayout => vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT_EXT,
            DebugReportObjectTypeExt::Sampler => vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_EXT,
            DebugReportObjectTypeExt::DescriptorPool => vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_POOL_EXT,
            DebugReportObjectTypeExt::DescriptorSet => vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_EXT,
            DebugReportObjectTypeExt::Framebuffer => vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_FRAMEBUFFER_EXT,
            DebugReportObjectTypeExt::CommandPool => vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_POOL_EXT,
            DebugReportObjectTypeExt::DebugReport => vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_EXT,
            DebugReportObjectTypeExt::SurfaceKhr => vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_SURFACE_KHR_EXT,
            DebugReportObjectTypeExt::SwapchainKhr => vks::ext_debug_report::VK_DEBUG_REPORT_OBJECT_TYPE_SWAPCHAIN_KHR_EXT,
            DebugReportObjectTypeExt::UnknownValue(ty) => ty,
        }
    }
}

dacite_bitflags! {
    /// See [`VkDebugReportFlagBitsEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDebugReportFlagBitsEXT)
    pub struct DebugReportFlagsExt: vks::ext_debug_report::VkDebugReportFlagsEXT;
    pub enum DebugReportFlagBitsExt: vks::ext_debug_report::VkDebugReportFlagBitsEXT;
    max_enum: vks::ext_debug_report::VK_DEBUG_REPORT_FLAG_BITS_MAX_ENUM_EXT;

    flags {
        const INFORMATION [Information] = vks::ext_debug_report::VK_DEBUG_REPORT_INFORMATION_BIT_EXT;
        const WARNING [Warning] = vks::ext_debug_report::VK_DEBUG_REPORT_WARNING_BIT_EXT;
        const PERFORMANCE_WARNING [PerformanceWarning] = vks::ext_debug_report::VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT;
        const ERROR [Error] = vks::ext_debug_report::VK_DEBUG_REPORT_ERROR_BIT_EXT;
        const DEBUG [Debug] = vks::ext_debug_report::VK_DEBUG_REPORT_DEBUG_BIT_EXT;
    }

    no_bits {}
}

/// See [`PFN_vkDebugReportCallbackEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#PFN_vkDebugReportCallbackEXT)
pub trait DebugReportCallbacksExt: Send + Sync + fmt::Debug {
    #[cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
    fn callback(&self, flags: DebugReportFlagsExt, object_type: DebugReportObjectTypeExt, object: u64, location: usize, message_code: i32, layer_prefix: Option<&str>, message: Option<&str>) -> bool;
}

gen_chain_struct! {
    name: DebugReportCallbackCreateInfoChainExt [DebugReportCallbackCreateInfoChainWrapperExt],
    query: DebugReportCallbackCreateInfoChainQueryExt [DebugReportCallbackCreateInfoChainQueryWrapperExt],
    vks: vks::ext_debug_report::VkDebugReportCallbackCreateInfoEXT,
    input: true,
    output: false,
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
    pub vks_struct: vks::ext_debug_report::VkDebugReportCallbackCreateInfoEXT,
    pub callback_helper: CallbackHelper,
    chain: Option<DebugReportCallbackCreateInfoChainWrapperExt>,
}

impl VkDebugReportCallbackCreateInfoEXTWrapper {
    pub fn new(create_info: &DebugReportCallbackCreateInfoExt, with_chain: bool) -> Self {
        let callback_helper = CallbackHelper::new(Arc::clone(&create_info.callback));
        let (pnext, chain) = DebugReportCallbackCreateInfoChainWrapperExt::new_optional(&create_info.chain, with_chain);

        VkDebugReportCallbackCreateInfoEXTWrapper {
            vks_struct: vks::ext_debug_report::VkDebugReportCallbackCreateInfoEXT {
                sType: vks::core::VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT,
                pNext: pnext,
                flags: create_info.flags.bits(),
                pfnCallback: callback_helper.vks_callback,
                pUserData: callback_helper.user_data,
            },
            callback_helper: callback_helper,
            chain: chain,
        }
    }
}
