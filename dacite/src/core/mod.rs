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

#[macro_use]
mod extension_structs;

mod buffer;
mod buffer_view;
mod command_buffer;
mod command_pool;
mod descriptor_pool;
mod descriptor_set;
mod descriptor_set_layout;
mod device;
mod device_memory;
mod event;
mod fence;
mod framebuffer;
mod image;
mod image_view;
mod instance;
mod physical_device;
mod pipeline;
mod pipeline_cache;
mod pipeline_layout;
mod query_pool;
mod queue;
mod render_pass;
mod sampler;
mod semaphore;
mod shader_module;

pub(crate) mod allocator_helper;

use libc::{c_char, c_void};
use std::cmp;
use std::collections::{HashMap, HashSet};
use std::ffi::{CStr, CString};
use std::fmt;
use std::mem;
use std::ptr;
use std::slice;
use std::time::Duration;
use utils;
use vks;

pub use self::buffer::{Buffer, FromNativeBufferParameters};
pub use self::buffer_view::{BufferView, FromNativeBufferViewParameters};
pub use self::command_buffer::{CommandBuffer, FromNativeCommandBufferParameters};
pub use self::command_pool::{CommandPool, FromNativeCommandPoolParameters};
pub use self::descriptor_pool::{DescriptorPool, FromNativeDescriptorPoolParameters};
pub use self::descriptor_set::DescriptorSet;
pub use self::descriptor_set_layout::{DescriptorSetLayout, FromNativeDescriptorSetLayoutParameters};
pub use self::device::Device;
pub use self::device_memory::{DeviceMemory, MappedMemory, FromNativeDeviceMemoryParameters};
pub use self::event::{Event, FromNativeEventParameters};
pub use self::fence::{Fence, FromNativeFenceParameters};
pub use self::framebuffer::{Framebuffer, FromNativeFramebufferParameters};
pub use self::image::{Image, FromNativeImageParameters};
pub use self::image_view::{ImageView, FromNativeImageViewParameters};
pub use self::instance::{EarlyInstanceError, Instance};
pub use self::physical_device::PhysicalDevice;
pub use self::pipeline::{Pipeline, FromNativePipelineParameters};
pub use self::pipeline_cache::{PipelineCache, FromNativePipelineCacheParameters};
pub use self::pipeline_layout::{PipelineLayout, FromNativePipelineLayoutParameters};
pub use self::query_pool::{QueryPool, FromNativeQueryPoolParameters};
pub use self::queue::Queue;
pub use self::render_pass::{RenderPass, FromNativeRenderPassParameters};
pub use self::sampler::{Sampler, FromNativeSamplerParameters};
pub use self::semaphore::{Semaphore, FromNativeSemaphoreParameters};
pub use self::shader_module::{ShaderModule, FromNativeShaderModuleParameters};

bitflags! {
    /// See [`VkInstanceCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkInstanceCreateFlags)
    #[derive(Default)]
    pub struct InstanceCreateFlags: u32 {
        /// See [`VkInstanceCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkInstanceCreateFlags)
        const INSTANCE_CREATE_FLAG_BITS_MAX_ENUM = vks::VK_INSTANCE_CREATE_FLAG_BITS_MAX_ENUM;
    }
}

/// See [`VkInstanceCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkInstanceCreateFlags)
pub type InstanceCreateFlagBits = InstanceCreateFlags;

bitflags! {
    /// See [`VkFormatFeatureFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlagBits)
    #[derive(Default)]
    pub struct FormatFeatureFlags: u32 {
        /// See [`VkFormatFeatureFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlagBits)
        const FORMAT_FEATURE_FLAG_BITS_MAX_ENUM = vks::VK_FORMAT_FEATURE_FLAG_BITS_MAX_ENUM;

        /// See [`VkFormatFeatureFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlagBits)
        const FORMAT_FEATURE_SAMPLED_IMAGE_BIT = vks::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT;

        /// See [`VkFormatFeatureFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlagBits)
        const FORMAT_FEATURE_STORAGE_IMAGE_BIT = vks::VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT;

        /// See [`VkFormatFeatureFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlagBits)
        const FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT = vks::VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT;

        /// See [`VkFormatFeatureFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlagBits)
        const FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT = vks::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT;

        /// See [`VkFormatFeatureFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlagBits)
        const FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT = vks::VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT;

        /// See [`VkFormatFeatureFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlagBits)
        const FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_ATOMIC_BIT = vks::VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_ATOMIC_BIT;

        /// See [`VkFormatFeatureFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlagBits)
        const FORMAT_FEATURE_VERTEX_BUFFER_BIT = vks::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT;

        /// See [`VkFormatFeatureFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlagBits)
        const FORMAT_FEATURE_COLOR_ATTACHMENT_BIT = vks::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT;

        /// See [`VkFormatFeatureFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlagBits)
        const FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT = vks::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT;

        /// See [`VkFormatFeatureFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlagBits)
        const FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT = vks::VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT;

        /// See [`VkFormatFeatureFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlagBits)
        const FORMAT_FEATURE_BLIT_SRC_BIT = vks::VK_FORMAT_FEATURE_BLIT_SRC_BIT;

        /// See [`VkFormatFeatureFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlagBits)
        const FORMAT_FEATURE_BLIT_DST_BIT = vks::VK_FORMAT_FEATURE_BLIT_DST_BIT;

        /// See [`VkFormatFeatureFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlagBits)
        const FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT = vks::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT;

        /// See [`VkFormatFeatureFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlagBits)
        /// and extension [`VK_IMG_filter_cubic`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_IMG_filter_cubic)
        const FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_IMG = vks::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_IMG;
    }
}

/// See [`VkFormatFeatureFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlagBits)
pub type FormatFeatureFlagBits = FormatFeatureFlags;

bitflags! {
    /// See [`VkImageUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageUsageFlagBits)
    #[derive(Default)]
    pub struct ImageUsageFlags: u32 {
        /// See [`VkImageUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageUsageFlagBits)
        const IMAGE_USAGE_FLAG_BITS_MAX_ENUM = vks::VK_IMAGE_USAGE_FLAG_BITS_MAX_ENUM;

        /// See [`VkImageUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageUsageFlagBits)
        const IMAGE_USAGE_TRANSFER_SRC_BIT = vks::VK_IMAGE_USAGE_TRANSFER_SRC_BIT;

        /// See [`VkImageUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageUsageFlagBits)
        const IMAGE_USAGE_TRANSFER_DST_BIT = vks::VK_IMAGE_USAGE_TRANSFER_DST_BIT;

        /// See [`VkImageUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageUsageFlagBits)
        const IMAGE_USAGE_SAMPLED_BIT = vks::VK_IMAGE_USAGE_SAMPLED_BIT;

        /// See [`VkImageUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageUsageFlagBits)
        const IMAGE_USAGE_STORAGE_BIT = vks::VK_IMAGE_USAGE_STORAGE_BIT;

        /// See [`VkImageUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageUsageFlagBits)
        const IMAGE_USAGE_COLOR_ATTACHMENT_BIT = vks::VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT;

        /// See [`VkImageUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageUsageFlagBits)
        const IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT = vks::VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT;

        /// See [`VkImageUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageUsageFlagBits)
        const IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT = vks::VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT;

        /// See [`VkImageUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageUsageFlagBits)
        const IMAGE_USAGE_INPUT_ATTACHMENT_BIT = vks::VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT;
    }
}

/// See [`VkImageUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageUsageFlagBits)
pub type ImageUsageFlagBits = ImageUsageFlags;

bitflags! {
    /// See [`VkImageCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageCreateFlagBits)
    #[derive(Default)]
    pub struct ImageCreateFlags: u32 {
        /// See [`VkImageCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageCreateFlagBits)
        const IMAGE_CREATE_FLAG_BITS_MAX_ENUM = vks::VK_IMAGE_CREATE_FLAG_BITS_MAX_ENUM;

        /// See [`VkImageCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageCreateFlagBits)
        const IMAGE_CREATE_SPARSE_BINDING_BIT = vks::VK_IMAGE_CREATE_SPARSE_BINDING_BIT;

        /// See [`VkImageCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageCreateFlagBits)
        const IMAGE_CREATE_SPARSE_RESIDENCY_BIT = vks::VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT;

        /// See [`VkImageCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageCreateFlagBits)
        const IMAGE_CREATE_SPARSE_ALIASED_BIT = vks::VK_IMAGE_CREATE_SPARSE_ALIASED_BIT;

        /// See [`VkImageCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageCreateFlagBits)
        const IMAGE_CREATE_MUTABLE_FORMAT_BIT = vks::VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT;

        /// See [`VkImageCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageCreateFlagBits)
        const IMAGE_CREATE_CUBE_COMPATIBLE_BIT = vks::VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT;
    }
}

/// See [`VkImageCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageCreateFlagBits)
pub type ImageCreateFlagBits = ImageCreateFlags;

bitflags! {
    /// See [`VkSampleCountFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSampleCountFlagBits)
    #[derive(Default)]
    pub struct SampleCountFlags: u32 {
        /// See [`VkSampleCountFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSampleCountFlagBits)
        const SAMPLE_COUNT_FLAG_BITS_MAX_ENUM = vks::VK_SAMPLE_COUNT_FLAG_BITS_MAX_ENUM;

        /// See [`VkSampleCountFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSampleCountFlagBits)
        const SAMPLE_COUNT_1_BIT = vks::VK_SAMPLE_COUNT_1_BIT;

        /// See [`VkSampleCountFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSampleCountFlagBits)
        const SAMPLE_COUNT_2_BIT = vks::VK_SAMPLE_COUNT_2_BIT;

        /// See [`VkSampleCountFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSampleCountFlagBits)
        const SAMPLE_COUNT_4_BIT = vks::VK_SAMPLE_COUNT_4_BIT;

        /// See [`VkSampleCountFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSampleCountFlagBits)
        const SAMPLE_COUNT_8_BIT = vks::VK_SAMPLE_COUNT_8_BIT;

        /// See [`VkSampleCountFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSampleCountFlagBits)
        const SAMPLE_COUNT_16_BIT = vks::VK_SAMPLE_COUNT_16_BIT;

        /// See [`VkSampleCountFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSampleCountFlagBits)
        const SAMPLE_COUNT_32_BIT = vks::VK_SAMPLE_COUNT_32_BIT;

        /// See [`VkSampleCountFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSampleCountFlagBits)
        const SAMPLE_COUNT_64_BIT = vks::VK_SAMPLE_COUNT_64_BIT;
    }
}

/// See [`VkSampleCountFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSampleCountFlagBits)
pub type SampleCountFlagBits = SampleCountFlags;

bitflags! {
    /// See [`VkQueueFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueueFlagBits)
    #[derive(Default)]
    pub struct QueueFlags: u32 {
        /// See [`VkQueueFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueueFlagBits)
        const QUEUE_FLAG_BITS_MAX_ENUM = vks::VK_QUEUE_FLAG_BITS_MAX_ENUM;

        /// See [`VkQueueFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueueFlagBits)
        const QUEUE_GRAPHICS_BIT = vks::VK_QUEUE_GRAPHICS_BIT;

        /// See [`VkQueueFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueueFlagBits)
        const QUEUE_COMPUTE_BIT = vks::VK_QUEUE_COMPUTE_BIT;

        /// See [`VkQueueFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueueFlagBits)
        const QUEUE_TRANSFER_BIT = vks::VK_QUEUE_TRANSFER_BIT;

        /// See [`VkQueueFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueueFlagBits)
        const QUEUE_SPARSE_BINDING_BIT = vks::VK_QUEUE_SPARSE_BINDING_BIT;
    }
}

/// See [`VkQueueFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueueFlagBits)
pub type QueueFlagBits = QueueFlags;

bitflags! {
    /// See [`VkMemoryPropertyFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryPropertyFlagBits)
    #[derive(Default)]
    pub struct MemoryPropertyFlags: u32 {
        /// See [`VkMemoryPropertyFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryPropertyFlagBits)
        const MEMORY_PROPERTY_FLAG_BITS_MAX_ENUM = vks::VK_MEMORY_PROPERTY_FLAG_BITS_MAX_ENUM;

        /// See [`VkMemoryPropertyFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryPropertyFlagBits)
        const MEMORY_PROPERTY_DEVICE_LOCAL_BIT = vks::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT;

        /// See [`VkMemoryPropertyFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryPropertyFlagBits)
        const MEMORY_PROPERTY_HOST_VISIBLE_BIT = vks::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT;

        /// See [`VkMemoryPropertyFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryPropertyFlagBits)
        const MEMORY_PROPERTY_HOST_COHERENT_BIT = vks::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT;

        /// See [`VkMemoryPropertyFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryPropertyFlagBits)
        const MEMORY_PROPERTY_HOST_CACHED_BIT = vks::VK_MEMORY_PROPERTY_HOST_CACHED_BIT;

        /// See [`VkMemoryPropertyFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryPropertyFlagBits)
        const MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT = vks::VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT;
    }
}

/// See [`VkMemoryPropertyFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryPropertyFlagBits)
pub type MemoryPropertyFlagBits = MemoryPropertyFlags;

bitflags! {
    /// See [`VkMemoryHeapFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryHeapFlagBits)
    #[derive(Default)]
    pub struct MemoryHeapFlags: u32 {
        /// See [`VkMemoryHeapFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryHeapFlagBits)
        const MEMORY_HEAP_FLAG_BITS_MAX_ENUM = vks::VK_MEMORY_HEAP_FLAG_BITS_MAX_ENUM;

        /// See [`VkMemoryHeapFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryHeapFlagBits)
        const MEMORY_HEAP_DEVICE_LOCAL_BIT = vks::VK_MEMORY_HEAP_DEVICE_LOCAL_BIT;
    }
}

/// See [`VkMemoryHeapFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryHeapFlagBits)
pub type MemoryHeapFlagBits = MemoryHeapFlags;

bitflags! {
    /// See [`VkDeviceCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceCreateFlags)
    #[derive(Default)]
    pub struct DeviceCreateFlags: u32 {
        /// See [`VkDeviceCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceCreateFlags)
        const DEVICE_CREATE_FLAG_BITS_MAX_ENUM = vks::VK_DEVICE_CREATE_FLAG_BITS_MAX_ENUM;
    }
}

/// See [`VkDeviceCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceCreateFlags)
pub type DeviceCreateFlagBits = DeviceCreateFlags;

bitflags! {
    /// See [`VkDeviceQueueCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceQueueCreateFlags)
    #[derive(Default)]
    pub struct DeviceQueueCreateFlags: u32 {
        /// See [`VkDeviceQueueCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceQueueCreateFlags)
        const DEVICE_QUEUE_CREATE_FLAG_BITS_MAX_ENUM = vks::VK_DEVICE_QUEUE_CREATE_FLAG_BITS_MAX_ENUM;
    }
}

/// See [`VkDeviceQueueCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceQueueCreateFlags)
pub type DeviceQueueCreateFlagBits = DeviceQueueCreateFlags;

bitflags! {
    /// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
    #[derive(Default)]
    pub struct PipelineStageFlags: u32 {
        /// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
        const PIPELINE_STAGE_FLAG_BITS_MAX_ENUM = vks::VK_PIPELINE_STAGE_FLAG_BITS_MAX_ENUM;

        /// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
        const PIPELINE_STAGE_TOP_OF_PIPE_BIT = vks::VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT;

        /// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
        const PIPELINE_STAGE_DRAW_INDIRECT_BIT = vks::VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT;

        /// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
        const PIPELINE_STAGE_VERTEX_INPUT_BIT = vks::VK_PIPELINE_STAGE_VERTEX_INPUT_BIT;

        /// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
        const PIPELINE_STAGE_VERTEX_SHADER_BIT = vks::VK_PIPELINE_STAGE_VERTEX_SHADER_BIT;

        /// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
        const PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT = vks::VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT;

        /// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
        const PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT = vks::VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT;

        /// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
        const PIPELINE_STAGE_GEOMETRY_SHADER_BIT = vks::VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT;

        /// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
        const PIPELINE_STAGE_FRAGMENT_SHADER_BIT = vks::VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT;

        /// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
        const PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT = vks::VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT;

        /// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
        const PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT = vks::VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT;

        /// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
        const PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT = vks::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT;

        /// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
        const PIPELINE_STAGE_COMPUTE_SHADER_BIT = vks::VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT;

        /// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
        const PIPELINE_STAGE_TRANSFER_BIT = vks::VK_PIPELINE_STAGE_TRANSFER_BIT;

        /// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
        const PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT = vks::VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT;

        /// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
        const PIPELINE_STAGE_HOST_BIT = vks::VK_PIPELINE_STAGE_HOST_BIT;

        /// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
        const PIPELINE_STAGE_ALL_GRAPHICS_BIT = vks::VK_PIPELINE_STAGE_ALL_GRAPHICS_BIT;

        /// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
        const PIPELINE_STAGE_ALL_COMMANDS_BIT = vks::VK_PIPELINE_STAGE_ALL_COMMANDS_BIT;
    }
}

/// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
pub type PipelineStageFlagBits = PipelineStageFlags;

bitflags! {
    /// See [`VkMemoryMapFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryMapFlags)
    #[derive(Default)]
    pub struct MemoryMapFlags: u32 {
        /// See [`VkMemoryMapFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryMapFlags)
        const MEMORY_MAP_FLAG_BITS_MAX_ENUM = vks::VK_MEMORY_MAP_FLAG_BITS_MAX_ENUM;
    }
}

/// See [`VkMemoryMapFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryMapFlags)
pub type MemoryMapFlagBits = MemoryMapFlags;

bitflags! {
    /// See [`VkImageAspectFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageAspectFlagBits)
    #[derive(Default)]
    pub struct ImageAspectFlags: u32 {
        /// See [`VkImageAspectFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageAspectFlagBits)
        const IMAGE_ASPECT_FLAG_BITS_MAX_ENUM = vks::VK_IMAGE_ASPECT_FLAG_BITS_MAX_ENUM;

        /// See [`VkImageAspectFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageAspectFlagBits)
        const IMAGE_ASPECT_COLOR_BIT = vks::VK_IMAGE_ASPECT_COLOR_BIT;

        /// See [`VkImageAspectFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageAspectFlagBits)
        const IMAGE_ASPECT_DEPTH_BIT = vks::VK_IMAGE_ASPECT_DEPTH_BIT;

        /// See [`VkImageAspectFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageAspectFlagBits)
        const IMAGE_ASPECT_STENCIL_BIT = vks::VK_IMAGE_ASPECT_STENCIL_BIT;

        /// See [`VkImageAspectFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageAspectFlagBits)
        const IMAGE_ASPECT_METADATA_BIT = vks::VK_IMAGE_ASPECT_METADATA_BIT;
    }
}

/// See [`VkImageAspectFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageAspectFlagBits)
pub type ImageAspectFlagBits = ImageAspectFlags;

bitflags! {
    /// See [`VkSparseImageFormatFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseImageFormatFlagBits)
    #[derive(Default)]
    pub struct SparseImageFormatFlags: u32 {
        /// See [`VkSparseImageFormatFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseImageFormatFlagBits)
        const SPARSE_IMAGE_FORMAT_FLAG_BITS_MAX_ENUM = vks::VK_SPARSE_IMAGE_FORMAT_FLAG_BITS_MAX_ENUM;

        /// See [`VkSparseImageFormatFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseImageFormatFlagBits)
        const SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT = vks::VK_SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT;

        /// See [`VkSparseImageFormatFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseImageFormatFlagBits)
        const SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT = vks::VK_SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT;

        /// See [`VkSparseImageFormatFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseImageFormatFlagBits)
        const SPARSE_IMAGE_FORMAT_NONSTANDARD_BLOCK_SIZE_BIT = vks::VK_SPARSE_IMAGE_FORMAT_NONSTANDARD_BLOCK_SIZE_BIT;
    }
}

/// See [`VkSparseImageFormatFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseImageFormatFlagBits)
pub type SparseImageFormatFlagBits = SparseImageFormatFlags;

bitflags! {
    /// See [`VkSparseMemoryBindFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseMemoryBindFlagBits)
    #[derive(Default)]
    pub struct SparseMemoryBindFlags: u32 {
        /// See [`VkSparseMemoryBindFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseMemoryBindFlagBits)
        const SPARSE_MEMORY_BIND_FLAG_BITS_MAX_ENUM = vks::VK_SPARSE_MEMORY_BIND_FLAG_BITS_MAX_ENUM;

        /// See [`VkSparseMemoryBindFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseMemoryBindFlagBits)
        const SPARSE_MEMORY_BIND_METADATA_BIT = vks::VK_SPARSE_MEMORY_BIND_METADATA_BIT;
    }
}

/// See [`VkSparseMemoryBindFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseMemoryBindFlagBits)
pub type SparseMemoryBindFlagBits = SparseMemoryBindFlags;

bitflags! {
    /// See [`VkFenceCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFenceCreateFlagBits)
    #[derive(Default)]
    pub struct FenceCreateFlags: u32 {
        /// See [`VkFenceCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFenceCreateFlagBits)
        const FENCE_CREATE_FLAG_BITS_MAX_ENUM = vks::VK_FENCE_CREATE_FLAG_BITS_MAX_ENUM;

        /// See [`VkFenceCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFenceCreateFlagBits)
        const FENCE_CREATE_SIGNALED_BIT = vks::VK_FENCE_CREATE_SIGNALED_BIT;
    }
}

/// See [`VkFenceCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFenceCreateFlagBits)
pub type FenceCreateFlagBits = FenceCreateFlags;

bitflags! {
    /// See [`VkSemaphoreCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSemaphoreCreateFlags)
    #[derive(Default)]
    pub struct SemaphoreCreateFlags: u32 {
        /// See [`VkSemaphoreCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSemaphoreCreateFlags)
        const SEMAPHORE_CREATE_FLAG_BITS_MAX_ENUM = vks::VK_SEMAPHORE_CREATE_FLAG_BITS_MAX_ENUM;
    }
}

/// See [`VkSemaphoreCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSemaphoreCreateFlags)
pub type SemaphoreCreateFlagBits = SemaphoreCreateFlags;

bitflags! {
    /// See [`VkEventCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkEventCreateFlags)
    #[derive(Default)]
    pub struct EventCreateFlags: u32 {
        /// See [`VkEventCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkEventCreateFlags)
        const EVENT_CREATE_FLAG_BITS_MAX_ENUM = vks::VK_EVENT_CREATE_FLAG_BITS_MAX_ENUM;
    }
}

/// See [`VkEventCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkEventCreateFlags)
pub type EventCreateFlagBits = EventCreateFlags;

bitflags! {
    /// See [`VkQueryPoolCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPoolCreateFlags)
    #[derive(Default)]
    pub struct QueryPoolCreateFlags: u32 {
        /// See [`VkQueryPoolCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPoolCreateFlags)
        const QUERY_POOL_CREATE_FLAG_BITS_MAX_ENUM = vks::VK_QUERY_POOL_CREATE_FLAG_BITS_MAX_ENUM;
    }
}

/// See [`VkQueryPoolCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPoolCreateFlags)
pub type QueryPoolCreateFlagBits = QueryPoolCreateFlags;

bitflags! {
    /// See [`VkQueryPipelineStatisticFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlagBits)
    #[derive(Default)]
    pub struct QueryPipelineStatisticFlags: u32 {
        /// See [`VkQueryPipelineStatisticFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlagBits)
        const QUERY_PIPELINE_STATISTIC_FLAG_BITS_MAX_ENUM = vks::VK_QUERY_PIPELINE_STATISTIC_FLAG_BITS_MAX_ENUM;

        /// See [`VkQueryPipelineStatisticFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlagBits)
        const QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_VERTICES_BIT = vks::VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_VERTICES_BIT;

        /// See [`VkQueryPipelineStatisticFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlagBits)
        const QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_PRIMITIVES_BIT = vks::VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_PRIMITIVES_BIT;

        /// See [`VkQueryPipelineStatisticFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlagBits)
        const QUERY_PIPELINE_STATISTIC_VERTEX_SHADER_INVOCATIONS_BIT = vks::VK_QUERY_PIPELINE_STATISTIC_VERTEX_SHADER_INVOCATIONS_BIT;

        /// See [`VkQueryPipelineStatisticFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlagBits)
        const QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_INVOCATIONS_BIT = vks::VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_INVOCATIONS_BIT;

        /// See [`VkQueryPipelineStatisticFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlagBits)
        const QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_PRIMITIVES_BIT = vks::VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_PRIMITIVES_BIT;

        /// See [`VkQueryPipelineStatisticFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlagBits)
        const QUERY_PIPELINE_STATISTIC_CLIPPING_INVOCATIONS_BIT = vks::VK_QUERY_PIPELINE_STATISTIC_CLIPPING_INVOCATIONS_BIT;

        /// See [`VkQueryPipelineStatisticFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlagBits)
        const QUERY_PIPELINE_STATISTIC_CLIPPING_PRIMITIVES_BIT = vks::VK_QUERY_PIPELINE_STATISTIC_CLIPPING_PRIMITIVES_BIT;

        /// See [`VkQueryPipelineStatisticFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlagBits)
        const QUERY_PIPELINE_STATISTIC_FRAGMENT_SHADER_INVOCATIONS_BIT = vks::VK_QUERY_PIPELINE_STATISTIC_FRAGMENT_SHADER_INVOCATIONS_BIT;

        /// See [`VkQueryPipelineStatisticFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlagBits)
        const QUERY_PIPELINE_STATISTIC_TESSELLATION_CONTROL_SHADER_PATCHES_BIT = vks::VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_CONTROL_SHADER_PATCHES_BIT;

        /// See [`VkQueryPipelineStatisticFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlagBits)
        const QUERY_PIPELINE_STATISTIC_TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT = vks::VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT;

        /// See [`VkQueryPipelineStatisticFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlagBits)
        const QUERY_PIPELINE_STATISTIC_COMPUTE_SHADER_INVOCATIONS_BIT = vks::VK_QUERY_PIPELINE_STATISTIC_COMPUTE_SHADER_INVOCATIONS_BIT;
    }
}

/// See [`VkQueryPipelineStatisticFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlagBits)
pub type QueryPipelineStatisticFlagBits = QueryPipelineStatisticFlags;

bitflags! {
    /// See [`VkQueryResultFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryResultFlagBits)
    #[derive(Default)]
    pub struct QueryResultFlags: u32 {
        /// See [`VkQueryResultFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryResultFlagBits)
        const QUERY_RESULT_FLAG_BITS_MAX_ENUM = vks::VK_QUERY_RESULT_FLAG_BITS_MAX_ENUM;

        /// See [`VkQueryResultFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryResultFlagBits)
        const QUERY_RESULT_64_BIT = vks::VK_QUERY_RESULT_64_BIT;

        /// See [`VkQueryResultFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryResultFlagBits)
        const QUERY_RESULT_WAIT_BIT = vks::VK_QUERY_RESULT_WAIT_BIT;

        /// See [`VkQueryResultFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryResultFlagBits)
        const QUERY_RESULT_WITH_AVAILABILITY_BIT = vks::VK_QUERY_RESULT_WITH_AVAILABILITY_BIT;

        /// See [`VkQueryResultFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryResultFlagBits)
        const QUERY_RESULT_PARTIAL_BIT = vks::VK_QUERY_RESULT_PARTIAL_BIT;
    }
}

/// See [`VkQueryResultFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryResultFlagBits)
pub type QueryResultFlagBits = QueryResultFlags;

bitflags! {
    /// See [`VkBufferCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferCreateFlagBits)
    #[derive(Default)]
    pub struct BufferCreateFlags: u32 {
        /// See [`VkBufferCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferCreateFlagBits)
        const BUFFER_CREATE_FLAG_BITS_MAX_ENUM = vks::VK_BUFFER_CREATE_FLAG_BITS_MAX_ENUM;

        /// See [`VkBufferCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferCreateFlagBits)
        const BUFFER_CREATE_SPARSE_BINDING_BIT = vks::VK_BUFFER_CREATE_SPARSE_BINDING_BIT;

        /// See [`VkBufferCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferCreateFlagBits)
        const BUFFER_CREATE_SPARSE_RESIDENCY_BIT = vks::VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT;

        /// See [`VkBufferCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferCreateFlagBits)
        const BUFFER_CREATE_SPARSE_ALIASED_BIT = vks::VK_BUFFER_CREATE_SPARSE_ALIASED_BIT;
    }
}

/// See [`VkBufferCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferCreateFlagBits)
pub type BufferCreateFlagBits = BufferCreateFlags;

bitflags! {
    /// See [`VkBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferUsageFlagBits)
    #[derive(Default)]
    pub struct BufferUsageFlags: u32 {
        /// See [`VkBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferUsageFlagBits)
        const BUFFER_USAGE_FLAG_BITS_MAX_ENUM = vks::VK_BUFFER_USAGE_FLAG_BITS_MAX_ENUM;

        /// See [`VkBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferUsageFlagBits)
        const BUFFER_USAGE_TRANSFER_SRC_BIT = vks::VK_BUFFER_USAGE_TRANSFER_SRC_BIT;

        /// See [`VkBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferUsageFlagBits)
        const BUFFER_USAGE_TRANSFER_DST_BIT = vks::VK_BUFFER_USAGE_TRANSFER_DST_BIT;

        /// See [`VkBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferUsageFlagBits)
        const BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT = vks::VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT;

        /// See [`VkBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferUsageFlagBits)
        const BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT = vks::VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT;

        /// See [`VkBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferUsageFlagBits)
        const BUFFER_USAGE_UNIFORM_BUFFER_BIT = vks::VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT;

        /// See [`VkBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferUsageFlagBits)
        const BUFFER_USAGE_STORAGE_BUFFER_BIT = vks::VK_BUFFER_USAGE_STORAGE_BUFFER_BIT;

        /// See [`VkBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferUsageFlagBits)
        const BUFFER_USAGE_INDEX_BUFFER_BIT = vks::VK_BUFFER_USAGE_INDEX_BUFFER_BIT;

        /// See [`VkBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferUsageFlagBits)
        const BUFFER_USAGE_VERTEX_BUFFER_BIT = vks::VK_BUFFER_USAGE_VERTEX_BUFFER_BIT;

        /// See [`VkBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferUsageFlagBits)
        const BUFFER_USAGE_INDIRECT_BUFFER_BIT = vks::VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT;
    }
}

/// See [`VkBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferUsageFlagBits)
pub type BufferUsageFlagBits = BufferUsageFlags;

bitflags! {
    /// See [`VkBufferViewCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferViewCreateFlags)
    #[derive(Default)]
    pub struct BufferViewCreateFlags: u32 {
        /// See [`VkBufferViewCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferViewCreateFlags)
        const BUFFER_VIEW_CREATE_FLAG_BITS_MAX_ENUM = vks::VK_BUFFER_VIEW_CREATE_FLAG_BITS_MAX_ENUM;
    }
}

/// See [`VkBufferViewCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferViewCreateFlags)
pub type BufferViewCreateFlagBits = BufferViewCreateFlags;

bitflags! {
    /// See [`VkImageViewCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageViewCreateFlags)
    #[derive(Default)]
    pub struct ImageViewCreateFlags: u32 {
        /// See [`VkImageViewCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageViewCreateFlags)
        const IMAGE_VIEW_CREATE_FLAG_BITS_MAX_ENUM = vks::VK_IMAGE_VIEW_CREATE_FLAG_BITS_MAX_ENUM;
    }
}

/// See [`VkImageViewCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageViewCreateFlags)
pub type ImageViewCreateFlagBits = ImageViewCreateFlags;

bitflags! {
    /// See [`VkShaderModuleCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderModuleCreateFlags)
    #[derive(Default)]
    pub struct ShaderModuleCreateFlags: u32 {
        /// See [`VkShaderModuleCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderModuleCreateFlags)
        const SHADER_MODULE_CREATE_FLAG_BITS_MAX_ENUM = vks::VK_SHADER_MODULE_CREATE_FLAG_BITS_MAX_ENUM;
    }
}

/// See [`VkShaderModuleCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderModuleCreateFlags)
pub type ShaderModuleCreateFlagBits = ShaderModuleCreateFlags;

bitflags! {
    /// See [`VkPipelineCacheCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCacheCreateFlags)
    #[derive(Default)]
    pub struct PipelineCacheCreateFlags: u32 {
        /// See [`VkPipelineCacheCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCacheCreateFlags)
        const PIPELINE_CACHE_CREATE_FLAG_BITS_MAX_ENUM = vks::VK_PIPELINE_CACHE_CREATE_FLAG_BITS_MAX_ENUM;
    }
}

/// See [`VkPipelineCacheCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCacheCreateFlags)
pub type PipelineCacheCreateFlagBits = PipelineCacheCreateFlags;

bitflags! {
    /// See [`VkPipelineCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCreateFlagBits)
    #[derive(Default)]
    pub struct PipelineCreateFlags: u32 {
        /// See [`VkPipelineCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCreateFlagBits)
        const PIPELINE_CREATE_FLAG_BITS_MAX_ENUM = vks::VK_PIPELINE_CREATE_FLAG_BITS_MAX_ENUM;

        /// See [`VkPipelineCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCreateFlagBits)
        const PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT = vks::VK_PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT;

        /// See [`VkPipelineCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCreateFlagBits)
        const PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT = vks::VK_PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT;

        /// See [`VkPipelineCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCreateFlagBits)
        const PIPELINE_CREATE_DERIVATIVE_BIT = vks::VK_PIPELINE_CREATE_DERIVATIVE_BIT;
    }
}

/// See [`VkPipelineCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCreateFlagBits)
pub type PipelineCreateFlagBits = PipelineCreateFlags;

bitflags! {
    /// See [`VkPipelineShaderStageCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineShaderStageCreateFlags)
    #[derive(Default)]
    pub struct PipelineShaderStageCreateFlags: u32 {
        /// See [`VkPipelineShaderStageCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineShaderStageCreateFlags)
        const PIPELINE_SHADER_STAGE_CREATE_FLAG_BITS_MAX_ENUM = vks::VK_PIPELINE_SHADER_STAGE_CREATE_FLAG_BITS_MAX_ENUM;
    }
}

/// See [`VkPipelineShaderStageCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineShaderStageCreateFlags)
pub type PipelineShaderStageCreateFlagBits = PipelineShaderStageCreateFlags;

bitflags! {
    /// See [`VkShaderStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderStageFlagBits)
    #[derive(Default)]
    pub struct ShaderStageFlags: u32 {
        /// See [`VkShaderStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderStageFlagBits)
        const SHADER_STAGE_FLAG_BITS_MAX_ENUM = vks::VK_SHADER_STAGE_FLAG_BITS_MAX_ENUM;

        /// See [`VkShaderStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderStageFlagBits)
        const SHADER_STAGE_VERTEX_BIT = vks::VK_SHADER_STAGE_VERTEX_BIT;

        /// See [`VkShaderStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderStageFlagBits)
        const SHADER_STAGE_TESSELLATION_CONTROL_BIT = vks::VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT;

        /// See [`VkShaderStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderStageFlagBits)
        const SHADER_STAGE_TESSELLATION_EVALUATION_BIT = vks::VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT;

        /// See [`VkShaderStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderStageFlagBits)
        const SHADER_STAGE_GEOMETRY_BIT = vks::VK_SHADER_STAGE_GEOMETRY_BIT;

        /// See [`VkShaderStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderStageFlagBits)
        const SHADER_STAGE_FRAGMENT_BIT = vks::VK_SHADER_STAGE_FRAGMENT_BIT;

        /// See [`VkShaderStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderStageFlagBits)
        const SHADER_STAGE_COMPUTE_BIT = vks::VK_SHADER_STAGE_COMPUTE_BIT;

        /// See [`VkShaderStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderStageFlagBits)
        const SHADER_STAGE_ALL_GRAPHICS = vks::VK_SHADER_STAGE_ALL_GRAPHICS;

        /// See [`VkShaderStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderStageFlagBits)
        const SHADER_STAGE_ALL = vks::VK_SHADER_STAGE_ALL;
    }
}

/// See [`VkShaderStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderStageFlagBits)
pub type ShaderStageFlagBits = ShaderStageFlags;

bitflags! {
    /// See [`VkPipelineVertexInputStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineVertexInputStateCreateFlags)
    #[derive(Default)]
    pub struct PipelineVertexInputStateCreateFlags: u32 {
        /// See [`VkPipelineVertexInputStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineVertexInputStateCreateFlags)
        const PIPELINE_VERTEX_INPUT_STATE_CREATE_FLAG_BITS_MAX_ENUM = vks::VK_PIPELINE_VERTEX_INPUT_STATE_CREATE_FLAG_BITS_MAX_ENUM;
    }
}

/// See [`VkPipelineVertexInputStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineVertexInputStateCreateFlags)
pub type PipelineVertexInputStateCreateFlagBits = PipelineVertexInputStateCreateFlags;

bitflags! {
    /// See [`VkPipelineInputAssemblyStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineInputAssemblyStateCreateFlags)
    #[derive(Default)]
    pub struct PipelineInputAssemblyStateCreateFlags: u32 {
        /// See [`VkPipelineInputAssemblyStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineInputAssemblyStateCreateFlags)
        const PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_FLAG_BITS_MAX_ENUM = vks::VK_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_FLAG_BITS_MAX_ENUM;
    }
}

/// See [`VkPipelineInputAssemblyStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineInputAssemblyStateCreateFlags)
pub type PipelineInputAssemblyStateCreateFlagBits = PipelineInputAssemblyStateCreateFlags;

bitflags! {
    /// See [`VkPipelineTessellationStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineTessellationStateCreateFlags)
    #[derive(Default)]
    pub struct PipelineTessellationStateCreateFlags: u32 {
        /// See [`VkPipelineTessellationStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineTessellationStateCreateFlags)
        const PIPELINE_TESSELLATION_STATE_CREATE_FLAG_BITS_MAX_ENUM = vks::VK_PIPELINE_TESSELLATION_STATE_CREATE_FLAG_BITS_MAX_ENUM;
    }
}

/// See [`VkPipelineTessellationStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineTessellationStateCreateFlags)
pub type PipelineTessellationStateCreateFlagBits = PipelineTessellationStateCreateFlags;

bitflags! {
    /// See [`VkPipelineViewportStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineViewportStateCreateFlags)
    #[derive(Default)]
    pub struct PipelineViewportStateCreateFlags: u32 {
        /// See [`VkPipelineViewportStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineViewportStateCreateFlags)
        const PIPELINE_VIEWPORT_STATE_CREATE_FLAG_BITS_MAX_ENUM = vks::VK_PIPELINE_VIEWPORT_STATE_CREATE_FLAG_BITS_MAX_ENUM;
    }
}

/// See [`VkPipelineViewportStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineViewportStateCreateFlags)
pub type PipelineViewportStateCreateFlagBits = PipelineViewportStateCreateFlags;

bitflags! {
    /// See [`VkPipelineRasterizationStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineRasterizationStateCreateFlags)
    #[derive(Default)]
    pub struct PipelineRasterizationStateCreateFlags: u32 {
        /// See [`VkPipelineRasterizationStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineRasterizationStateCreateFlags)
        const PIPELINE_RASTERIZATION_STATE_CREATE_FLAG_BITS_MAX_ENUM = vks::VK_PIPELINE_RASTERIZATION_STATE_CREATE_FLAG_BITS_MAX_ENUM;
    }
}

/// See [`VkPipelineRasterizationStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineRasterizationStateCreateFlags)
pub type PipelineRasterizationStateCreateFlagBits = PipelineRasterizationStateCreateFlags;

bitflags! {
    /// See [`VkCullModeFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCullModeFlagBits)
    #[derive(Default)]
    pub struct CullModeFlags: u32 {
        /// See [`VkCullModeFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCullModeFlagBits)
        const CULL_MODE_FLAG_BITS_MAX_ENUM = vks::VK_CULL_MODE_FLAG_BITS_MAX_ENUM;

        /// See [`VkCullModeFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCullModeFlagBits)
        const CULL_MODE_NONE = vks::VK_CULL_MODE_NONE;

        /// See [`VkCullModeFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCullModeFlagBits)
        const CULL_MODE_FRONT_BIT = vks::VK_CULL_MODE_FRONT_BIT;

        /// See [`VkCullModeFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCullModeFlagBits)
        const CULL_MODE_BACK_BIT = vks::VK_CULL_MODE_BACK_BIT;

        /// See [`VkCullModeFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCullModeFlagBits)
        const CULL_MODE_FRONT_AND_BACK = vks::VK_CULL_MODE_FRONT_AND_BACK;
    }
}

/// See [`VkCullModeFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCullModeFlagBits)
pub type CullModeFlagBits = CullModeFlags;

bitflags! {
    /// See [`VkPipelineMultisampleStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineMultisampleStateCreateFlags)
    #[derive(Default)]
    pub struct PipelineMultisampleStateCreateFlags: u32 {
        /// See [`VkPipelineMultisampleStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineMultisampleStateCreateFlags)
        const PIPELINE_MULTISAMPLE_STATE_CREATE_FLAG_BITS_MAX_ENUM = vks::VK_PIPELINE_MULTISAMPLE_STATE_CREATE_FLAG_BITS_MAX_ENUM;
    }
}

/// See [`VkPipelineMultisampleStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineMultisampleStateCreateFlags)
pub type PipelineMultisampleStateCreateFlagBits = PipelineMultisampleStateCreateFlags;

bitflags! {
    /// See [`VkPipelineDepthStencilStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineDepthStencilStateCreateFlags)
    #[derive(Default)]
    pub struct PipelineDepthStencilStateCreateFlags: u32 {
        /// See [`VkPipelineDepthStencilStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineDepthStencilStateCreateFlags)
        const PIPELINE_DEPTH_STENCIL_STATE_CREATE_FLAG_BITS_MAX_ENUM = vks::VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_FLAG_BITS_MAX_ENUM;
    }
}

/// See [`VkPipelineDepthStencilStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineDepthStencilStateCreateFlags)
pub type PipelineDepthStencilStateCreateFlagBits = PipelineDepthStencilStateCreateFlags;

bitflags! {
    /// See [`VkPipelineColorBlendStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineColorBlendStateCreateFlags)
    #[derive(Default)]
    pub struct PipelineColorBlendStateCreateFlags: u32 {
        /// See [`VkPipelineColorBlendStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineColorBlendStateCreateFlags)
        const PIPELINE_COLOR_BLEND_STATE_CREATE_FLAG_BITS_MAX_ENUM = vks::VK_PIPELINE_COLOR_BLEND_STATE_CREATE_FLAG_BITS_MAX_ENUM;
    }
}

/// See [`VkPipelineColorBlendStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineColorBlendStateCreateFlags)
pub type PipelineColorBlendStateCreateFlagBits = PipelineColorBlendStateCreateFlags;

bitflags! {
    /// See [`VkColorComponentFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorComponentFlagBits)
    #[derive(Default)]
    pub struct ColorComponentFlags: u32 {
        /// See [`VkColorComponentFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorComponentFlagBits)
        const COLOR_COMPONENT_FLAG_BITS_MAX_ENUM = vks::VK_COLOR_COMPONENT_FLAG_BITS_MAX_ENUM;

        /// See [`VkColorComponentFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorComponentFlagBits)
        const COLOR_COMPONENT_R_BIT = vks::VK_COLOR_COMPONENT_R_BIT;

        /// See [`VkColorComponentFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorComponentFlagBits)
        const COLOR_COMPONENT_G_BIT = vks::VK_COLOR_COMPONENT_G_BIT;

        /// See [`VkColorComponentFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorComponentFlagBits)
        const COLOR_COMPONENT_B_BIT = vks::VK_COLOR_COMPONENT_B_BIT;

        /// See [`VkColorComponentFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorComponentFlagBits)
        const COLOR_COMPONENT_A_BIT = vks::VK_COLOR_COMPONENT_A_BIT;
    }
}

/// See [`VkColorComponentFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorComponentFlagBits)
pub type ColorComponentFlagBits = ColorComponentFlags;

bitflags! {
    /// See [`VkPipelineDynamicStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineDynamicStateCreateFlags)
    #[derive(Default)]
    pub struct PipelineDynamicStateCreateFlags: u32 {
        /// See [`VkPipelineDynamicStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineDynamicStateCreateFlags)
        const PIPELINE_DYNAMIC_STATE_CREATE_FLAG_BITS_MAX_ENUM = vks::VK_PIPELINE_DYNAMIC_STATE_CREATE_FLAG_BITS_MAX_ENUM;
    }
}

/// See [`VkPipelineDynamicStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineDynamicStateCreateFlags)
pub type PipelineDynamicStateCreateFlagBits = PipelineDynamicStateCreateFlags;

bitflags! {
    /// See [`VkPipelineLayoutCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineLayoutCreateFlags)
    #[derive(Default)]
    pub struct PipelineLayoutCreateFlags: u32 {
        /// See [`VkPipelineLayoutCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineLayoutCreateFlags)
        const PIPELINE_LAYOUT_CREATE_FLAG_BITS_MAX_ENUM = vks::VK_PIPELINE_LAYOUT_CREATE_FLAG_BITS_MAX_ENUM;
    }
}

/// See [`VkPipelineLayoutCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineLayoutCreateFlags)
pub type PipelineLayoutCreateFlagBits = PipelineLayoutCreateFlags;

bitflags! {
    /// See [`VkSamplerCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSamplerCreateFlags)
    #[derive(Default)]
    pub struct SamplerCreateFlags: u32 {
        /// See [`VkSamplerCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSamplerCreateFlags)
        const SAMPLER_CREATE_FLAG_BITS_MAX_ENUM = vks::VK_SAMPLER_CREATE_FLAG_BITS_MAX_ENUM;
    }
}

/// See [`VkSamplerCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSamplerCreateFlags)
pub type SamplerCreateFlagBits = SamplerCreateFlags;

bitflags! {
    /// See [`VkDescriptorSetLayoutCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorSetLayoutCreateFlagBits)
    #[derive(Default)]
    pub struct DescriptorSetLayoutCreateFlags: u32 {
        /// See [`VkDescriptorSetLayoutCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorSetLayoutCreateFlagBits)
        const DESCRIPTOR_SET_LAYOUT_CREATE_FLAG_BITS_MAX_ENUM = vks::VK_DESCRIPTOR_SET_LAYOUT_CREATE_FLAG_BITS_MAX_ENUM;
    }
}

/// See [`VkDescriptorSetLayoutCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorSetLayoutCreateFlagBits)
pub type DescriptorSetLayoutCreateFlagBits = DescriptorSetLayoutCreateFlags;

bitflags! {
    /// See [`VkDescriptorPoolCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorPoolCreateFlagBits)
    #[derive(Default)]
    pub struct DescriptorPoolCreateFlags: u32 {
        /// See [`VkDescriptorPoolCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorPoolCreateFlagBits)
        const DESCRIPTOR_POOL_CREATE_FLAG_BITS_MAX_ENUM = vks::VK_DESCRIPTOR_POOL_CREATE_FLAG_BITS_MAX_ENUM;

        /// See [`VkDescriptorPoolCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorPoolCreateFlagBits)
        const DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT = vks::VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT;
    }
}

/// See [`VkDescriptorPoolCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorPoolCreateFlagBits)
pub type DescriptorPoolCreateFlagBits = DescriptorPoolCreateFlags;

bitflags! {
    /// See [`VkDescriptorPoolResetFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorPoolResetFlags)
    #[derive(Default)]
    pub struct DescriptorPoolResetFlags: u32 {
        /// See [`VkDescriptorPoolResetFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorPoolResetFlags)
        const DESCRIPTOR_POOL_RESET_FLAG_BITS_MAX_ENUM = vks::VK_DESCRIPTOR_POOL_RESET_FLAG_BITS_MAX_ENUM;
    }
}

/// See [`VkDescriptorPoolResetFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorPoolResetFlags)
pub type DescriptorPoolResetFlagBits = DescriptorPoolResetFlags;

bitflags! {
    /// See [`VkFramebufferCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFramebufferCreateFlags)
    #[derive(Default)]
    pub struct FramebufferCreateFlags: u32 {
        /// See [`VkFramebufferCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFramebufferCreateFlags)
        const FRAMEBUFFER_CREATE_FLAG_BITS_MAX_ENUM = vks::VK_FRAMEBUFFER_CREATE_FLAG_BITS_MAX_ENUM;
    }
}

/// See [`VkFramebufferCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFramebufferCreateFlags)
pub type FramebufferCreateFlagBits = FramebufferCreateFlags;

bitflags! {
    /// See [`VkRenderPassCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkRenderPassCreateFlags)
    #[derive(Default)]
    pub struct RenderPassCreateFlags: u32 {
        /// See [`VkRenderPassCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkRenderPassCreateFlags)
        const RENDER_PASS_CREATE_FLAG_BITS_MAX_ENUM = vks::VK_RENDER_PASS_CREATE_FLAG_BITS_MAX_ENUM;
    }
}

/// See [`VkRenderPassCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkRenderPassCreateFlags)
pub type RenderPassCreateFlagBits = RenderPassCreateFlags;

bitflags! {
    /// See [`VkAttachmentDescriptionFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAttachmentDescriptionFlagBits)
    #[derive(Default)]
    pub struct AttachmentDescriptionFlags: u32 {
        /// See [`VkAttachmentDescriptionFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAttachmentDescriptionFlagBits)
        const ATTACHMENT_DESCRIPTION_FLAG_BITS_MAX_ENUM = vks::VK_ATTACHMENT_DESCRIPTION_FLAG_BITS_MAX_ENUM;

        /// See [`VkAttachmentDescriptionFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAttachmentDescriptionFlagBits)
        const ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT = vks::VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT;
    }
}

/// See [`VkAttachmentDescriptionFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAttachmentDescriptionFlagBits)
pub type AttachmentDescriptionFlagBits = AttachmentDescriptionFlags;

bitflags! {
    /// See [`VkSubpassDescriptionFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSubpassDescriptionFlagBits)
    #[derive(Default)]
    pub struct SubpassDescriptionFlags: u32 {
        /// See [`VkSubpassDescriptionFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSubpassDescriptionFlagBits)
        const SUBPASS_DESCRIPTION_FLAG_BITS_MAX_ENUM = vks::VK_SUBPASS_DESCRIPTION_FLAG_BITS_MAX_ENUM;
    }
}

/// See [`VkSubpassDescriptionFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSubpassDescriptionFlagBits)
pub type SubpassDescriptionFlagBits = SubpassDescriptionFlags;

bitflags! {
    /// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
    #[derive(Default)]
    pub struct AccessFlags: u32 {
        /// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
        const ACCESS_FLAG_BITS_MAX_ENUM = vks::VK_ACCESS_FLAG_BITS_MAX_ENUM;

        /// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
        const ACCESS_INDIRECT_COMMAND_READ_BIT = vks::VK_ACCESS_INDIRECT_COMMAND_READ_BIT;

        /// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
        const ACCESS_INDEX_READ_BIT = vks::VK_ACCESS_INDEX_READ_BIT;

        /// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
        const ACCESS_VERTEX_ATTRIBUTE_READ_BIT = vks::VK_ACCESS_VERTEX_ATTRIBUTE_READ_BIT;

        /// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
        const ACCESS_UNIFORM_READ_BIT = vks::VK_ACCESS_UNIFORM_READ_BIT;

        /// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
        const ACCESS_INPUT_ATTACHMENT_READ_BIT = vks::VK_ACCESS_INPUT_ATTACHMENT_READ_BIT;

        /// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
        const ACCESS_SHADER_READ_BIT = vks::VK_ACCESS_SHADER_READ_BIT;

        /// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
        const ACCESS_SHADER_WRITE_BIT = vks::VK_ACCESS_SHADER_WRITE_BIT;

        /// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
        const ACCESS_COLOR_ATTACHMENT_READ_BIT = vks::VK_ACCESS_COLOR_ATTACHMENT_READ_BIT;

        /// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
        const ACCESS_COLOR_ATTACHMENT_WRITE_BIT = vks::VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT;

        /// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
        const ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT = vks::VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT;

        /// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
        const ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT = vks::VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT;

        /// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
        const ACCESS_TRANSFER_READ_BIT = vks::VK_ACCESS_TRANSFER_READ_BIT;

        /// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
        const ACCESS_TRANSFER_WRITE_BIT = vks::VK_ACCESS_TRANSFER_WRITE_BIT;

        /// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
        const ACCESS_HOST_READ_BIT = vks::VK_ACCESS_HOST_READ_BIT;

        /// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
        const ACCESS_HOST_WRITE_BIT = vks::VK_ACCESS_HOST_WRITE_BIT;

        /// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
        const ACCESS_MEMORY_READ_BIT = vks::VK_ACCESS_MEMORY_READ_BIT;

        /// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
        const ACCESS_MEMORY_WRITE_BIT = vks::VK_ACCESS_MEMORY_WRITE_BIT;
    }
}

/// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
pub type AccessFlagBits = AccessFlags;

bitflags! {
    /// See [`VkDependencyFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDependencyFlagBits)
    #[derive(Default)]
    pub struct DependencyFlags: u32 {
        /// See [`VkDependencyFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDependencyFlagBits)
        const DEPENDENCY_FLAG_BITS_MAX_ENUM = vks::VK_DEPENDENCY_FLAG_BITS_MAX_ENUM;

        /// See [`VkDependencyFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDependencyFlagBits)
        const DEPENDENCY_BY_REGION_BIT = vks::VK_DEPENDENCY_BY_REGION_BIT;
    }
}

/// See [`VkDependencyFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDependencyFlagBits)
pub type DependencyFlagBits = DependencyFlags;

bitflags! {
    /// See [`VkCommandPoolCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandPoolCreateFlagBits)
    #[derive(Default)]
    pub struct CommandPoolCreateFlags: u32 {
        /// See [`VkCommandPoolCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandPoolCreateFlagBits)
        const COMMAND_POOL_CREATE_FLAG_BITS_MAX_ENUM = vks::VK_COMMAND_POOL_CREATE_FLAG_BITS_MAX_ENUM;

        /// See [`VkCommandPoolCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandPoolCreateFlagBits)
        const COMMAND_POOL_CREATE_TRANSIENT_BIT = vks::VK_COMMAND_POOL_CREATE_TRANSIENT_BIT;

        /// See [`VkCommandPoolCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandPoolCreateFlagBits)
        const COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT = vks::VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT;
    }
}

/// See [`VkCommandPoolCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandPoolCreateFlagBits)
pub type CommandPoolCreateFlagBits = CommandPoolCreateFlags;

bitflags! {
    /// See [`VkCommandPoolResetFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandPoolResetFlagBits)
    #[derive(Default)]
    pub struct CommandPoolResetFlags: u32 {
        /// See [`VkCommandPoolResetFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandPoolResetFlagBits)
        const COMMAND_POOL_RESET_FLAG_BITS_MAX_ENUM = vks::VK_COMMAND_POOL_RESET_FLAG_BITS_MAX_ENUM;

        /// See [`VkCommandPoolResetFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandPoolResetFlagBits)
        const COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT = vks::VK_COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT;
    }
}

/// See [`VkCommandPoolResetFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandPoolResetFlagBits)
pub type CommandPoolResetFlagBits = CommandPoolResetFlags;

bitflags! {
    /// See [`VkCommandBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferUsageFlagBits)
    #[derive(Default)]
    pub struct CommandBufferUsageFlags: u32 {
        /// See [`VkCommandBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferUsageFlagBits)
        const COMMAND_BUFFER_USAGE_FLAG_BITS_MAX_ENUM = vks::VK_COMMAND_BUFFER_USAGE_FLAG_BITS_MAX_ENUM;

        /// See [`VkCommandBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferUsageFlagBits)
        const COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT = vks::VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT;

        /// See [`VkCommandBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferUsageFlagBits)
        const COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT = vks::VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT;

        /// See [`VkCommandBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferUsageFlagBits)
        const COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT = vks::VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT;
    }
}

/// See [`VkCommandBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferUsageFlagBits)
pub type CommandBufferUsageFlagBits = CommandBufferUsageFlags;

bitflags! {
    /// See [`VkQueryControlFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryControlFlagBits)
    #[derive(Default)]
    pub struct QueryControlFlags: u32 {
        /// See [`VkQueryControlFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryControlFlagBits)
        const QUERY_CONTROL_FLAG_BITS_MAX_ENUM = vks::VK_QUERY_CONTROL_FLAG_BITS_MAX_ENUM;

        /// See [`VkQueryControlFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryControlFlagBits)
        const QUERY_CONTROL_PRECISE_BIT = vks::VK_QUERY_CONTROL_PRECISE_BIT;
    }
}

/// See [`VkQueryControlFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryControlFlagBits)
pub type QueryControlFlagBits = QueryControlFlags;

bitflags! {
    /// See [`VkCommandBufferResetFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferResetFlagBits)
    #[derive(Default)]
    pub struct CommandBufferResetFlags: u32 {
        /// See [`VkCommandBufferResetFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferResetFlagBits)
        const COMMAND_BUFFER_RESET_FLAG_BITS_MAX_ENUM = vks::VK_COMMAND_BUFFER_RESET_FLAG_BITS_MAX_ENUM;

        /// See [`VkCommandBufferResetFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferResetFlagBits)
        const COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT = vks::VK_COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT;
    }
}

/// See [`VkCommandBufferResetFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferResetFlagBits)
pub type CommandBufferResetFlagBits = CommandBufferResetFlags;

bitflags! {
    /// See [`VkStencilFaceFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStencilFaceFlagBits)
    #[derive(Default)]
    pub struct StencilFaceFlags: u32 {
        /// See [`VkStencilFaceFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStencilFaceFlagBits)
        const STENCIL_FACE_FLAG_BITS_MAX_ENUM = vks::VK_STENCIL_FACE_FLAG_BITS_MAX_ENUM;

        /// See [`VkStencilFaceFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStencilFaceFlagBits)
        const STENCIL_FACE_FRONT_BIT = vks::VK_STENCIL_FACE_FRONT_BIT;

        /// See [`VkStencilFaceFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStencilFaceFlagBits)
        const STENCIL_FACE_BACK_BIT = vks::VK_STENCIL_FACE_BACK_BIT;

        /// See [`VkStencilFaceFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStencilFaceFlagBits)
        const STENCIL_FRONT_AND_BACK = vks::VK_STENCIL_FRONT_AND_BACK;
    }
}

/// See [`VkStencilFaceFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStencilFaceFlagBits)
pub type StencilFaceFlagBits = StencilFaceFlags;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum OptionalDeviceSize {
    Size(u64),
    WholeSize,
}

impl From<OptionalDeviceSize> for u64 {
    fn from(size: OptionalDeviceSize) -> Self {
        match size {
            OptionalDeviceSize::Size(size) => size,
            OptionalDeviceSize::WholeSize => vks::VK_WHOLE_SIZE,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum OptionalMipLevels {
    MipLevels(u32),
    Remaining,
}

impl From<OptionalMipLevels> for u32 {
    fn from(mip_levels: OptionalMipLevels) -> Self {
        match mip_levels {
            OptionalMipLevels::MipLevels(mip_levels) => mip_levels,
            OptionalMipLevels::Remaining => vks::VK_REMAINING_MIP_LEVELS,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum OptionalArrayLayers {
    ArrayLayers(u32),
    Remaining,
}

impl From<OptionalArrayLayers> for u32 {
    fn from(array_layers: OptionalArrayLayers) -> Self {
        match array_layers {
            OptionalArrayLayers::ArrayLayers(array_layers) => array_layers,
            OptionalArrayLayers::Remaining => vks::VK_REMAINING_ARRAY_LAYERS,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AttachmentIndex {
    Index(u32),
    Unused,
}

impl From<AttachmentIndex> for u32 {
    fn from(index: AttachmentIndex) -> Self {
        match index {
            AttachmentIndex::Index(index) => index,
            AttachmentIndex::Unused => vks::VK_ATTACHMENT_UNUSED
        }
    }
}

impl From<u32> for AttachmentIndex {
    fn from(index: u32) -> Self {
        match index {
            vks::VK_ATTACHMENT_UNUSED => AttachmentIndex::Unused,
            _ => AttachmentIndex::Index(index),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum QueueFamilyIndex {
    Index(u32),
    Ignored,
}

impl From<QueueFamilyIndex> for u32 {
    fn from(index: QueueFamilyIndex) -> Self {
        match index {
            QueueFamilyIndex::Index(index) => index,
            QueueFamilyIndex::Ignored => vks::VK_QUEUE_FAMILY_IGNORED,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SubpassIndex {
    Index(u32),
    External,
}

impl From<SubpassIndex> for u32 {
    fn from(index: SubpassIndex) -> Self {
        match index {
            SubpassIndex::Index(index) => index,
            SubpassIndex::External => vks::VK_SUBPASS_EXTERNAL,
        }
    }
}

impl From<u32> for SubpassIndex {
    fn from(index: u32) -> Self {
        match index {
            vks::VK_SUBPASS_EXTERNAL => SubpassIndex::External,
            _ => SubpassIndex::Index(index),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum QueryResult {
    U32(u32),
    U64(u64),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Timeout {
    None,
    Some(Duration),
    Infinite,
}

impl Timeout {
    #[inline]
    pub fn as_nanoseconds(&self) -> u64 {
        match *self {
            Timeout::None => 0,
            Timeout::Some(ref d) => 1000000000u64 * d.as_secs() + d.subsec_nanos() as u64,
            Timeout::Infinite => u64::max_value(),
        }
    }
}

/// See [API Version Numbers](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#fundamentals-versionnum)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Version {
    pub fn from_api_version(version: u32) -> Self {
        Version {
            major: vks::vk_version_major(version),
            minor: vks::vk_version_minor(version),
            patch: vks::vk_version_patch(version),
        }
    }

    pub fn from_optional_api_version(version: u32) -> Option<Self> {
        if version != 0 {
            Some(Version::from_api_version(version))
        }
        else {
            None
        }
    }

    pub fn as_api_version(&self) -> u32 {
        vks::vk_make_version(self.major, self.minor, self.patch)
    }

    pub fn api_version_from_optional(version: Option<Version>) -> u32 {
        match version {
            Some(version) => version.as_api_version(),
            None => 0,
        }
    }
}

pub const LOD_CLAMP_NONE: f32 = vks::VK_LOD_CLAMP_NONE;
pub const REMAINING_MIP_LEVELS: u32 = vks::VK_REMAINING_MIP_LEVELS;
pub const REMAINING_ARRAY_LAYERS: u32 = vks::VK_REMAINING_ARRAY_LAYERS;
pub const WHOLE_SIZE: u64 = vks::VK_WHOLE_SIZE;
pub const ATTACHMENT_UNUSED: u32 = vks::VK_ATTACHMENT_UNUSED;
pub const QUEUE_FAMILY_IGNORED: u32 = vks::VK_QUEUE_FAMILY_IGNORED;
pub const SUBPASS_EXTERNAL: u32 = vks::VK_SUBPASS_EXTERNAL;
pub const MAX_PHYSICAL_DEVICE_NAME_SIZE: usize = vks::VK_MAX_PHYSICAL_DEVICE_NAME_SIZE;
pub const UUID_SIZE: usize = vks::VK_UUID_SIZE;
pub const MAX_MEMORY_TYPES: usize = vks::VK_MAX_MEMORY_TYPES;
pub const MAX_MEMORY_HEAPS: usize = vks::VK_MAX_MEMORY_HEAPS;
pub const MAX_EXTENSION_NAME_SIZE: usize = vks::VK_MAX_EXTENSION_NAME_SIZE;
pub const MAX_DESCRIPTION_SIZE: usize = vks::VK_MAX_DESCRIPTION_SIZE;

/// See [`VkPipelineCacheHeaderVersion`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCacheHeaderVersion)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PipelineCacheHeaderVersion {
    One,
    Unknown(vks::VkPipelineCacheHeaderVersion),
}

/// See [`VkResult`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkResult)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Error {
    OutOfHostMemory,
    OutOfDeviceMemory,
    InitializationFailed,
    DeviceLost,
    MemoryMapFailed,
    LayerNotPresent,
    ExtensionNotPresent,
    FeatureNotPresent,
    IncompatibleDriver,
    TooManyObjects,
    FormatNotSupported,

    /// See extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
    SurfaceLostKhr,

    /// See extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
    NativeWindowInUseKhr,

    /// See extension [`VK_EXT_debug_report`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_report)
    ValidationFailedExt,

    /// See extension [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)
    OutOfDateKhr,

    /// See extension [`VK_KHR_display_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display_swapchain)
    IncompatibleDisplayKhr,

    /// See extension [`VK_NV_glsl_shader`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_NV_glsl_shader)
    InvalidShaderNv,

    Unknown(vks::VkResult),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ::std::error::Error::description(self))
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::OutOfHostMemory => "OutOfHostMemory",
            Error::OutOfDeviceMemory => "OutOfDeviceMemory",
            Error::InitializationFailed => "InitializationFailed",
            Error::DeviceLost => "DeviceLost",
            Error::MemoryMapFailed => "MemoryMapFailed",
            Error::LayerNotPresent => "LayerNotPresent",
            Error::ExtensionNotPresent => "ExtensionNotPresent",
            Error::FeatureNotPresent => "FeatureNotPresent",
            Error::IncompatibleDriver => "IncompatibleDriver",
            Error::TooManyObjects => "TooManyObjects",
            Error::FormatNotSupported => "FormatNotSupported",
            Error::SurfaceLostKhr => "SurfaceLost",
            Error::NativeWindowInUseKhr => "NativeWindowInUse",
            Error::ValidationFailedExt => "ValidationFailed",
            Error::OutOfDateKhr => "OutOfDate",
            Error::IncompatibleDisplayKhr => "IncompatibleDisplay",
            Error::InvalidShaderNv => "InvalidShader",
            Error::Unknown(_) => "unknown error",
        }
    }
}

impl From<vks::VkResult> for Error {
    fn from(res: vks::VkResult) -> Self {
        debug_assert!(res < 0);

        match res {
            vks::VK_ERROR_OUT_OF_HOST_MEMORY => Error::OutOfHostMemory,
            vks::VK_ERROR_OUT_OF_DEVICE_MEMORY => Error::OutOfDeviceMemory,
            vks::VK_ERROR_INITIALIZATION_FAILED => Error::InitializationFailed,
            vks::VK_ERROR_DEVICE_LOST => Error::DeviceLost,
            vks::VK_ERROR_MEMORY_MAP_FAILED => Error::MemoryMapFailed,
            vks::VK_ERROR_LAYER_NOT_PRESENT => Error::LayerNotPresent,
            vks::VK_ERROR_EXTENSION_NOT_PRESENT => Error::ExtensionNotPresent,
            vks::VK_ERROR_FEATURE_NOT_PRESENT => Error::FeatureNotPresent,
            vks::VK_ERROR_INCOMPATIBLE_DRIVER => Error::IncompatibleDriver,
            vks::VK_ERROR_TOO_MANY_OBJECTS => Error::TooManyObjects,
            vks::VK_ERROR_FORMAT_NOT_SUPPORTED => Error::FormatNotSupported,
            vks::VK_ERROR_SURFACE_LOST_KHR => Error::SurfaceLostKhr,
            vks::VK_ERROR_NATIVE_WINDOW_IN_USE_KHR => Error::NativeWindowInUseKhr,
            vks::VK_ERROR_VALIDATION_FAILED_EXT => Error::ValidationFailedExt,
            vks::VK_ERROR_OUT_OF_DATE_KHR => Error::OutOfDateKhr,
            vks::VK_ERROR_INCOMPATIBLE_DISPLAY_KHR => Error::IncompatibleDisplayKhr,
            vks::VK_ERROR_INVALID_SHADER_NV => Error::InvalidShaderNv,
            _ => Error::Unknown(res),
        }
    }
}

impl From<Error> for vks::VkResult {
    fn from(e: Error) -> Self {
        match e {
            Error::OutOfHostMemory => vks::VK_ERROR_OUT_OF_HOST_MEMORY,
            Error::OutOfDeviceMemory => vks::VK_ERROR_OUT_OF_DEVICE_MEMORY,
            Error::InitializationFailed => vks::VK_ERROR_INITIALIZATION_FAILED,
            Error::DeviceLost => vks::VK_ERROR_DEVICE_LOST,
            Error::MemoryMapFailed => vks::VK_ERROR_MEMORY_MAP_FAILED,
            Error::LayerNotPresent => vks::VK_ERROR_LAYER_NOT_PRESENT,
            Error::ExtensionNotPresent => vks::VK_ERROR_EXTENSION_NOT_PRESENT,
            Error::FeatureNotPresent => vks::VK_ERROR_FEATURE_NOT_PRESENT,
            Error::IncompatibleDriver => vks::VK_ERROR_INCOMPATIBLE_DRIVER,
            Error::TooManyObjects => vks::VK_ERROR_TOO_MANY_OBJECTS,
            Error::FormatNotSupported => vks::VK_ERROR_FORMAT_NOT_SUPPORTED,
            Error::SurfaceLostKhr => vks::VK_ERROR_SURFACE_LOST_KHR,
            Error::NativeWindowInUseKhr => vks::VK_ERROR_NATIVE_WINDOW_IN_USE_KHR,
            Error::ValidationFailedExt => vks::VK_ERROR_VALIDATION_FAILED_EXT,
            Error::OutOfDateKhr => vks::VK_ERROR_OUT_OF_DATE_KHR,
            Error::IncompatibleDisplayKhr => vks::VK_ERROR_INCOMPATIBLE_DISPLAY_KHR,
            Error::InvalidShaderNv => vks::VK_ERROR_INVALID_SHADER_NV,
            Error::Unknown(res) => res,
        }
    }
}

/// See [`VkSystemAllocationSope`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSystemAllocationSope)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SystemAllocationSope {
    Command,
    Object,
    Cache,
    Device,
    Instance,
    Unknown(vks::VkSystemAllocationScope),
}

impl From<vks::VkSystemAllocationScope> for SystemAllocationSope {
    fn from(scope: vks::VkSystemAllocationScope) -> Self {
        match scope {
            vks::VK_SYSTEM_ALLOCATION_SCOPE_COMMAND => SystemAllocationSope::Command,
            vks::VK_SYSTEM_ALLOCATION_SCOPE_OBJECT => SystemAllocationSope::Object,
            vks::VK_SYSTEM_ALLOCATION_SCOPE_CACHE => SystemAllocationSope::Cache,
            vks::VK_SYSTEM_ALLOCATION_SCOPE_DEVICE => SystemAllocationSope::Device,
            vks::VK_SYSTEM_ALLOCATION_SCOPE_INSTANCE => SystemAllocationSope::Instance,
            _ => SystemAllocationSope::Unknown(scope),
        }
    }
}

/// See [`VkInternalAllocationType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkInternalAllocationType)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum InternalAllocationType {
    Executable,
    Unknown(vks::VkInternalAllocationType),
}

impl From<vks::VkInternalAllocationType> for InternalAllocationType {
    fn from(allocation_type: vks::VkInternalAllocationType) -> Self {
        match allocation_type {
            vks::VK_INTERNAL_ALLOCATION_TYPE_EXECUTABLE => InternalAllocationType::Executable,
            _ => InternalAllocationType::Unknown(allocation_type),
        }
    }
}

/// See [`VkFormat`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormat)
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Format {
    Undefined,
    R4G4_UNorm_Pack8,
    R4G4B4A4_UNorm_Pack16,
    B4G4R4A4_UNorm_Pack16,
    R5G6B5_UNorm_Pack16,
    B5G6R5_UNorm_Pack16,
    R5G5B5A1_UNorm_Pack16,
    B5G5R5A1_UNorm_Pack16,
    A1R5G5B5_UNorm_Pack16,
    R8_UNorm,
    R8_SNorm,
    R8_UScaled,
    R8_SScaled,
    R8_UInt,
    R8_SInt,
    R8_sRGB,
    R8G8_UNorm,
    R8G8_SNorm,
    R8G8_UScaled,
    R8G8_SScaled,
    R8G8_UInt,
    R8G8_SInt,
    R8G8_sRGB,
    R8G8B8_UNorm,
    R8G8B8_SNorm,
    R8G8B8_UScaled,
    R8G8B8_SScaled,
    R8G8B8_UInt,
    R8G8B8_SInt,
    R8G8B8_sRGB,
    B8G8R8_UNorm,
    B8G8R8_SNorm,
    B8G8R8_UScaled,
    B8G8R8_SScaled,
    B8G8R8_UInt,
    B8G8R8_SInt,
    B8G8R8_sRGB,
    R8G8B8A8_UNorm,
    R8G8B8A8_SNorm,
    R8G8B8A8_UScaled,
    R8G8B8A8_SScaled,
    R8G8B8A8_UInt,
    R8G8B8A8_SInt,
    R8G8B8A8_sRGB,
    B8G8R8A8_UNorm,
    B8G8R8A8_SNorm,
    B8G8R8A8_UScaled,
    B8G8R8A8_SScaled,
    B8G8R8A8_UInt,
    B8G8R8A8_SInt,
    B8G8R8A8_sRGB,
    A8B8G8R8_UNorm_Pack32,
    A8B8G8R8_SNorm_Pack32,
    A8B8G8R8_UScaled_Pack32,
    A8B8G8R8_SScaled_Pack32,
    A8B8G8R8_UInt_Pack32,
    A8B8G8R8_SInt_Pack32,
    A8B8G8R8_sRGB_Pack32,
    A2R10G10B10_UNorm_Pack32,
    A2R10G10B10_SNorm_Pack32,
    A2R10G10B10_UScaled_Pack32,
    A2R10G10B10_SScaled_Pack32,
    A2R10G10B10_UInt_Pack32,
    A2R10G10B10_SInt_Pack32,
    A2B10G10R10_UNorm_Pack32,
    A2B10G10R10_SNorm_Pack32,
    A2B10G10R10_UScaled_Pack32,
    A2B10G10R10_SScaled_Pack32,
    A2B10G10R10_UInt_Pack32,
    A2B10G10R10_SInt_Pack32,
    R16_UNorm,
    R16_SNorm,
    R16_UScaled,
    R16_SScaled,
    R16_UInt,
    R16_SInt,
    R16_SFloat,
    R16G16_UNorm,
    R16G16_SNorm,
    R16G16_UScaled,
    R16G16_SScaled,
    R16G16_UInt,
    R16G16_SInt,
    R16G16_SFloat,
    R16G16B16_UNorm,
    R16G16B16_SNorm,
    R16G16B16_UScaled,
    R16G16B16_SScaled,
    R16G16B16_UInt,
    R16G16B16_SInt,
    R16G16B16_SFloat,
    R16G16B16A16_UNorm,
    R16G16B16A16_SNorm,
    R16G16B16A16_UScaled,
    R16G16B16A16_SScaled,
    R16G16B16A16_UInt,
    R16G16B16A16_SInt,
    R16G16B16A16_SFloat,
    R32_UInt,
    R32_SInt,
    R32_SFloat,
    R32G32_UInt,
    R32G32_SInt,
    R32G32_SFloat,
    R32G32B32_UInt,
    R32G32B32_SInt,
    R32G32B32_SFloat,
    R32G32B32A32_UInt,
    R32G32B32A32_SInt,
    R32G32B32A32_SFloat,
    R64_UInt,
    R64_SInt,
    R64_SFloat,
    R64G64_UInt,
    R64G64_SInt,
    R64G64_SFloat,
    R64G64B64_UInt,
    R64G64B64_SInt,
    R64G64B64_SFloat,
    R64G64B64A64_UInt,
    R64G64B64A64_SInt,
    R64G64B64A64_SFloat,
    B10G11R11_UFloat_Pack32,
    E5B9G9R9_UFloat_Pack32,
    D16_UNorm,
    X8_D24_UNorm_Pack32,
    D32_SFloat,
    S8_UInt,
    D16_UNorm_S8_UInt,
    D24_UNorm_S8_UInt,
    D32_SFloat_S8_UInt,
    BC1_RGB_UNorm_Block,
    BC1_RGB_sRGB_Block,
    BC1_RGBA_UNorm_Block,
    BC1_RGBA_sRGB_Block,
    BC2_UNorm_Block,
    BC2_sRGB_Block,
    BC3_UNorm_Block,
    BC3_sRGB_Block,
    BC4_UNorm_Block,
    BC4_SNorm_Block,
    BC5_UNorm_Block,
    BC5_SNorm_Block,
    BC6H_UFloat_Block,
    BC6H_SFloat_Block,
    BC7_UNorm_Block,
    BC7_sRGB_Block,
    ETC2_R8G8B8_UNorm_Block,
    ETC2_R8G8B8_sRGB_Block,
    ETC2_R8G8B8A1_UNorm_Block,
    ETC2_R8G8B8A1_sRGB_Block,
    ETC2_R8G8B8A8_UNorm_Block,
    ETC2_R8G8B8A8_sRGB_Block,
    EAC_R11_UNorm_Block,
    EAC_R11_SNorm_Block,
    EAC_R11G11_UNorm_Block,
    EAC_R11G11_SNorm_Block,
    ASTC_4x4_UNorm_Block,
    ASTC_4x4_sRGB_Block,
    ASTC_5x4_UNorm_Block,
    ASTC_5x4_sRGB_Block,
    ASTC_5x5_UNorm_Block,
    ASTC_5x5_sRGB_Block,
    ASTC_6x5_UNorm_Block,
    ASTC_6x5_sRGB_Block,
    ASTC_6x6_UNorm_Block,
    ASTC_6x6_sRGB_Block,
    ASTC_8x5_UNorm_Block,
    ASTC_8x5_sRGB_Block,
    ASTC_8x6_UNorm_Block,
    ASTC_8x6_sRGB_Block,
    ASTC_8x8_UNorm_Block,
    ASTC_8x8_sRGB_Block,
    ASTC_10x5_UNorm_Block,
    ASTC_10x5_sRGB_Block,
    ASTC_10x6_UNorm_Block,
    ASTC_10x6_sRGB_Block,
    ASTC_10x8_UNorm_Block,
    ASTC_10x8_sRGB_Block,
    ASTC_10x10_UNorm_Block,
    ASTC_10x10_sRGB_Block,
    ASTC_12x10_UNorm_Block,
    ASTC_12x10_sRGB_Block,
    ASTC_12x12_UNorm_Block,
    ASTC_12x12_sRGB_Block,
    Unknown(vks::VkFormat),
}

impl From<vks::VkFormat> for Format {
    fn from(format: vks::VkFormat) -> Self {
        match format {
            vks::VK_FORMAT_UNDEFINED => Format::Undefined,
            vks::VK_FORMAT_R4G4_UNORM_PACK8 => Format::R4G4_UNorm_Pack8,
            vks::VK_FORMAT_R4G4B4A4_UNORM_PACK16 => Format::R4G4B4A4_UNorm_Pack16,
            vks::VK_FORMAT_B4G4R4A4_UNORM_PACK16 => Format::B4G4R4A4_UNorm_Pack16,
            vks::VK_FORMAT_R5G6B5_UNORM_PACK16 => Format::R5G6B5_UNorm_Pack16,
            vks::VK_FORMAT_B5G6R5_UNORM_PACK16 => Format::B5G6R5_UNorm_Pack16,
            vks::VK_FORMAT_R5G5B5A1_UNORM_PACK16 => Format::R5G5B5A1_UNorm_Pack16,
            vks::VK_FORMAT_B5G5R5A1_UNORM_PACK16 => Format::B5G5R5A1_UNorm_Pack16,
            vks::VK_FORMAT_A1R5G5B5_UNORM_PACK16 => Format::A1R5G5B5_UNorm_Pack16,
            vks::VK_FORMAT_R8_UNORM => Format::R8_UNorm,
            vks::VK_FORMAT_R8_SNORM => Format::R8_SNorm,
            vks::VK_FORMAT_R8_USCALED => Format::R8_UScaled,
            vks::VK_FORMAT_R8_SSCALED => Format::R8_SScaled,
            vks::VK_FORMAT_R8_UINT => Format::R8_UInt,
            vks::VK_FORMAT_R8_SINT => Format::R8_SInt,
            vks::VK_FORMAT_R8_SRGB => Format::R8_sRGB,
            vks::VK_FORMAT_R8G8_UNORM => Format::R8G8_UNorm,
            vks::VK_FORMAT_R8G8_SNORM => Format::R8G8_SNorm,
            vks::VK_FORMAT_R8G8_USCALED => Format::R8G8_UScaled,
            vks::VK_FORMAT_R8G8_SSCALED => Format::R8G8_SScaled,
            vks::VK_FORMAT_R8G8_UINT => Format::R8G8_UInt,
            vks::VK_FORMAT_R8G8_SINT => Format::R8G8_SInt,
            vks::VK_FORMAT_R8G8_SRGB => Format::R8G8_sRGB,
            vks::VK_FORMAT_R8G8B8_UNORM => Format::R8G8B8_UNorm,
            vks::VK_FORMAT_R8G8B8_SNORM => Format::R8G8B8_SNorm,
            vks::VK_FORMAT_R8G8B8_USCALED => Format::R8G8B8_UScaled,
            vks::VK_FORMAT_R8G8B8_SSCALED => Format::R8G8B8_SScaled,
            vks::VK_FORMAT_R8G8B8_UINT => Format::R8G8B8_UInt,
            vks::VK_FORMAT_R8G8B8_SINT => Format::R8G8B8_SInt,
            vks::VK_FORMAT_R8G8B8_SRGB => Format::R8G8B8_sRGB,
            vks::VK_FORMAT_B8G8R8_UNORM => Format::B8G8R8_UNorm,
            vks::VK_FORMAT_B8G8R8_SNORM => Format::B8G8R8_SNorm,
            vks::VK_FORMAT_B8G8R8_USCALED => Format::B8G8R8_UScaled,
            vks::VK_FORMAT_B8G8R8_SSCALED => Format::B8G8R8_SScaled,
            vks::VK_FORMAT_B8G8R8_UINT => Format::B8G8R8_UInt,
            vks::VK_FORMAT_B8G8R8_SINT => Format::B8G8R8_SInt,
            vks::VK_FORMAT_B8G8R8_SRGB => Format::B8G8R8_sRGB,
            vks::VK_FORMAT_R8G8B8A8_UNORM => Format::R8G8B8A8_UNorm,
            vks::VK_FORMAT_R8G8B8A8_SNORM => Format::R8G8B8A8_SNorm,
            vks::VK_FORMAT_R8G8B8A8_USCALED => Format::R8G8B8A8_UScaled,
            vks::VK_FORMAT_R8G8B8A8_SSCALED => Format::R8G8B8A8_SScaled,
            vks::VK_FORMAT_R8G8B8A8_UINT => Format::R8G8B8A8_UInt,
            vks::VK_FORMAT_R8G8B8A8_SINT => Format::R8G8B8A8_SInt,
            vks::VK_FORMAT_R8G8B8A8_SRGB => Format::R8G8B8A8_sRGB,
            vks::VK_FORMAT_B8G8R8A8_UNORM => Format::B8G8R8A8_UNorm,
            vks::VK_FORMAT_B8G8R8A8_SNORM => Format::B8G8R8A8_SNorm,
            vks::VK_FORMAT_B8G8R8A8_USCALED => Format::B8G8R8A8_UScaled,
            vks::VK_FORMAT_B8G8R8A8_SSCALED => Format::B8G8R8A8_SScaled,
            vks::VK_FORMAT_B8G8R8A8_UINT => Format::B8G8R8A8_UInt,
            vks::VK_FORMAT_B8G8R8A8_SINT => Format::B8G8R8A8_SInt,
            vks::VK_FORMAT_B8G8R8A8_SRGB => Format::B8G8R8A8_sRGB,
            vks::VK_FORMAT_A8B8G8R8_UNORM_PACK32 => Format::A8B8G8R8_UNorm_Pack32,
            vks::VK_FORMAT_A8B8G8R8_SNORM_PACK32 => Format::A8B8G8R8_SNorm_Pack32,
            vks::VK_FORMAT_A8B8G8R8_USCALED_PACK32 => Format::A8B8G8R8_UScaled_Pack32,
            vks::VK_FORMAT_A8B8G8R8_SSCALED_PACK32 => Format::A8B8G8R8_SScaled_Pack32,
            vks::VK_FORMAT_A8B8G8R8_UINT_PACK32 => Format::A8B8G8R8_UInt_Pack32,
            vks::VK_FORMAT_A8B8G8R8_SINT_PACK32 => Format::A8B8G8R8_SInt_Pack32,
            vks::VK_FORMAT_A8B8G8R8_SRGB_PACK32 => Format::A8B8G8R8_sRGB_Pack32,
            vks::VK_FORMAT_A2R10G10B10_UNORM_PACK32 => Format::A2R10G10B10_UNorm_Pack32,
            vks::VK_FORMAT_A2R10G10B10_SNORM_PACK32 => Format::A2R10G10B10_SNorm_Pack32,
            vks::VK_FORMAT_A2R10G10B10_USCALED_PACK32 => Format::A2R10G10B10_UScaled_Pack32,
            vks::VK_FORMAT_A2R10G10B10_SSCALED_PACK32 => Format::A2R10G10B10_SScaled_Pack32,
            vks::VK_FORMAT_A2R10G10B10_UINT_PACK32 => Format::A2R10G10B10_UInt_Pack32,
            vks::VK_FORMAT_A2R10G10B10_SINT_PACK32 => Format::A2R10G10B10_SInt_Pack32,
            vks::VK_FORMAT_A2B10G10R10_UNORM_PACK32 => Format::A2B10G10R10_UNorm_Pack32,
            vks::VK_FORMAT_A2B10G10R10_SNORM_PACK32 => Format::A2B10G10R10_SNorm_Pack32,
            vks::VK_FORMAT_A2B10G10R10_USCALED_PACK32 => Format::A2B10G10R10_UScaled_Pack32,
            vks::VK_FORMAT_A2B10G10R10_SSCALED_PACK32 => Format::A2B10G10R10_SScaled_Pack32,
            vks::VK_FORMAT_A2B10G10R10_UINT_PACK32 => Format::A2B10G10R10_UInt_Pack32,
            vks::VK_FORMAT_A2B10G10R10_SINT_PACK32 => Format::A2B10G10R10_SInt_Pack32,
            vks::VK_FORMAT_R16_UNORM => Format::R16_UNorm,
            vks::VK_FORMAT_R16_SNORM => Format::R16_SNorm,
            vks::VK_FORMAT_R16_USCALED => Format::R16_UScaled,
            vks::VK_FORMAT_R16_SSCALED => Format::R16_SScaled,
            vks::VK_FORMAT_R16_UINT => Format::R16_UInt,
            vks::VK_FORMAT_R16_SINT => Format::R16_SInt,
            vks::VK_FORMAT_R16_SFLOAT => Format::R16_SFloat,
            vks::VK_FORMAT_R16G16_UNORM => Format::R16G16_UNorm,
            vks::VK_FORMAT_R16G16_SNORM => Format::R16G16_SNorm,
            vks::VK_FORMAT_R16G16_USCALED => Format::R16G16_UScaled,
            vks::VK_FORMAT_R16G16_SSCALED => Format::R16G16_SScaled,
            vks::VK_FORMAT_R16G16_UINT => Format::R16G16_UInt,
            vks::VK_FORMAT_R16G16_SINT => Format::R16G16_SInt,
            vks::VK_FORMAT_R16G16_SFLOAT => Format::R16G16_SFloat,
            vks::VK_FORMAT_R16G16B16_UNORM => Format::R16G16B16_UNorm,
            vks::VK_FORMAT_R16G16B16_SNORM => Format::R16G16B16_SNorm,
            vks::VK_FORMAT_R16G16B16_USCALED => Format::R16G16B16_UScaled,
            vks::VK_FORMAT_R16G16B16_SSCALED => Format::R16G16B16_SScaled,
            vks::VK_FORMAT_R16G16B16_UINT => Format::R16G16B16_UInt,
            vks::VK_FORMAT_R16G16B16_SINT => Format::R16G16B16_SInt,
            vks::VK_FORMAT_R16G16B16_SFLOAT => Format::R16G16B16_SFloat,
            vks::VK_FORMAT_R16G16B16A16_UNORM => Format::R16G16B16A16_UNorm,
            vks::VK_FORMAT_R16G16B16A16_SNORM => Format::R16G16B16A16_SNorm,
            vks::VK_FORMAT_R16G16B16A16_USCALED => Format::R16G16B16A16_UScaled,
            vks::VK_FORMAT_R16G16B16A16_SSCALED => Format::R16G16B16A16_SScaled,
            vks::VK_FORMAT_R16G16B16A16_UINT => Format::R16G16B16A16_UInt,
            vks::VK_FORMAT_R16G16B16A16_SINT => Format::R16G16B16A16_SInt,
            vks::VK_FORMAT_R16G16B16A16_SFLOAT => Format::R16G16B16A16_SFloat,
            vks::VK_FORMAT_R32_UINT => Format::R32_UInt,
            vks::VK_FORMAT_R32_SINT => Format::R32_SInt,
            vks::VK_FORMAT_R32_SFLOAT => Format::R32_SFloat,
            vks::VK_FORMAT_R32G32_UINT => Format::R32G32_UInt,
            vks::VK_FORMAT_R32G32_SINT => Format::R32G32_SInt,
            vks::VK_FORMAT_R32G32_SFLOAT => Format::R32G32_SFloat,
            vks::VK_FORMAT_R32G32B32_UINT => Format::R32G32B32_UInt,
            vks::VK_FORMAT_R32G32B32_SINT => Format::R32G32B32_SInt,
            vks::VK_FORMAT_R32G32B32_SFLOAT => Format::R32G32B32_SFloat,
            vks::VK_FORMAT_R32G32B32A32_UINT => Format::R32G32B32A32_UInt,
            vks::VK_FORMAT_R32G32B32A32_SINT => Format::R32G32B32A32_SInt,
            vks::VK_FORMAT_R32G32B32A32_SFLOAT => Format::R32G32B32A32_SFloat,
            vks::VK_FORMAT_R64_UINT => Format::R64_UInt,
            vks::VK_FORMAT_R64_SINT => Format::R64_SInt,
            vks::VK_FORMAT_R64_SFLOAT => Format::R64_SFloat,
            vks::VK_FORMAT_R64G64_UINT => Format::R64G64_UInt,
            vks::VK_FORMAT_R64G64_SINT => Format::R64G64_SInt,
            vks::VK_FORMAT_R64G64_SFLOAT => Format::R64G64_SFloat,
            vks::VK_FORMAT_R64G64B64_UINT => Format::R64G64B64_UInt,
            vks::VK_FORMAT_R64G64B64_SINT => Format::R64G64B64_SInt,
            vks::VK_FORMAT_R64G64B64_SFLOAT => Format::R64G64B64_SFloat,
            vks::VK_FORMAT_R64G64B64A64_UINT => Format::R64G64B64A64_UInt,
            vks::VK_FORMAT_R64G64B64A64_SINT => Format::R64G64B64A64_SInt,
            vks::VK_FORMAT_R64G64B64A64_SFLOAT => Format::R64G64B64A64_SFloat,
            vks::VK_FORMAT_B10G11R11_UFLOAT_PACK32 => Format::B10G11R11_UFloat_Pack32,
            vks::VK_FORMAT_E5B9G9R9_UFLOAT_PACK32 => Format::E5B9G9R9_UFloat_Pack32,
            vks::VK_FORMAT_D16_UNORM => Format::D16_UNorm,
            vks::VK_FORMAT_X8_D24_UNORM_PACK32 => Format::X8_D24_UNorm_Pack32,
            vks::VK_FORMAT_D32_SFLOAT => Format::D32_SFloat,
            vks::VK_FORMAT_S8_UINT => Format::S8_UInt,
            vks::VK_FORMAT_D16_UNORM_S8_UINT => Format::D16_UNorm_S8_UInt,
            vks::VK_FORMAT_D24_UNORM_S8_UINT => Format::D24_UNorm_S8_UInt,
            vks::VK_FORMAT_D32_SFLOAT_S8_UINT => Format::D32_SFloat_S8_UInt,
            vks::VK_FORMAT_BC1_RGB_UNORM_BLOCK => Format::BC1_RGB_UNorm_Block,
            vks::VK_FORMAT_BC1_RGB_SRGB_BLOCK => Format::BC1_RGB_sRGB_Block,
            vks::VK_FORMAT_BC1_RGBA_UNORM_BLOCK => Format::BC1_RGBA_UNorm_Block,
            vks::VK_FORMAT_BC1_RGBA_SRGB_BLOCK => Format::BC1_RGBA_sRGB_Block,
            vks::VK_FORMAT_BC2_UNORM_BLOCK => Format::BC2_UNorm_Block,
            vks::VK_FORMAT_BC2_SRGB_BLOCK => Format::BC2_sRGB_Block,
            vks::VK_FORMAT_BC3_UNORM_BLOCK => Format::BC3_UNorm_Block,
            vks::VK_FORMAT_BC3_SRGB_BLOCK => Format::BC3_sRGB_Block,
            vks::VK_FORMAT_BC4_UNORM_BLOCK => Format::BC4_UNorm_Block,
            vks::VK_FORMAT_BC4_SNORM_BLOCK => Format::BC4_SNorm_Block,
            vks::VK_FORMAT_BC5_UNORM_BLOCK => Format::BC5_UNorm_Block,
            vks::VK_FORMAT_BC5_SNORM_BLOCK => Format::BC5_SNorm_Block,
            vks::VK_FORMAT_BC6H_UFLOAT_BLOCK => Format::BC6H_UFloat_Block,
            vks::VK_FORMAT_BC6H_SFLOAT_BLOCK => Format::BC6H_SFloat_Block,
            vks::VK_FORMAT_BC7_UNORM_BLOCK => Format::BC7_UNorm_Block,
            vks::VK_FORMAT_BC7_SRGB_BLOCK => Format::BC7_sRGB_Block,
            vks::VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK => Format::ETC2_R8G8B8_UNorm_Block,
            vks::VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK => Format::ETC2_R8G8B8_sRGB_Block,
            vks::VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK => Format::ETC2_R8G8B8A1_UNorm_Block,
            vks::VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK => Format::ETC2_R8G8B8A1_sRGB_Block,
            vks::VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK => Format::ETC2_R8G8B8A8_UNorm_Block,
            vks::VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK => Format::ETC2_R8G8B8A8_sRGB_Block,
            vks::VK_FORMAT_EAC_R11_UNORM_BLOCK => Format::EAC_R11_UNorm_Block,
            vks::VK_FORMAT_EAC_R11_SNORM_BLOCK => Format::EAC_R11_SNorm_Block,
            vks::VK_FORMAT_EAC_R11G11_UNORM_BLOCK => Format::EAC_R11G11_UNorm_Block,
            vks::VK_FORMAT_EAC_R11G11_SNORM_BLOCK => Format::EAC_R11G11_SNorm_Block,
            vks::VK_FORMAT_ASTC_4x4_UNORM_BLOCK => Format::ASTC_4x4_UNorm_Block,
            vks::VK_FORMAT_ASTC_4x4_SRGB_BLOCK => Format::ASTC_4x4_sRGB_Block,
            vks::VK_FORMAT_ASTC_5x4_UNORM_BLOCK => Format::ASTC_5x4_UNorm_Block,
            vks::VK_FORMAT_ASTC_5x4_SRGB_BLOCK => Format::ASTC_5x4_sRGB_Block,
            vks::VK_FORMAT_ASTC_5x5_UNORM_BLOCK => Format::ASTC_5x5_UNorm_Block,
            vks::VK_FORMAT_ASTC_5x5_SRGB_BLOCK => Format::ASTC_5x5_sRGB_Block,
            vks::VK_FORMAT_ASTC_6x5_UNORM_BLOCK => Format::ASTC_6x5_UNorm_Block,
            vks::VK_FORMAT_ASTC_6x5_SRGB_BLOCK => Format::ASTC_6x5_sRGB_Block,
            vks::VK_FORMAT_ASTC_6x6_UNORM_BLOCK => Format::ASTC_6x6_UNorm_Block,
            vks::VK_FORMAT_ASTC_6x6_SRGB_BLOCK => Format::ASTC_6x6_sRGB_Block,
            vks::VK_FORMAT_ASTC_8x5_UNORM_BLOCK => Format::ASTC_8x5_UNorm_Block,
            vks::VK_FORMAT_ASTC_8x5_SRGB_BLOCK => Format::ASTC_8x5_sRGB_Block,
            vks::VK_FORMAT_ASTC_8x6_UNORM_BLOCK => Format::ASTC_8x6_UNorm_Block,
            vks::VK_FORMAT_ASTC_8x6_SRGB_BLOCK => Format::ASTC_8x6_sRGB_Block,
            vks::VK_FORMAT_ASTC_8x8_UNORM_BLOCK => Format::ASTC_8x8_UNorm_Block,
            vks::VK_FORMAT_ASTC_8x8_SRGB_BLOCK => Format::ASTC_8x8_sRGB_Block,
            vks::VK_FORMAT_ASTC_10x5_UNORM_BLOCK => Format::ASTC_10x5_UNorm_Block,
            vks::VK_FORMAT_ASTC_10x5_SRGB_BLOCK => Format::ASTC_10x5_sRGB_Block,
            vks::VK_FORMAT_ASTC_10x6_UNORM_BLOCK => Format::ASTC_10x6_UNorm_Block,
            vks::VK_FORMAT_ASTC_10x6_SRGB_BLOCK => Format::ASTC_10x6_sRGB_Block,
            vks::VK_FORMAT_ASTC_10x8_UNORM_BLOCK => Format::ASTC_10x8_UNorm_Block,
            vks::VK_FORMAT_ASTC_10x8_SRGB_BLOCK => Format::ASTC_10x8_sRGB_Block,
            vks::VK_FORMAT_ASTC_10x10_UNORM_BLOCK => Format::ASTC_10x10_UNorm_Block,
            vks::VK_FORMAT_ASTC_10x10_SRGB_BLOCK => Format::ASTC_10x10_sRGB_Block,
            vks::VK_FORMAT_ASTC_12x10_UNORM_BLOCK => Format::ASTC_12x10_UNorm_Block,
            vks::VK_FORMAT_ASTC_12x10_SRGB_BLOCK => Format::ASTC_12x10_sRGB_Block,
            vks::VK_FORMAT_ASTC_12x12_UNORM_BLOCK => Format::ASTC_12x12_UNorm_Block,
            vks::VK_FORMAT_ASTC_12x12_SRGB_BLOCK => Format::ASTC_12x12_sRGB_Block,
            _ => Format::Unknown(format),
        }
    }
}

impl From<Format> for vks::VkFormat {
    fn from(format: Format) -> Self {
        match format {
            Format::Undefined => vks::VK_FORMAT_UNDEFINED,
            Format::R4G4_UNorm_Pack8 => vks::VK_FORMAT_R4G4_UNORM_PACK8,
            Format::R4G4B4A4_UNorm_Pack16 => vks::VK_FORMAT_R4G4B4A4_UNORM_PACK16,
            Format::B4G4R4A4_UNorm_Pack16 => vks::VK_FORMAT_B4G4R4A4_UNORM_PACK16,
            Format::R5G6B5_UNorm_Pack16 => vks::VK_FORMAT_R5G6B5_UNORM_PACK16,
            Format::B5G6R5_UNorm_Pack16 => vks::VK_FORMAT_B5G6R5_UNORM_PACK16,
            Format::R5G5B5A1_UNorm_Pack16 => vks::VK_FORMAT_R5G5B5A1_UNORM_PACK16,
            Format::B5G5R5A1_UNorm_Pack16 => vks::VK_FORMAT_B5G5R5A1_UNORM_PACK16,
            Format::A1R5G5B5_UNorm_Pack16 => vks::VK_FORMAT_A1R5G5B5_UNORM_PACK16,
            Format::R8_UNorm => vks::VK_FORMAT_R8_UNORM,
            Format::R8_SNorm => vks::VK_FORMAT_R8_SNORM,
            Format::R8_UScaled => vks::VK_FORMAT_R8_USCALED,
            Format::R8_SScaled => vks::VK_FORMAT_R8_SSCALED,
            Format::R8_UInt => vks::VK_FORMAT_R8_UINT,
            Format::R8_SInt => vks::VK_FORMAT_R8_SINT,
            Format::R8_sRGB => vks::VK_FORMAT_R8_SRGB,
            Format::R8G8_UNorm => vks::VK_FORMAT_R8G8_UNORM,
            Format::R8G8_SNorm => vks::VK_FORMAT_R8G8_SNORM,
            Format::R8G8_UScaled => vks::VK_FORMAT_R8G8_USCALED,
            Format::R8G8_SScaled => vks::VK_FORMAT_R8G8_SSCALED,
            Format::R8G8_UInt => vks::VK_FORMAT_R8G8_UINT,
            Format::R8G8_SInt => vks::VK_FORMAT_R8G8_SINT,
            Format::R8G8_sRGB => vks::VK_FORMAT_R8G8_SRGB,
            Format::R8G8B8_UNorm => vks::VK_FORMAT_R8G8B8_UNORM,
            Format::R8G8B8_SNorm => vks::VK_FORMAT_R8G8B8_SNORM,
            Format::R8G8B8_UScaled => vks::VK_FORMAT_R8G8B8_USCALED,
            Format::R8G8B8_SScaled => vks::VK_FORMAT_R8G8B8_SSCALED,
            Format::R8G8B8_UInt => vks::VK_FORMAT_R8G8B8_UINT,
            Format::R8G8B8_SInt => vks::VK_FORMAT_R8G8B8_SINT,
            Format::R8G8B8_sRGB => vks::VK_FORMAT_R8G8B8_SRGB,
            Format::B8G8R8_UNorm => vks::VK_FORMAT_B8G8R8_UNORM,
            Format::B8G8R8_SNorm => vks::VK_FORMAT_B8G8R8_SNORM,
            Format::B8G8R8_UScaled => vks::VK_FORMAT_B8G8R8_USCALED,
            Format::B8G8R8_SScaled => vks::VK_FORMAT_B8G8R8_SSCALED,
            Format::B8G8R8_UInt => vks::VK_FORMAT_B8G8R8_UINT,
            Format::B8G8R8_SInt => vks::VK_FORMAT_B8G8R8_SINT,
            Format::B8G8R8_sRGB => vks::VK_FORMAT_B8G8R8_SRGB,
            Format::R8G8B8A8_UNorm => vks::VK_FORMAT_R8G8B8A8_UNORM,
            Format::R8G8B8A8_SNorm => vks::VK_FORMAT_R8G8B8A8_SNORM,
            Format::R8G8B8A8_UScaled => vks::VK_FORMAT_R8G8B8A8_USCALED,
            Format::R8G8B8A8_SScaled => vks::VK_FORMAT_R8G8B8A8_SSCALED,
            Format::R8G8B8A8_UInt => vks::VK_FORMAT_R8G8B8A8_UINT,
            Format::R8G8B8A8_SInt => vks::VK_FORMAT_R8G8B8A8_SINT,
            Format::R8G8B8A8_sRGB => vks::VK_FORMAT_R8G8B8A8_SRGB,
            Format::B8G8R8A8_UNorm => vks::VK_FORMAT_B8G8R8A8_UNORM,
            Format::B8G8R8A8_SNorm => vks::VK_FORMAT_B8G8R8A8_SNORM,
            Format::B8G8R8A8_UScaled => vks::VK_FORMAT_B8G8R8A8_USCALED,
            Format::B8G8R8A8_SScaled => vks::VK_FORMAT_B8G8R8A8_SSCALED,
            Format::B8G8R8A8_UInt => vks::VK_FORMAT_B8G8R8A8_UINT,
            Format::B8G8R8A8_SInt => vks::VK_FORMAT_B8G8R8A8_SINT,
            Format::B8G8R8A8_sRGB => vks::VK_FORMAT_B8G8R8A8_SRGB,
            Format::A8B8G8R8_UNorm_Pack32 => vks::VK_FORMAT_A8B8G8R8_UNORM_PACK32,
            Format::A8B8G8R8_SNorm_Pack32 => vks::VK_FORMAT_A8B8G8R8_SNORM_PACK32,
            Format::A8B8G8R8_UScaled_Pack32 => vks::VK_FORMAT_A8B8G8R8_USCALED_PACK32,
            Format::A8B8G8R8_SScaled_Pack32 => vks::VK_FORMAT_A8B8G8R8_SSCALED_PACK32,
            Format::A8B8G8R8_UInt_Pack32 => vks::VK_FORMAT_A8B8G8R8_UINT_PACK32,
            Format::A8B8G8R8_SInt_Pack32 => vks::VK_FORMAT_A8B8G8R8_SINT_PACK32,
            Format::A8B8G8R8_sRGB_Pack32 => vks::VK_FORMAT_A8B8G8R8_SRGB_PACK32,
            Format::A2R10G10B10_UNorm_Pack32 => vks::VK_FORMAT_A2R10G10B10_UNORM_PACK32,
            Format::A2R10G10B10_SNorm_Pack32 => vks::VK_FORMAT_A2R10G10B10_SNORM_PACK32,
            Format::A2R10G10B10_UScaled_Pack32 => vks::VK_FORMAT_A2R10G10B10_USCALED_PACK32,
            Format::A2R10G10B10_SScaled_Pack32 => vks::VK_FORMAT_A2R10G10B10_SSCALED_PACK32,
            Format::A2R10G10B10_UInt_Pack32 => vks::VK_FORMAT_A2R10G10B10_UINT_PACK32,
            Format::A2R10G10B10_SInt_Pack32 => vks::VK_FORMAT_A2R10G10B10_SINT_PACK32,
            Format::A2B10G10R10_UNorm_Pack32 => vks::VK_FORMAT_A2B10G10R10_UNORM_PACK32,
            Format::A2B10G10R10_SNorm_Pack32 => vks::VK_FORMAT_A2B10G10R10_SNORM_PACK32,
            Format::A2B10G10R10_UScaled_Pack32 => vks::VK_FORMAT_A2B10G10R10_USCALED_PACK32,
            Format::A2B10G10R10_SScaled_Pack32 => vks::VK_FORMAT_A2B10G10R10_SSCALED_PACK32,
            Format::A2B10G10R10_UInt_Pack32 => vks::VK_FORMAT_A2B10G10R10_UINT_PACK32,
            Format::A2B10G10R10_SInt_Pack32 => vks::VK_FORMAT_A2B10G10R10_SINT_PACK32,
            Format::R16_UNorm => vks::VK_FORMAT_R16_UNORM,
            Format::R16_SNorm => vks::VK_FORMAT_R16_SNORM,
            Format::R16_UScaled => vks::VK_FORMAT_R16_USCALED,
            Format::R16_SScaled => vks::VK_FORMAT_R16_SSCALED,
            Format::R16_UInt => vks::VK_FORMAT_R16_UINT,
            Format::R16_SInt => vks::VK_FORMAT_R16_SINT,
            Format::R16_SFloat => vks::VK_FORMAT_R16_SFLOAT,
            Format::R16G16_UNorm => vks::VK_FORMAT_R16G16_UNORM,
            Format::R16G16_SNorm => vks::VK_FORMAT_R16G16_SNORM,
            Format::R16G16_UScaled => vks::VK_FORMAT_R16G16_USCALED,
            Format::R16G16_SScaled => vks::VK_FORMAT_R16G16_SSCALED,
            Format::R16G16_UInt => vks::VK_FORMAT_R16G16_UINT,
            Format::R16G16_SInt => vks::VK_FORMAT_R16G16_SINT,
            Format::R16G16_SFloat => vks::VK_FORMAT_R16G16_SFLOAT,
            Format::R16G16B16_UNorm => vks::VK_FORMAT_R16G16B16_UNORM,
            Format::R16G16B16_SNorm => vks::VK_FORMAT_R16G16B16_SNORM,
            Format::R16G16B16_UScaled => vks::VK_FORMAT_R16G16B16_USCALED,
            Format::R16G16B16_SScaled => vks::VK_FORMAT_R16G16B16_SSCALED,
            Format::R16G16B16_UInt => vks::VK_FORMAT_R16G16B16_UINT,
            Format::R16G16B16_SInt => vks::VK_FORMAT_R16G16B16_SINT,
            Format::R16G16B16_SFloat => vks::VK_FORMAT_R16G16B16_SFLOAT,
            Format::R16G16B16A16_UNorm => vks::VK_FORMAT_R16G16B16A16_UNORM,
            Format::R16G16B16A16_SNorm => vks::VK_FORMAT_R16G16B16A16_SNORM,
            Format::R16G16B16A16_UScaled => vks::VK_FORMAT_R16G16B16A16_USCALED,
            Format::R16G16B16A16_SScaled => vks::VK_FORMAT_R16G16B16A16_SSCALED,
            Format::R16G16B16A16_UInt => vks::VK_FORMAT_R16G16B16A16_UINT,
            Format::R16G16B16A16_SInt => vks::VK_FORMAT_R16G16B16A16_SINT,
            Format::R16G16B16A16_SFloat => vks::VK_FORMAT_R16G16B16A16_SFLOAT,
            Format::R32_UInt => vks::VK_FORMAT_R32_UINT,
            Format::R32_SInt => vks::VK_FORMAT_R32_SINT,
            Format::R32_SFloat => vks::VK_FORMAT_R32_SFLOAT,
            Format::R32G32_UInt => vks::VK_FORMAT_R32G32_UINT,
            Format::R32G32_SInt => vks::VK_FORMAT_R32G32_SINT,
            Format::R32G32_SFloat => vks::VK_FORMAT_R32G32_SFLOAT,
            Format::R32G32B32_UInt => vks::VK_FORMAT_R32G32B32_UINT,
            Format::R32G32B32_SInt => vks::VK_FORMAT_R32G32B32_SINT,
            Format::R32G32B32_SFloat => vks::VK_FORMAT_R32G32B32_SFLOAT,
            Format::R32G32B32A32_UInt => vks::VK_FORMAT_R32G32B32A32_UINT,
            Format::R32G32B32A32_SInt => vks::VK_FORMAT_R32G32B32A32_SINT,
            Format::R32G32B32A32_SFloat => vks::VK_FORMAT_R32G32B32A32_SFLOAT,
            Format::R64_UInt => vks::VK_FORMAT_R64_UINT,
            Format::R64_SInt => vks::VK_FORMAT_R64_SINT,
            Format::R64_SFloat => vks::VK_FORMAT_R64_SFLOAT,
            Format::R64G64_UInt => vks::VK_FORMAT_R64G64_UINT,
            Format::R64G64_SInt => vks::VK_FORMAT_R64G64_SINT,
            Format::R64G64_SFloat => vks::VK_FORMAT_R64G64_SFLOAT,
            Format::R64G64B64_UInt => vks::VK_FORMAT_R64G64B64_UINT,
            Format::R64G64B64_SInt => vks::VK_FORMAT_R64G64B64_SINT,
            Format::R64G64B64_SFloat => vks::VK_FORMAT_R64G64B64_SFLOAT,
            Format::R64G64B64A64_UInt => vks::VK_FORMAT_R64G64B64A64_UINT,
            Format::R64G64B64A64_SInt => vks::VK_FORMAT_R64G64B64A64_SINT,
            Format::R64G64B64A64_SFloat => vks::VK_FORMAT_R64G64B64A64_SFLOAT,
            Format::B10G11R11_UFloat_Pack32 => vks::VK_FORMAT_B10G11R11_UFLOAT_PACK32,
            Format::E5B9G9R9_UFloat_Pack32 => vks::VK_FORMAT_E5B9G9R9_UFLOAT_PACK32,
            Format::D16_UNorm => vks::VK_FORMAT_D16_UNORM,
            Format::X8_D24_UNorm_Pack32 => vks::VK_FORMAT_X8_D24_UNORM_PACK32,
            Format::D32_SFloat => vks::VK_FORMAT_D32_SFLOAT,
            Format::S8_UInt => vks::VK_FORMAT_S8_UINT,
            Format::D16_UNorm_S8_UInt => vks::VK_FORMAT_D16_UNORM_S8_UINT,
            Format::D24_UNorm_S8_UInt => vks::VK_FORMAT_D24_UNORM_S8_UINT,
            Format::D32_SFloat_S8_UInt => vks::VK_FORMAT_D32_SFLOAT_S8_UINT,
            Format::BC1_RGB_UNorm_Block => vks::VK_FORMAT_BC1_RGB_UNORM_BLOCK,
            Format::BC1_RGB_sRGB_Block => vks::VK_FORMAT_BC1_RGB_SRGB_BLOCK,
            Format::BC1_RGBA_UNorm_Block => vks::VK_FORMAT_BC1_RGBA_UNORM_BLOCK,
            Format::BC1_RGBA_sRGB_Block => vks::VK_FORMAT_BC1_RGBA_SRGB_BLOCK,
            Format::BC2_UNorm_Block => vks::VK_FORMAT_BC2_UNORM_BLOCK,
            Format::BC2_sRGB_Block => vks::VK_FORMAT_BC2_SRGB_BLOCK,
            Format::BC3_UNorm_Block => vks::VK_FORMAT_BC3_UNORM_BLOCK,
            Format::BC3_sRGB_Block => vks::VK_FORMAT_BC3_SRGB_BLOCK,
            Format::BC4_UNorm_Block => vks::VK_FORMAT_BC4_UNORM_BLOCK,
            Format::BC4_SNorm_Block => vks::VK_FORMAT_BC4_SNORM_BLOCK,
            Format::BC5_UNorm_Block => vks::VK_FORMAT_BC5_UNORM_BLOCK,
            Format::BC5_SNorm_Block => vks::VK_FORMAT_BC5_SNORM_BLOCK,
            Format::BC6H_UFloat_Block => vks::VK_FORMAT_BC6H_UFLOAT_BLOCK,
            Format::BC6H_SFloat_Block => vks::VK_FORMAT_BC6H_SFLOAT_BLOCK,
            Format::BC7_UNorm_Block => vks::VK_FORMAT_BC7_UNORM_BLOCK,
            Format::BC7_sRGB_Block => vks::VK_FORMAT_BC7_SRGB_BLOCK,
            Format::ETC2_R8G8B8_UNorm_Block => vks::VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK,
            Format::ETC2_R8G8B8_sRGB_Block => vks::VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK,
            Format::ETC2_R8G8B8A1_UNorm_Block => vks::VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK,
            Format::ETC2_R8G8B8A1_sRGB_Block => vks::VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK,
            Format::ETC2_R8G8B8A8_UNorm_Block => vks::VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK,
            Format::ETC2_R8G8B8A8_sRGB_Block => vks::VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK,
            Format::EAC_R11_UNorm_Block => vks::VK_FORMAT_EAC_R11_UNORM_BLOCK,
            Format::EAC_R11_SNorm_Block => vks::VK_FORMAT_EAC_R11_SNORM_BLOCK,
            Format::EAC_R11G11_UNorm_Block => vks::VK_FORMAT_EAC_R11G11_UNORM_BLOCK,
            Format::EAC_R11G11_SNorm_Block => vks::VK_FORMAT_EAC_R11G11_SNORM_BLOCK,
            Format::ASTC_4x4_UNorm_Block => vks::VK_FORMAT_ASTC_4x4_UNORM_BLOCK,
            Format::ASTC_4x4_sRGB_Block => vks::VK_FORMAT_ASTC_4x4_SRGB_BLOCK,
            Format::ASTC_5x4_UNorm_Block => vks::VK_FORMAT_ASTC_5x4_UNORM_BLOCK,
            Format::ASTC_5x4_sRGB_Block => vks::VK_FORMAT_ASTC_5x4_SRGB_BLOCK,
            Format::ASTC_5x5_UNorm_Block => vks::VK_FORMAT_ASTC_5x5_UNORM_BLOCK,
            Format::ASTC_5x5_sRGB_Block => vks::VK_FORMAT_ASTC_5x5_SRGB_BLOCK,
            Format::ASTC_6x5_UNorm_Block => vks::VK_FORMAT_ASTC_6x5_UNORM_BLOCK,
            Format::ASTC_6x5_sRGB_Block => vks::VK_FORMAT_ASTC_6x5_SRGB_BLOCK,
            Format::ASTC_6x6_UNorm_Block => vks::VK_FORMAT_ASTC_6x6_UNORM_BLOCK,
            Format::ASTC_6x6_sRGB_Block => vks::VK_FORMAT_ASTC_6x6_SRGB_BLOCK,
            Format::ASTC_8x5_UNorm_Block => vks::VK_FORMAT_ASTC_8x5_UNORM_BLOCK,
            Format::ASTC_8x5_sRGB_Block => vks::VK_FORMAT_ASTC_8x5_SRGB_BLOCK,
            Format::ASTC_8x6_UNorm_Block => vks::VK_FORMAT_ASTC_8x6_UNORM_BLOCK,
            Format::ASTC_8x6_sRGB_Block => vks::VK_FORMAT_ASTC_8x6_SRGB_BLOCK,
            Format::ASTC_8x8_UNorm_Block => vks::VK_FORMAT_ASTC_8x8_UNORM_BLOCK,
            Format::ASTC_8x8_sRGB_Block => vks::VK_FORMAT_ASTC_8x8_SRGB_BLOCK,
            Format::ASTC_10x5_UNorm_Block => vks::VK_FORMAT_ASTC_10x5_UNORM_BLOCK,
            Format::ASTC_10x5_sRGB_Block => vks::VK_FORMAT_ASTC_10x5_SRGB_BLOCK,
            Format::ASTC_10x6_UNorm_Block => vks::VK_FORMAT_ASTC_10x6_UNORM_BLOCK,
            Format::ASTC_10x6_sRGB_Block => vks::VK_FORMAT_ASTC_10x6_SRGB_BLOCK,
            Format::ASTC_10x8_UNorm_Block => vks::VK_FORMAT_ASTC_10x8_UNORM_BLOCK,
            Format::ASTC_10x8_sRGB_Block => vks::VK_FORMAT_ASTC_10x8_SRGB_BLOCK,
            Format::ASTC_10x10_UNorm_Block => vks::VK_FORMAT_ASTC_10x10_UNORM_BLOCK,
            Format::ASTC_10x10_sRGB_Block => vks::VK_FORMAT_ASTC_10x10_SRGB_BLOCK,
            Format::ASTC_12x10_UNorm_Block => vks::VK_FORMAT_ASTC_12x10_UNORM_BLOCK,
            Format::ASTC_12x10_sRGB_Block => vks::VK_FORMAT_ASTC_12x10_SRGB_BLOCK,
            Format::ASTC_12x12_UNorm_Block => vks::VK_FORMAT_ASTC_12x12_UNORM_BLOCK,
            Format::ASTC_12x12_sRGB_Block => vks::VK_FORMAT_ASTC_12x12_SRGB_BLOCK,
            Format::Unknown(format) => format,
        }
    }
}

/// See [`VkImageType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageType)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ImageType {
    Type1D,
    Type2D,
    Type3D,
    Unknown(vks::VkImageType),
}

impl From<ImageType> for vks::VkImageType {
    fn from(image_type: ImageType) -> Self {
        match image_type {
            ImageType::Type1D => vks::VK_IMAGE_TYPE_1D,
            ImageType::Type2D => vks::VK_IMAGE_TYPE_2D,
            ImageType::Type3D => vks::VK_IMAGE_TYPE_3D,
            ImageType::Unknown(image_type) => image_type,
        }
    }
}

/// See [`VkImageTiling`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageTiling)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ImageTiling {
    Optimal,
    Linear,
    Unknown(vks::VkImageTiling),
}

impl From<ImageTiling> for vks::VkImageTiling {
    fn from(tiling: ImageTiling) -> Self {
        match tiling {
            ImageTiling::Optimal => vks::VK_IMAGE_TILING_OPTIMAL,
            ImageTiling::Linear => vks::VK_IMAGE_TILING_LINEAR,
            ImageTiling::Unknown(tiling) => tiling,
        }
    }
}

/// See [`VkPhysicalDeviceType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceType)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PhysicalDeviceType {
    Other,
    IntegratedGpu,
    DiscreteGpu,
    VirtualGpu,
    Cpu,
    Unknown(vks::VkPhysicalDeviceType),
}

impl From<vks::VkPhysicalDeviceType> for PhysicalDeviceType {
    fn from(physical_device_type: vks::VkPhysicalDeviceType) -> Self {
        match physical_device_type {
            vks::VK_PHYSICAL_DEVICE_TYPE_OTHER => PhysicalDeviceType::Other,
            vks::VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU => PhysicalDeviceType::IntegratedGpu,
            vks::VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU => PhysicalDeviceType::DiscreteGpu,
            vks::VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU => PhysicalDeviceType::VirtualGpu,
            vks::VK_PHYSICAL_DEVICE_TYPE_CPU => PhysicalDeviceType::Cpu,
            _ => PhysicalDeviceType::Unknown(physical_device_type),
        }
    }
}

impl From<PhysicalDeviceType> for vks::VkPhysicalDeviceType {
    fn from(physical_device_type: PhysicalDeviceType) -> Self {
        match physical_device_type {
            PhysicalDeviceType::Other => vks::VK_PHYSICAL_DEVICE_TYPE_OTHER,
            PhysicalDeviceType::IntegratedGpu => vks::VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU,
            PhysicalDeviceType::DiscreteGpu => vks::VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU,
            PhysicalDeviceType::VirtualGpu => vks::VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU,
            PhysicalDeviceType::Cpu => vks::VK_PHYSICAL_DEVICE_TYPE_CPU,
            PhysicalDeviceType::Unknown(physical_device_type) => physical_device_type,
        }
    }
}

/// See [`VkQueryType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryType)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum QueryType {
    Occlusion,
    PipelineStatistics,
    Timestamp,
    Unknown(vks::VkQueryType),
}

impl From<QueryType> for vks::VkQueryType {
    fn from(query_type: QueryType) -> Self {
        match query_type {
            QueryType::Occlusion => vks::VK_QUERY_TYPE_OCCLUSION,
            QueryType::PipelineStatistics => vks::VK_QUERY_TYPE_PIPELINE_STATISTICS,
            QueryType::Timestamp => vks::VK_QUERY_TYPE_TIMESTAMP,
            QueryType::Unknown(query_type) => query_type,
        }
    }
}

/// See [`VkSharingMode`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSharingMode)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SharingMode {
    Exclusive,
    Concurrent,
    Unknown(vks::VkSharingMode),
}

impl From<SharingMode> for vks::VkSharingMode {
    fn from(mode: SharingMode) -> Self {
        match mode {
            SharingMode::Exclusive => vks::VK_SHARING_MODE_EXCLUSIVE,
            SharingMode::Concurrent => vks::VK_SHARING_MODE_CONCURRENT,
            SharingMode::Unknown(mode) => mode,
        }
    }
}

/// See [`VkImageLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageLayout)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ImageLayout {
    Undefined,
    General,
    ColorAttachmentOptimal,
    DepthStencilAttachmentOptimal,
    DepthStencilReadOnlyOptimal,
    ShaderReadOnlyOptimal,
    TransferSrcOptimal,
    TransferDstOptimal,
    Preinitialized,

    /// See extension [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)
    PresentSrcKhr,

    Unknown(vks::VkImageLayout),
}

impl From<ImageLayout> for vks::VkImageLayout {
    fn from(layout: ImageLayout) -> Self {
        match layout {
            ImageLayout::Undefined => vks::VK_IMAGE_LAYOUT_UNDEFINED,
            ImageLayout::General => vks::VK_IMAGE_LAYOUT_GENERAL,
            ImageLayout::ColorAttachmentOptimal => vks::VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
            ImageLayout::DepthStencilAttachmentOptimal => vks::VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
            ImageLayout::DepthStencilReadOnlyOptimal => vks::VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL,
            ImageLayout::ShaderReadOnlyOptimal => vks::VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL,
            ImageLayout::TransferSrcOptimal => vks::VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
            ImageLayout::TransferDstOptimal => vks::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
            ImageLayout::Preinitialized => vks::VK_IMAGE_LAYOUT_PREINITIALIZED,
            ImageLayout::PresentSrcKhr => vks::VK_IMAGE_LAYOUT_PRESENT_SRC_KHR,
            ImageLayout::Unknown(layout) => layout,
        }
    }
}

impl From<vks::VkImageLayout> for ImageLayout {
    fn from(layout: vks::VkImageLayout) -> Self {
        match layout {
            vks::VK_IMAGE_LAYOUT_UNDEFINED => ImageLayout::Undefined,
            vks::VK_IMAGE_LAYOUT_GENERAL => ImageLayout::General,
            vks::VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL => ImageLayout::ColorAttachmentOptimal,
            vks::VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL => ImageLayout::DepthStencilAttachmentOptimal,
            vks::VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL => ImageLayout::DepthStencilReadOnlyOptimal,
            vks::VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL => ImageLayout::ShaderReadOnlyOptimal,
            vks::VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL => ImageLayout::TransferSrcOptimal,
            vks::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL => ImageLayout::TransferDstOptimal,
            vks::VK_IMAGE_LAYOUT_PREINITIALIZED => ImageLayout::Preinitialized,
            vks::VK_IMAGE_LAYOUT_PRESENT_SRC_KHR => ImageLayout::PresentSrcKhr,
            _ => ImageLayout::Unknown(layout),
        }
    }
}

/// See [`VkImageViewType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageViewType)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ImageViewType {
    Type1D,
    Type2D,
    Type3D,
    TypeCube,
    Type1DArray,
    Type2DArray,
    TypeCubeArray,
    Unknown(vks::VkImageViewType),
}

impl From<ImageViewType> for vks::VkImageViewType {
    fn from(view_type: ImageViewType) -> Self {
        match view_type {
            ImageViewType::Type1D => vks::VK_IMAGE_VIEW_TYPE_1D,
            ImageViewType::Type2D => vks::VK_IMAGE_VIEW_TYPE_2D,
            ImageViewType::Type3D => vks::VK_IMAGE_VIEW_TYPE_3D,
            ImageViewType::TypeCube => vks::VK_IMAGE_VIEW_TYPE_CUBE,
            ImageViewType::Type1DArray => vks::VK_IMAGE_VIEW_TYPE_1D_ARRAY,
            ImageViewType::Type2DArray => vks::VK_IMAGE_VIEW_TYPE_2D_ARRAY,
            ImageViewType::TypeCubeArray => vks::VK_IMAGE_VIEW_TYPE_CUBE_ARRAY,
            ImageViewType::Unknown(view_type) => view_type,
        }
    }
}

/// See [`VkComponentSwizzle`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkComponentSwizzle)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ComponentSwizzle {
    Identity,
    Zero,
    One,
    R,
    G,
    B,
    A,
    Unknown(vks::VkComponentSwizzle),
}

impl From<ComponentSwizzle> for vks::VkComponentSwizzle {
    fn from(swizzle: ComponentSwizzle) -> Self {
        match swizzle {
            ComponentSwizzle::Identity => vks::VK_COMPONENT_SWIZZLE_IDENTITY,
            ComponentSwizzle::Zero => vks::VK_COMPONENT_SWIZZLE_ZERO,
            ComponentSwizzle::One => vks::VK_COMPONENT_SWIZZLE_ONE,
            ComponentSwizzle::R => vks::VK_COMPONENT_SWIZZLE_R,
            ComponentSwizzle::G => vks::VK_COMPONENT_SWIZZLE_G,
            ComponentSwizzle::B => vks::VK_COMPONENT_SWIZZLE_B,
            ComponentSwizzle::A => vks::VK_COMPONENT_SWIZZLE_A,
            ComponentSwizzle::Unknown(swizzle) => swizzle,
        }
    }
}

/// See [`VkVertexInputRate`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkVertexInputRate)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VertexInputRate {
    Vertex,
    Instance,
    Unknown(vks::VkVertexInputRate),
}

impl From<VertexInputRate> for vks::VkVertexInputRate {
    fn from(rate: VertexInputRate) -> Self {
        match rate {
            VertexInputRate::Vertex => vks::VK_VERTEX_INPUT_RATE_VERTEX,
            VertexInputRate::Instance => vks::VK_VERTEX_INPUT_RATE_INSTANCE,
            VertexInputRate::Unknown(rate) => rate,
        }
    }
}

/// See [`VkPrimitiveTopology`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPrimitiveTopology)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PrimitiveTopology {
    PointList,
    LineList,
    LineStrip,
    TriangleList,
    TriangleStrip,
    TriangleFan,
    LineListWithAdjacency,
    LineStripWithAdjacency,
    TriangleListWithAdjacency,
    TriangleStripWithAdjacency,
    PatchList,
    Unknown(vks::VkPrimitiveTopology)
}

impl From<PrimitiveTopology> for vks::VkPrimitiveTopology {
    fn from(topology: PrimitiveTopology) -> Self {
        match topology {
            PrimitiveTopology::PointList => vks::VK_PRIMITIVE_TOPOLOGY_POINT_LIST,
            PrimitiveTopology::LineList => vks::VK_PRIMITIVE_TOPOLOGY_LINE_LIST,
            PrimitiveTopology::LineStrip => vks::VK_PRIMITIVE_TOPOLOGY_LINE_STRIP,
            PrimitiveTopology::TriangleList => vks::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
            PrimitiveTopology::TriangleStrip => vks::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP,
            PrimitiveTopology::TriangleFan => vks::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_FAN,
            PrimitiveTopology::LineListWithAdjacency => vks::VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY,
            PrimitiveTopology::LineStripWithAdjacency => vks::VK_PRIMITIVE_TOPOLOGY_LINE_STRIP_WITH_ADJACENCY,
            PrimitiveTopology::TriangleListWithAdjacency => vks::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY,
            PrimitiveTopology::TriangleStripWithAdjacency => vks::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_WITH_ADJACENCY,
            PrimitiveTopology::PatchList => vks::VK_PRIMITIVE_TOPOLOGY_PATCH_LIST,
            PrimitiveTopology::Unknown(topology) => topology,
        }
    }
}

/// See [`VkPolygonMode`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPolygonMode)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PolygonMode {
    Fill,
    Line,
    Point,
    Unknown(vks::VkPolygonMode),
}

impl From<PolygonMode> for vks::VkPolygonMode {
    fn from(mode: PolygonMode) -> Self {
        match mode {
            PolygonMode::Fill => vks::VK_POLYGON_MODE_FILL,
            PolygonMode::Line => vks::VK_POLYGON_MODE_LINE,
            PolygonMode::Point => vks::VK_POLYGON_MODE_POINT,
            PolygonMode::Unknown(mode) => mode,
        }
    }
}

/// See [`VkFrontFace`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFrontFace)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FrontFace {
    CounterClockwise,
    Clockwise,
    Unknown(vks::VkFrontFace),
}

impl From<FrontFace> for vks::VkFrontFace {
    fn from(face: FrontFace) -> Self {
        match face {
            FrontFace::CounterClockwise => vks::VK_FRONT_FACE_COUNTER_CLOCKWISE,
            FrontFace::Clockwise => vks::VK_FRONT_FACE_CLOCKWISE,
            FrontFace::Unknown(face) => face,
        }
    }
}

/// See [`VkCompareOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCompareOp)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CompareOp {
    Never,
    Less,
    Equal,
    LessOrEqual,
    Greater,
    NotEqual,
    GreaterOrEqual,
    Always,
    Unknown(vks::VkCompareOp),
}

impl From<CompareOp> for vks::VkCompareOp {
    fn from(op: CompareOp) -> Self {
        match op {
            CompareOp::Never => vks::VK_COMPARE_OP_NEVER,
            CompareOp::Less => vks::VK_COMPARE_OP_LESS,
            CompareOp::Equal => vks::VK_COMPARE_OP_EQUAL,
            CompareOp::LessOrEqual => vks::VK_COMPARE_OP_LESS_OR_EQUAL,
            CompareOp::Greater => vks::VK_COMPARE_OP_GREATER,
            CompareOp::NotEqual => vks::VK_COMPARE_OP_NOT_EQUAL,
            CompareOp::GreaterOrEqual => vks::VK_COMPARE_OP_GREATER_OR_EQUAL,
            CompareOp::Always => vks::VK_COMPARE_OP_ALWAYS,
            CompareOp::Unknown(op) => op,
        }
    }
}

/// See [`VkStencilOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStencilOp)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum StencilOp {
    Keep,
    Zero,
    Replace,
    IncrementAndClamp,
    DecrementAndClamp,
    Invert,
    IncrementAndWrap,
    DecrementAndWrap,
    Unknown(vks::VkStencilOp),
}

impl From<StencilOp> for vks::VkStencilOp {
    fn from(op: StencilOp) -> Self {
        match op {
            StencilOp::Keep => vks::VK_STENCIL_OP_KEEP,
            StencilOp::Zero => vks::VK_STENCIL_OP_ZERO,
            StencilOp::Replace => vks::VK_STENCIL_OP_REPLACE,
            StencilOp::IncrementAndClamp => vks::VK_STENCIL_OP_INCREMENT_AND_CLAMP,
            StencilOp::DecrementAndClamp => vks::VK_STENCIL_OP_DECREMENT_AND_CLAMP,
            StencilOp::Invert => vks::VK_STENCIL_OP_INVERT,
            StencilOp::IncrementAndWrap => vks::VK_STENCIL_OP_INCREMENT_AND_WRAP,
            StencilOp::DecrementAndWrap => vks::VK_STENCIL_OP_DECREMENT_AND_WRAP,
            StencilOp::Unknown(op) => op,
        }
    }
}

/// See [`VkLogicOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkLogicOp)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LogicOp {
    Clear,
    And,
    AndReverse,
    Copy,
    AndInverted,
    NoOp,
    Xor,
    Or,
    Nor,
    Equivalent,
    Invert,
    OrReverse,
    CopyInverted,
    OrInverted,
    Nand,
    Set,
    Unknown(vks::VkLogicOp),
}

impl From<LogicOp> for vks::VkLogicOp {
    fn from(op: LogicOp) -> Self {
        match op {
            LogicOp::Clear => vks::VK_LOGIC_OP_CLEAR,
            LogicOp::And => vks::VK_LOGIC_OP_AND,
            LogicOp::AndReverse => vks::VK_LOGIC_OP_AND_REVERSE,
            LogicOp::Copy => vks::VK_LOGIC_OP_COPY,
            LogicOp::AndInverted => vks::VK_LOGIC_OP_AND_INVERTED,
            LogicOp::NoOp => vks::VK_LOGIC_OP_NO_OP,
            LogicOp::Xor => vks::VK_LOGIC_OP_XOR,
            LogicOp::Or => vks::VK_LOGIC_OP_OR,
            LogicOp::Nor => vks::VK_LOGIC_OP_NOR,
            LogicOp::Equivalent => vks::VK_LOGIC_OP_EQUIVALENT,
            LogicOp::Invert => vks::VK_LOGIC_OP_INVERT,
            LogicOp::OrReverse => vks::VK_LOGIC_OP_OR_REVERSE,
            LogicOp::CopyInverted => vks::VK_LOGIC_OP_COPY_INVERTED,
            LogicOp::OrInverted => vks::VK_LOGIC_OP_OR_INVERTED,
            LogicOp::Nand => vks::VK_LOGIC_OP_NAND,
            LogicOp::Set => vks::VK_LOGIC_OP_SET,
            LogicOp::Unknown(op) => op,
        }
    }
}

/// See [`VkBlendFactor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBlendFactor)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BlendFactor {
    Zero,
    One,
    SrcColor,
    OneMinusSrcColor,
    DstColor,
    OneMinusDstColor,
    SrcAlpha,
    OneMinusSrcAlpha,
    DstAlpha,
    OneMinusDstAlpha,
    ConstantColor,
    OneMinusConstantColor,
    ConstantAlpha,
    OneMinusConstantAlpha,
    SrcAlphaSaturate,
    Src1Color,
    OneMinusSrc1Color,
    Src1Alpha,
    OneMinusSrc1Alpha,
    Unknown(vks::VkBlendFactor),
}

impl From<BlendFactor> for vks::VkBlendFactor {
    fn from(factor: BlendFactor) -> Self {
        match factor {
            BlendFactor::Zero => vks::VK_BLEND_FACTOR_ZERO,
            BlendFactor::One => vks::VK_BLEND_FACTOR_ONE,
            BlendFactor::SrcColor => vks::VK_BLEND_FACTOR_SRC_COLOR,
            BlendFactor::OneMinusSrcColor => vks::VK_BLEND_FACTOR_ONE_MINUS_SRC_COLOR,
            BlendFactor::DstColor => vks::VK_BLEND_FACTOR_DST_COLOR,
            BlendFactor::OneMinusDstColor => vks::VK_BLEND_FACTOR_ONE_MINUS_DST_COLOR,
            BlendFactor::SrcAlpha => vks::VK_BLEND_FACTOR_SRC_ALPHA,
            BlendFactor::OneMinusSrcAlpha => vks::VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA,
            BlendFactor::DstAlpha => vks::VK_BLEND_FACTOR_DST_ALPHA,
            BlendFactor::OneMinusDstAlpha => vks::VK_BLEND_FACTOR_ONE_MINUS_DST_ALPHA,
            BlendFactor::ConstantColor => vks::VK_BLEND_FACTOR_CONSTANT_COLOR,
            BlendFactor::OneMinusConstantColor => vks::VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR,
            BlendFactor::ConstantAlpha => vks::VK_BLEND_FACTOR_CONSTANT_ALPHA,
            BlendFactor::OneMinusConstantAlpha => vks::VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA,
            BlendFactor::SrcAlphaSaturate => vks::VK_BLEND_FACTOR_SRC_ALPHA_SATURATE,
            BlendFactor::Src1Color => vks::VK_BLEND_FACTOR_SRC1_COLOR,
            BlendFactor::OneMinusSrc1Color => vks::VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR,
            BlendFactor::Src1Alpha => vks::VK_BLEND_FACTOR_SRC1_ALPHA,
            BlendFactor::OneMinusSrc1Alpha => vks::VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA,
            BlendFactor::Unknown(factor) => factor,
        }
    }
}

/// See [`VkBlendOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBlendOp)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BlendOp {
    Add,
    Subtract,
    ReverseSubtract,
    Min,
    Max,
    Unknown(vks::VkBlendOp),
}

impl From<BlendOp> for vks::VkBlendOp {
    fn from(op: BlendOp) -> Self {
        match op {
            BlendOp::Add => vks::VK_BLEND_OP_ADD,
            BlendOp::Subtract => vks::VK_BLEND_OP_SUBTRACT,
            BlendOp::ReverseSubtract => vks::VK_BLEND_OP_REVERSE_SUBTRACT,
            BlendOp::Min => vks::VK_BLEND_OP_MIN,
            BlendOp::Max => vks::VK_BLEND_OP_MAX,
            BlendOp::Unknown(op) => op,
        }
    }
}

/// See [`VkDynamicState`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDynamicState)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DynamicState {
    Viewport,
    Scissor,
    LineWidth,
    DepthBias,
    BlendConstants,
    DepthBounds,
    StencilCompareMask,
    StencilWriteMask,
    StencilReference,
    Unknown(vks::VkDynamicState),
}

impl From<DynamicState> for vks::VkDynamicState {
    fn from(state: DynamicState) -> Self {
        match state {
            DynamicState::Viewport => vks::VK_DYNAMIC_STATE_VIEWPORT,
            DynamicState::Scissor => vks::VK_DYNAMIC_STATE_SCISSOR,
            DynamicState::LineWidth => vks::VK_DYNAMIC_STATE_LINE_WIDTH,
            DynamicState::DepthBias => vks::VK_DYNAMIC_STATE_DEPTH_BIAS,
            DynamicState::BlendConstants => vks::VK_DYNAMIC_STATE_BLEND_CONSTANTS,
            DynamicState::DepthBounds => vks::VK_DYNAMIC_STATE_DEPTH_BOUNDS,
            DynamicState::StencilCompareMask => vks::VK_DYNAMIC_STATE_STENCIL_COMPARE_MASK,
            DynamicState::StencilWriteMask => vks::VK_DYNAMIC_STATE_STENCIL_WRITE_MASK,
            DynamicState::StencilReference => vks::VK_DYNAMIC_STATE_STENCIL_REFERENCE,
            DynamicState::Unknown(state) => state,
        }
    }
}

/// See [`VkFilter`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFilter)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Filter {
    Nearest,
    Linear,
    CubicImg,
    Unknown(vks::VkFilter),
}

impl From<Filter> for vks::VkFilter {
    fn from(filter: Filter) -> Self {
        match filter {
            Filter::Nearest => vks::VK_FILTER_NEAREST,
            Filter::Linear => vks::VK_FILTER_LINEAR,
            Filter::CubicImg => vks::VK_FILTER_CUBIC_IMG,
            Filter::Unknown(filter) => filter,
        }
    }
}

/// See [`VkSamplerMipmapMode`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSamplerMipmapMode)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SamplerMipmapMode {
    Nearest,
    Linear,
    Unknown(vks::VkSamplerMipmapMode),
}

impl From<SamplerMipmapMode> for vks::VkSamplerMipmapMode {
    fn from(mode: SamplerMipmapMode) -> Self {
        match mode {
            SamplerMipmapMode::Nearest => vks::VK_SAMPLER_MIPMAP_MODE_NEAREST,
            SamplerMipmapMode::Linear => vks::VK_SAMPLER_MIPMAP_MODE_LINEAR,
            SamplerMipmapMode::Unknown(mode) => mode,
        }
    }
}

/// See [`VkSamplerAddressMode`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSamplerAddressMode)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SamplerAddressMode {
    Repeat,
    MirroredRepeat,
    ClampToEdge,
    ClampToBorder,
    MirrorClampToEdge,
    Unknown(vks::VkSamplerAddressMode),
}

impl From<SamplerAddressMode> for vks::VkSamplerAddressMode {
    fn from(mode: SamplerAddressMode) -> Self {
        match mode {
            SamplerAddressMode::Repeat => vks::VK_SAMPLER_ADDRESS_MODE_REPEAT,
            SamplerAddressMode::MirroredRepeat => vks::VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT,
            SamplerAddressMode::ClampToEdge => vks::VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE,
            SamplerAddressMode::ClampToBorder => vks::VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER,
            SamplerAddressMode::MirrorClampToEdge => vks::VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE,
            SamplerAddressMode::Unknown(mode) => mode,
        }
    }
}

/// See [`VkBorderColor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBorderColor)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BorderColor {
    FloatTransparentBlack,
    IntTransparentBlack,
    FloatOpaqueBlack,
    IntOpaqueBlack,
    FloatOpaqueWhite,
    IntOpaqueWhite,
    Unknown(vks::VkBorderColor),
}

impl From<BorderColor> for vks::VkBorderColor {
    fn from(color: BorderColor) -> Self {
        match color {
            BorderColor::FloatTransparentBlack => vks::VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK,
            BorderColor::IntTransparentBlack => vks::VK_BORDER_COLOR_INT_TRANSPARENT_BLACK,
            BorderColor::FloatOpaqueBlack => vks::VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK,
            BorderColor::IntOpaqueBlack => vks::VK_BORDER_COLOR_INT_OPAQUE_BLACK,
            BorderColor::FloatOpaqueWhite => vks::VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE,
            BorderColor::IntOpaqueWhite => vks::VK_BORDER_COLOR_INT_OPAQUE_WHITE,
            BorderColor::Unknown(color) => color,
        }
    }
}

/// See [`VkDescriptorType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorType)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DescriptorType {
    Sampler,
    CombinedImageSampler,
    SampledImage,
    StorageImage,
    UniformTexelBuffer,
    StorageTexelBuffer,
    UniformBuffer,
    StorageBuffer,
    UniformBufferDynamic,
    StorageBufferDynamic,
    InputAttachment,
    Unknown(vks::VkDescriptorType),
}

impl From<DescriptorType> for vks::VkDescriptorType {
    fn from(descriptor_type: DescriptorType) -> Self {
        match descriptor_type {
            DescriptorType::Sampler => vks::VK_DESCRIPTOR_TYPE_SAMPLER,
            DescriptorType::CombinedImageSampler => vks::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
            DescriptorType::SampledImage => vks::VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE,
            DescriptorType::StorageImage => vks::VK_DESCRIPTOR_TYPE_STORAGE_IMAGE,
            DescriptorType::UniformTexelBuffer => vks::VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER,
            DescriptorType::StorageTexelBuffer => vks::VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER,
            DescriptorType::UniformBuffer => vks::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,
            DescriptorType::StorageBuffer => vks::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
            DescriptorType::UniformBufferDynamic => vks::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC,
            DescriptorType::StorageBufferDynamic => vks::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC,
            DescriptorType::InputAttachment => vks::VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT,
            DescriptorType::Unknown(descriptor_type) => descriptor_type,
        }
    }
}

/// See [`VkAttachmentLoadOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAttachmentLoadOp)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AttachmentLoadOp {
    Load,
    Clear,
    DontCare,
    Unknown(vks::VkAttachmentLoadOp),
}

impl From<AttachmentLoadOp> for vks::VkAttachmentLoadOp {
    fn from(op: AttachmentLoadOp) -> Self {
        match op {
            AttachmentLoadOp::Load => vks::VK_ATTACHMENT_LOAD_OP_LOAD,
            AttachmentLoadOp::Clear => vks::VK_ATTACHMENT_LOAD_OP_CLEAR,
            AttachmentLoadOp::DontCare => vks::VK_ATTACHMENT_LOAD_OP_DONT_CARE,
            AttachmentLoadOp::Unknown(op) => op,
        }
    }
}

impl From<vks::VkAttachmentLoadOp> for AttachmentLoadOp {
    fn from(op: vks::VkAttachmentLoadOp) -> Self {
        match op {
            vks::VK_ATTACHMENT_LOAD_OP_LOAD => AttachmentLoadOp::Load,
            vks::VK_ATTACHMENT_LOAD_OP_CLEAR => AttachmentLoadOp::Clear,
            vks::VK_ATTACHMENT_LOAD_OP_DONT_CARE => AttachmentLoadOp::DontCare,
            _ => AttachmentLoadOp::Unknown(op),
        }
    }
}

/// See [`VkAttachmentStoreOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAttachmentStoreOp)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AttachmentStoreOp {
    Store,
    DontCare,
    Unknown(vks::VkAttachmentStoreOp),
}

impl From<AttachmentStoreOp> for vks::VkAttachmentStoreOp {
    fn from(op: AttachmentStoreOp) -> Self {
        match op {
            AttachmentStoreOp::Store => vks::VK_ATTACHMENT_STORE_OP_STORE,
            AttachmentStoreOp::DontCare => vks::VK_ATTACHMENT_STORE_OP_DONT_CARE,
            AttachmentStoreOp::Unknown(op) => op,
        }
    }
}

impl From<vks::VkAttachmentStoreOp> for AttachmentStoreOp {
    fn from(op: vks::VkAttachmentStoreOp) -> Self {
        match op {
            vks::VK_ATTACHMENT_STORE_OP_STORE => AttachmentStoreOp::Store,
            vks::VK_ATTACHMENT_STORE_OP_DONT_CARE => AttachmentStoreOp::DontCare,
            _ => AttachmentStoreOp::Unknown(op),
        }
    }
}

/// See [`VkPipelineBindPoint`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineBindPoint)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PipelineBindPoint {
    Graphics,
    Compute,
    Unknown(vks::VkPipelineBindPoint),
}

impl From<PipelineBindPoint> for vks::VkPipelineBindPoint {
    fn from(bind_point: PipelineBindPoint) -> Self {
        match bind_point {
            PipelineBindPoint::Graphics => vks::VK_PIPELINE_BIND_POINT_GRAPHICS,
            PipelineBindPoint::Compute => vks::VK_PIPELINE_BIND_POINT_COMPUTE,
            PipelineBindPoint::Unknown(bind_point) => bind_point,
        }
    }
}

impl From<vks::VkPipelineBindPoint> for PipelineBindPoint {
    fn from(bind_point: vks::VkPipelineBindPoint) -> Self {
        match bind_point {
            vks::VK_PIPELINE_BIND_POINT_GRAPHICS => PipelineBindPoint::Graphics,
            vks::VK_PIPELINE_BIND_POINT_COMPUTE => PipelineBindPoint::Compute,
            _ => PipelineBindPoint::Unknown(bind_point),
        }
    }
}

/// See [`VkCommandBufferLevel`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferLevel)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CommandBufferLevel {
    Primary,
    Secondary,
    Unknown(vks::VkCommandBufferLevel),
}

impl From<CommandBufferLevel> for vks::VkCommandBufferLevel {
    fn from(level: CommandBufferLevel) -> Self {
        match level {
            CommandBufferLevel::Primary => vks::VK_COMMAND_BUFFER_LEVEL_PRIMARY,
            CommandBufferLevel::Secondary => vks::VK_COMMAND_BUFFER_LEVEL_SECONDARY,
            CommandBufferLevel::Unknown(level) => level,
        }
    }
}

/// See [`VkIndexType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkIndexType)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum IndexType {
    UInt16,
    UInt32,
    Unknown(vks::VkIndexType),
}

impl From<IndexType> for vks::VkIndexType {
    fn from(index_type: IndexType) -> Self {
        match index_type {
            IndexType::UInt16 => vks::VK_INDEX_TYPE_UINT16,
            IndexType::UInt32 => vks::VK_INDEX_TYPE_UINT32,
            IndexType::Unknown(index_type) => index_type,
        }
    }
}

/// See [`VkSubpassContents`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSubpassContents)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SubpassContents {
    Inline,
    SecondaryCommandBuffers,
    Unknown(vks::VkSubpassContents),
}

impl From<SubpassContents> for vks::VkSubpassContents {
    fn from(contents: SubpassContents) -> Self {
        match contents {
            SubpassContents::Inline => vks::VK_SUBPASS_CONTENTS_INLINE,
            SubpassContents::SecondaryCommandBuffers => vks::VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS,
            SubpassContents::Unknown(contents) => contents,
        }
    }
}

gen_chain_struct! {
    name: ApplicationInfoChain [ApplicationInfoChainWrapper],
    query: ApplicationInfoChainQuery [ApplicationInfoChainQueryWrapper],
    vks: VkApplicationInfo,
    input: true,
    output: false,
}

/// See [`VkApplicationInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkApplicationInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct ApplicationInfo {
    pub application_name: Option<String>,
    pub application_version: u32,
    pub engine_name: Option<String>,
    pub engine_version: u32,
    pub api_version: Option<Version>,
    pub chain: Option<ApplicationInfoChain>,
}

#[derive(Debug)]
struct VkApplicationInfoWrapper {
    pub vks_struct: vks::VkApplicationInfo,
    application_name_cstr: Option<CString>,
    engine_name_cstr: Option<CString>,
    chain: Option<ApplicationInfoChainWrapper>,
}

impl VkApplicationInfoWrapper {
    pub fn new(info: &ApplicationInfo, with_chain: bool) -> Self {
        let application_name_cstr = utils::cstr_from_string(info.application_name.clone());
        let engine_name_cstr = utils::cstr_from_string(info.engine_name.clone());
        let (pnext, chain) = ApplicationInfoChainWrapper::new_optional(&info.chain, with_chain);

        VkApplicationInfoWrapper {
            vks_struct: vks::VkApplicationInfo {
                sType: vks::VK_STRUCTURE_TYPE_APPLICATION_INFO,
                pNext: pnext,
                pApplicationName: application_name_cstr.1,
                applicationVersion: info.application_version,
                pEngineName: engine_name_cstr.1,
                engineVersion: info.engine_version,
                apiVersion: Version::api_version_from_optional(info.api_version),
            },
            application_name_cstr: application_name_cstr.0,
            engine_name_cstr: engine_name_cstr.0,
            chain: chain,
        }
    }
}

gen_chain_struct! {
    name: InstanceCreateInfoChain [InstanceCreateInfoChainWrapper],
    query: InstanceCreateInfoChainQuery [InstanceCreateInfoChainQueryWrapper],
    vks: VkInstanceCreateInfo,
    input: true,
    output: false,

    debug_report_callback_create_info_ext: DebugReportCallbackCreateInfoExt {
        mod: ext_debug_report,
        fn_add: add_debug_report_callback_create_info_ext,
        fn_has: has_debug_report_callback_create_info_ext,
        fn_get: get_debug_report_callback_create_info_ext,
        wrapper: VkDebugReportCallbackCreateInfoEXTWrapper,
        vks: VkDebugReportCallbackCreateInfoEXT,
        stype: vks::VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT,
    }
}

/// See [`VkInstanceCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkInstanceCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct InstanceCreateInfo {
    pub flags: InstanceCreateFlags,
    pub application_info: Option<ApplicationInfo>,
    pub enabled_layers: Vec<String>,
    pub enabled_extensions: InstanceExtensions,
    pub chain: Option<InstanceCreateInfoChain>,
}

#[derive(Debug)]
struct VkInstanceCreateInfoWrapper {
    pub vks_struct: vks::VkInstanceCreateInfo,
    application_info: Option<Box<VkApplicationInfoWrapper>>,
    enabled_layers: Vec<CString>,
    enabled_layers_ptrs: Vec<*const c_char>,
    enabled_extensions: Vec<CString>,
    enabled_extensions_ptrs: Vec<*const c_char>,
    chain: Option<InstanceCreateInfoChainWrapper>,
}

impl VkInstanceCreateInfoWrapper {
    pub fn new(create_info: &InstanceCreateInfo, with_chain: bool) -> Self {
        let (application_info_ptr, application_info) = match create_info.application_info {
            Some(ref application_info) => {
                let application_info: Box<_> = Box::new(VkApplicationInfoWrapper::new(application_info, true));
                (&application_info.vks_struct as *const _, Some(application_info))
            }

            None => (ptr::null(), None),
        };

        let enabled_layers: Vec<_> = create_info.enabled_layers.iter()
            .cloned()
            .map(CString::new)
            .map(Result::unwrap)
            .collect();
        let enabled_layers_ptrs: Vec<_> = enabled_layers
            .iter()
            .map(|l| l.as_ptr())
            .collect();
        let enabled_layers_ptr = if !enabled_layers_ptrs.is_empty() {
            enabled_layers_ptrs.as_ptr()
        }
        else {
            ptr::null()
        };

        let enabled_extensions = create_info.enabled_extensions.to_cstring_vec();
        let enabled_extensions_ptrs: Vec<_> = enabled_extensions
            .iter()
            .map(|l| l.as_ptr())
            .collect();
        let enabled_extensions_ptr = if !enabled_extensions_ptrs.is_empty() {
            enabled_extensions_ptrs.as_ptr()
        }
        else {
            ptr::null()
        };

        let (pnext, chain) = InstanceCreateInfoChainWrapper::new_optional(&create_info.chain, with_chain);

        VkInstanceCreateInfoWrapper {
            vks_struct: vks::VkInstanceCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
                pNext: pnext,
                flags: create_info.flags.bits,
                pApplicationInfo: application_info_ptr,
                enabledLayerCount: enabled_layers_ptrs.len() as u32,
                ppEnabledLayerNames: enabled_layers_ptr,
                enabledExtensionCount: enabled_extensions_ptrs.len() as u32,
                ppEnabledExtensionNames: enabled_extensions_ptr,
            },
            application_info: application_info,
            enabled_layers: enabled_layers,
            enabled_layers_ptrs: enabled_layers_ptrs,
            enabled_extensions: enabled_extensions,
            enabled_extensions_ptrs: enabled_extensions_ptrs,
            chain: chain,
        }
    }
}

/// See [`VkAllocationCallbacks`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAllocationCallbacks)
pub trait Allocator: Send + Sync + fmt::Debug {
    fn alloc(&self, size: usize, alignment: usize, allocation_scope: SystemAllocationSope) -> *mut c_void;
    fn realloc(&self, original: *mut c_void, size: usize, alignment: usize, allocation_scope: SystemAllocationSope) -> *mut c_void;
    fn free(&self, memory: *mut c_void);

    fn has_internal_alloc(&self) -> bool {
        false
    }

    #[allow(unused_variables)]
    fn internal_alloc(&self, size: usize, allocation_type: InternalAllocationType, allocation_scope: SystemAllocationSope) {
        panic!("Default dummy implementation of Allocator::internal_alloc called. Make sure to either implement all three of has_internal_alloc, internal_alloc, and internal_free, or none of them.");
    }

    #[allow(unused_variables)]
    fn internal_free(&self, size: usize, allocation_type: InternalAllocationType, allocation_scope: SystemAllocationSope) {
        panic!("Default dummy implementation of Allocator::internal_free called. Make sure to either implement all three of has_internal_alloc, internal_alloc, and internal_free, or none of them.");
    }
}

/// See [`VkPhysicalDeviceFeatures`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceFeatures)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct PhysicalDeviceFeatures {
    pub robust_buffer_access: bool,
    pub full_draw_index_uint32: bool,
    pub image_cube_array: bool,
    pub independent_blend: bool,
    pub geometry_shader: bool,
    pub tessellation_shader: bool,
    pub sample_rate_shading: bool,
    pub dual_src_blend: bool,
    pub logic_op: bool,
    pub multi_draw_indirect: bool,
    pub draw_indirect_first_instance: bool,
    pub depth_clamp: bool,
    pub depth_bias_clamp: bool,
    pub fill_mode_non_solid: bool,
    pub depth_bounds: bool,
    pub wide_lines: bool,
    pub large_points: bool,
    pub alpha_to_one: bool,
    pub multi_viewport: bool,
    pub sampler_anisotropy: bool,
    pub texture_compression_etc2: bool,
    pub texture_compression_astc_ldr: bool,
    pub texture_compression_bc: bool,
    pub occlusion_query_precise: bool,
    pub pipeline_statistics_query: bool,
    pub vertex_pipeline_stores_and_atomics: bool,
    pub fragment_stores_and_atomics: bool,
    pub shader_tessellation_and_geometry_point_size: bool,
    pub shader_image_gather_extended: bool,
    pub shader_storage_image_extended_formats: bool,
    pub shader_storage_image_multisample: bool,
    pub shader_storage_image_read_without_format: bool,
    pub shader_storage_image_write_without_format: bool,
    pub shader_uniform_buffer_array_dynamic_indexing: bool,
    pub shader_sampled_image_array_dynamic_indexing: bool,
    pub shader_storage_buffer_array_dynamic_indexing: bool,
    pub shader_storage_image_array_dynamic_indexing: bool,
    pub shader_clip_distance: bool,
    pub shader_cull_distance: bool,
    pub shader_float64: bool,
    pub shader_int64: bool,
    pub shader_int16: bool,
    pub shader_resource_residency: bool,
    pub shader_resource_min_lod: bool,
    pub sparse_binding: bool,
    pub sparse_residency_buffer: bool,
    pub sparse_residency_image_2d: bool,
    pub sparse_residency_image_3d: bool,
    pub sparse_residency_2_samples: bool,
    pub sparse_residency_4_samples: bool,
    pub sparse_residency_8_samples: bool,
    pub sparse_residency_16_samples: bool,
    pub sparse_residency_aliased: bool,
    pub variable_multisample_rate: bool,
    pub inherited_queries: bool,
}

impl<'a> From<&'a vks::VkPhysicalDeviceFeatures> for PhysicalDeviceFeatures {
    fn from(featurs: &'a vks::VkPhysicalDeviceFeatures) -> Self {
        PhysicalDeviceFeatures {
            robust_buffer_access: utils::from_vk_bool(featurs.robustBufferAccess),
            full_draw_index_uint32: utils::from_vk_bool(featurs.fullDrawIndexUint32),
            image_cube_array: utils::from_vk_bool(featurs.imageCubeArray),
            independent_blend: utils::from_vk_bool(featurs.independentBlend),
            geometry_shader: utils::from_vk_bool(featurs.geometryShader),
            tessellation_shader: utils::from_vk_bool(featurs.tessellationShader),
            sample_rate_shading: utils::from_vk_bool(featurs.sampleRateShading),
            dual_src_blend: utils::from_vk_bool(featurs.dualSrcBlend),
            logic_op: utils::from_vk_bool(featurs.logicOp),
            multi_draw_indirect: utils::from_vk_bool(featurs.multiDrawIndirect),
            draw_indirect_first_instance: utils::from_vk_bool(featurs.drawIndirectFirstInstance),
            depth_clamp: utils::from_vk_bool(featurs.depthClamp),
            depth_bias_clamp: utils::from_vk_bool(featurs.depthBiasClamp),
            fill_mode_non_solid: utils::from_vk_bool(featurs.fillModeNonSolid),
            depth_bounds: utils::from_vk_bool(featurs.depthBounds),
            wide_lines: utils::from_vk_bool(featurs.wideLines),
            large_points: utils::from_vk_bool(featurs.largePoints),
            alpha_to_one: utils::from_vk_bool(featurs.alphaToOne),
            multi_viewport: utils::from_vk_bool(featurs.multiViewport),
            sampler_anisotropy: utils::from_vk_bool(featurs.samplerAnisotropy),
            texture_compression_etc2: utils::from_vk_bool(featurs.textureCompressionETC2),
            texture_compression_astc_ldr: utils::from_vk_bool(featurs.textureCompressionASTC_LDR),
            texture_compression_bc: utils::from_vk_bool(featurs.textureCompressionBC),
            occlusion_query_precise: utils::from_vk_bool(featurs.occlusionQueryPrecise),
            pipeline_statistics_query: utils::from_vk_bool(featurs.pipelineStatisticsQuery),
            vertex_pipeline_stores_and_atomics: utils::from_vk_bool(featurs.vertexPipelineStoresAndAtomics),
            fragment_stores_and_atomics: utils::from_vk_bool(featurs.fragmentStoresAndAtomics),
            shader_tessellation_and_geometry_point_size: utils::from_vk_bool(featurs.shaderTessellationAndGeometryPointSize),
            shader_image_gather_extended: utils::from_vk_bool(featurs.shaderImageGatherExtended),
            shader_storage_image_extended_formats: utils::from_vk_bool(featurs.shaderStorageImageExtendedFormats),
            shader_storage_image_multisample: utils::from_vk_bool(featurs.shaderStorageImageMultisample),
            shader_storage_image_read_without_format: utils::from_vk_bool(featurs.shaderStorageImageReadWithoutFormat),
            shader_storage_image_write_without_format: utils::from_vk_bool(featurs.shaderStorageImageWriteWithoutFormat),
            shader_uniform_buffer_array_dynamic_indexing: utils::from_vk_bool(featurs.shaderUniformBufferArrayDynamicIndexing),
            shader_sampled_image_array_dynamic_indexing: utils::from_vk_bool(featurs.shaderSampledImageArrayDynamicIndexing),
            shader_storage_buffer_array_dynamic_indexing: utils::from_vk_bool(featurs.shaderStorageBufferArrayDynamicIndexing),
            shader_storage_image_array_dynamic_indexing: utils::from_vk_bool(featurs.shaderStorageImageArrayDynamicIndexing),
            shader_clip_distance: utils::from_vk_bool(featurs.shaderClipDistance),
            shader_cull_distance: utils::from_vk_bool(featurs.shaderCullDistance),
            shader_float64: utils::from_vk_bool(featurs.shaderFloat64),
            shader_int64: utils::from_vk_bool(featurs.shaderInt64),
            shader_int16: utils::from_vk_bool(featurs.shaderInt16),
            shader_resource_residency: utils::from_vk_bool(featurs.shaderResourceResidency),
            shader_resource_min_lod: utils::from_vk_bool(featurs.shaderResourceMinLod),
            sparse_binding: utils::from_vk_bool(featurs.sparseBinding),
            sparse_residency_buffer: utils::from_vk_bool(featurs.sparseResidencyBuffer),
            sparse_residency_image_2d: utils::from_vk_bool(featurs.sparseResidencyImage2D),
            sparse_residency_image_3d: utils::from_vk_bool(featurs.sparseResidencyImage3D),
            sparse_residency_2_samples: utils::from_vk_bool(featurs.sparseResidency2Samples),
            sparse_residency_4_samples: utils::from_vk_bool(featurs.sparseResidency4Samples),
            sparse_residency_8_samples: utils::from_vk_bool(featurs.sparseResidency8Samples),
            sparse_residency_16_samples: utils::from_vk_bool(featurs.sparseResidency16Samples),
            sparse_residency_aliased: utils::from_vk_bool(featurs.sparseResidencyAliased),
            variable_multisample_rate: utils::from_vk_bool(featurs.variableMultisampleRate),
            inherited_queries: utils::from_vk_bool(featurs.inheritedQueries),
        }
    }
}

impl<'a> From<&'a PhysicalDeviceFeatures> for vks::VkPhysicalDeviceFeatures {
    fn from(featurs: &'a PhysicalDeviceFeatures) -> Self {
        vks::VkPhysicalDeviceFeatures {
            robustBufferAccess: utils::to_vk_bool(featurs.robust_buffer_access),
            fullDrawIndexUint32: utils::to_vk_bool(featurs.full_draw_index_uint32),
            imageCubeArray: utils::to_vk_bool(featurs.image_cube_array),
            independentBlend: utils::to_vk_bool(featurs.independent_blend),
            geometryShader: utils::to_vk_bool(featurs.geometry_shader),
            tessellationShader: utils::to_vk_bool(featurs.tessellation_shader),
            sampleRateShading: utils::to_vk_bool(featurs.sample_rate_shading),
            dualSrcBlend: utils::to_vk_bool(featurs.dual_src_blend),
            logicOp: utils::to_vk_bool(featurs.logic_op),
            multiDrawIndirect: utils::to_vk_bool(featurs.multi_draw_indirect),
            drawIndirectFirstInstance: utils::to_vk_bool(featurs.draw_indirect_first_instance),
            depthClamp: utils::to_vk_bool(featurs.depth_clamp),
            depthBiasClamp: utils::to_vk_bool(featurs.depth_bias_clamp),
            fillModeNonSolid: utils::to_vk_bool(featurs.fill_mode_non_solid),
            depthBounds: utils::to_vk_bool(featurs.depth_bounds),
            wideLines: utils::to_vk_bool(featurs.wide_lines),
            largePoints: utils::to_vk_bool(featurs.large_points),
            alphaToOne: utils::to_vk_bool(featurs.alpha_to_one),
            multiViewport: utils::to_vk_bool(featurs.multi_viewport),
            samplerAnisotropy: utils::to_vk_bool(featurs.sampler_anisotropy),
            textureCompressionETC2: utils::to_vk_bool(featurs.texture_compression_etc2),
            textureCompressionASTC_LDR: utils::to_vk_bool(featurs.texture_compression_astc_ldr),
            textureCompressionBC: utils::to_vk_bool(featurs.texture_compression_bc),
            occlusionQueryPrecise: utils::to_vk_bool(featurs.occlusion_query_precise),
            pipelineStatisticsQuery: utils::to_vk_bool(featurs.pipeline_statistics_query),
            vertexPipelineStoresAndAtomics: utils::to_vk_bool(featurs.vertex_pipeline_stores_and_atomics),
            fragmentStoresAndAtomics: utils::to_vk_bool(featurs.fragment_stores_and_atomics),
            shaderTessellationAndGeometryPointSize: utils::to_vk_bool(featurs.shader_tessellation_and_geometry_point_size),
            shaderImageGatherExtended: utils::to_vk_bool(featurs.shader_image_gather_extended),
            shaderStorageImageExtendedFormats: utils::to_vk_bool(featurs.shader_storage_image_extended_formats),
            shaderStorageImageMultisample: utils::to_vk_bool(featurs.shader_storage_image_multisample),
            shaderStorageImageReadWithoutFormat: utils::to_vk_bool(featurs.shader_storage_image_read_without_format),
            shaderStorageImageWriteWithoutFormat: utils::to_vk_bool(featurs.shader_storage_image_write_without_format),
            shaderUniformBufferArrayDynamicIndexing: utils::to_vk_bool(featurs.shader_uniform_buffer_array_dynamic_indexing),
            shaderSampledImageArrayDynamicIndexing: utils::to_vk_bool(featurs.shader_sampled_image_array_dynamic_indexing),
            shaderStorageBufferArrayDynamicIndexing: utils::to_vk_bool(featurs.shader_storage_buffer_array_dynamic_indexing),
            shaderStorageImageArrayDynamicIndexing: utils::to_vk_bool(featurs.shader_storage_image_array_dynamic_indexing),
            shaderClipDistance: utils::to_vk_bool(featurs.shader_clip_distance),
            shaderCullDistance: utils::to_vk_bool(featurs.shader_cull_distance),
            shaderFloat64: utils::to_vk_bool(featurs.shader_float64),
            shaderInt64: utils::to_vk_bool(featurs.shader_int64),
            shaderInt16: utils::to_vk_bool(featurs.shader_int16),
            shaderResourceResidency: utils::to_vk_bool(featurs.shader_resource_residency),
            shaderResourceMinLod: utils::to_vk_bool(featurs.shader_resource_min_lod),
            sparseBinding: utils::to_vk_bool(featurs.sparse_binding),
            sparseResidencyBuffer: utils::to_vk_bool(featurs.sparse_residency_buffer),
            sparseResidencyImage2D: utils::to_vk_bool(featurs.sparse_residency_image_2d),
            sparseResidencyImage3D: utils::to_vk_bool(featurs.sparse_residency_image_3d),
            sparseResidency2Samples: utils::to_vk_bool(featurs.sparse_residency_2_samples),
            sparseResidency4Samples: utils::to_vk_bool(featurs.sparse_residency_4_samples),
            sparseResidency8Samples: utils::to_vk_bool(featurs.sparse_residency_8_samples),
            sparseResidency16Samples: utils::to_vk_bool(featurs.sparse_residency_16_samples),
            sparseResidencyAliased: utils::to_vk_bool(featurs.sparse_residency_aliased),
            variableMultisampleRate: utils::to_vk_bool(featurs.variable_multisample_rate),
            inheritedQueries: utils::to_vk_bool(featurs.inherited_queries),
        }
    }
}

/// See [`VkFormatProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatProperties)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct FormatProperties {
    pub linear_tiling_features: FormatFeatureFlags,
    pub optimal_tiling_features: FormatFeatureFlags,
    pub buffer_features: FormatFeatureFlags,
}

impl<'a> From<&'a vks::VkFormatProperties> for FormatProperties {
    fn from(properties: &'a vks::VkFormatProperties) -> Self {
        FormatProperties {
            linear_tiling_features: FormatFeatureFlags::from_bits_truncate(properties.linearTilingFeatures),
            optimal_tiling_features: FormatFeatureFlags::from_bits_truncate(properties.optimalTilingFeatures),
            buffer_features: FormatFeatureFlags::from_bits_truncate(properties.bufferFeatures),
        }
    }
}

impl<'a> From<&'a FormatProperties> for vks::VkFormatProperties {
    fn from(properties: &'a FormatProperties) -> Self {
        vks::VkFormatProperties {
            linearTilingFeatures: properties.linear_tiling_features.bits(),
            optimalTilingFeatures: properties.optimal_tiling_features.bits(),
            bufferFeatures: properties.buffer_features.bits(),
        }
    }
}

/// See [`VkExtent3D`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExtent3D)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Extent3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

impl<'a> From<&'a vks::VkExtent3D> for Extent3D {
    fn from(extent: &'a vks::VkExtent3D) -> Self {
        Extent3D {
            width: extent.width,
            height: extent.height,
            depth: extent.depth,
        }
    }
}

impl<'a> From<&'a Extent3D> for vks::VkExtent3D {
    fn from(extent: &'a Extent3D) -> Self {
        vks::VkExtent3D {
            width: extent.width,
            height: extent.height,
            depth: extent.depth,
        }
    }
}

impl Extent3D {
    /// Creates a new `Extent3D`.
    #[inline]
    pub fn new(width: u32, height: u32, depth: u32) -> Self {
        Extent3D {
            width: width,
            height: height,
            depth: depth,
        }
    }

    /// Creates an `Extent3D` with all components set to 0.
    #[inline]
    pub fn zero() -> Self {
        Extent3D {
            width: 0,
            height: 0,
            depth: 0,
        }
    }

    /// Creates an `Extent3D` from an `Extent2D` and the specified `depth` component.
    #[inline]
    pub fn from_2d(extent: &Extent2D, depth: u32) -> Self {
        Extent3D {
            width: extent.width,
            height: extent.height,
            depth: depth,
        }
    }
}

/// See [`VkImageFormatProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageFormatProperties)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ImageFormatProperties {
    pub max_extent: Extent3D,
    pub max_mip_levels: u32,
    pub max_array_layers: u32,
    pub sample_counts: SampleCountFlags,
    pub max_resource_size: u64,
}

impl<'a> From<&'a vks::VkImageFormatProperties> for ImageFormatProperties {
    fn from(properties: &'a vks::VkImageFormatProperties) -> Self {
        ImageFormatProperties {
            max_extent: (&properties.maxExtent).into(),
            max_mip_levels: properties.maxMipLevels,
            max_array_layers: properties.maxArrayLayers,
            sample_counts: SampleCountFlags::from_bits_truncate(properties.sampleCounts),
            max_resource_size: properties.maxResourceSize,
        }
    }
}

/// See [`VkPhysicalDeviceLimits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceLimits)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PhysicalDeviceLimits {
    pub max_image_dimension_1d: u32,
    pub max_image_dimension_2d: u32,
    pub max_image_dimension_3d: u32,
    pub max_image_dimension_cube: u32,
    pub max_image_array_layers: u32,
    pub max_texel_buffer_elements: u32,
    pub max_uniform_buffer_range: u32,
    pub max_storage_buffer_range: u32,
    pub max_push_constants_size: u32,
    pub max_memory_allocation_count: u32,
    pub max_sampler_allocation_count: u32,
    pub buffer_image_granularity: u64,
    pub sparse_address_space_size: u64,
    pub max_bound_descriptor_sets: u32,
    pub max_per_stage_descriptor_samplers: u32,
    pub max_per_stage_descriptor_uniform_buffers: u32,
    pub max_per_stage_descriptor_storage_buffers: u32,
    pub max_per_stage_descriptor_sampled_images: u32,
    pub max_per_stage_descriptor_storage_images: u32,
    pub max_per_stage_descriptor_input_attachments: u32,
    pub max_per_stage_resources: u32,
    pub max_descriptor_set_samplers: u32,
    pub max_descriptor_set_uniform_buffers: u32,
    pub max_descriptor_set_uniform_buffers_dynamic: u32,
    pub max_descriptor_set_storage_buffers: u32,
    pub max_descriptor_set_storage_buffers_dynamic: u32,
    pub max_descriptor_set_sampled_images: u32,
    pub max_descriptor_set_storage_images: u32,
    pub max_descriptor_set_input_attachments: u32,
    pub max_vertex_input_attributes: u32,
    pub max_vertex_input_bindings: u32,
    pub max_vertex_input_attribute_offset: u32,
    pub max_vertex_input_binding_stride: u32,
    pub max_vertex_output_components: u32,
    pub max_tessellation_generation_level: u32,
    pub max_tessellation_patch_size: u32,
    pub max_tessellation_control_per_vertex_input_components: u32,
    pub max_tessellation_control_per_vertex_output_components: u32,
    pub max_tessellation_control_per_patch_output_components: u32,
    pub max_tessellation_control_total_output_components: u32,
    pub max_tessellation_evaluation_input_components: u32,
    pub max_tessellation_evaluation_output_components: u32,
    pub max_geometry_shader_invocations: u32,
    pub max_geometry_input_components: u32,
    pub max_geometry_output_components: u32,
    pub max_geometry_output_vertices: u32,
    pub max_geometry_total_output_components: u32,
    pub max_fragment_input_components: u32,
    pub max_fragment_output_attachments: u32,
    pub max_fragment_dual_src_attachments: u32,
    pub max_fragment_combined_output_resources: u32,
    pub max_compute_shared_memory_size: u32,
    pub max_compute_work_group_count: [u32; 3],
    pub max_compute_work_group_invocations: u32,
    pub max_compute_work_group_size: [u32; 3],
    pub sub_pixel_precision_bits: u32,
    pub sub_texel_precision_bits: u32,
    pub mipmap_precision_bits: u32,
    pub max_draw_indexed_index_value: u32,
    pub max_draw_indirect_count: u32,
    pub max_sampler_lod_bias: f32,
    pub max_sampler_anisotropy: f32,
    pub max_viewports: u32,
    pub max_viewport_dimensions: [u32; 2],
    pub viewport_bounds_range: [f32; 2],
    pub viewport_sub_pixel_bits: u32,
    pub min_memory_map_alignment: usize,
    pub min_texel_buffer_offset_alignment: u64,
    pub min_uniform_buffer_offset_alignment: u64,
    pub min_storage_buffer_offset_alignment: u64,
    pub min_texel_offset: i32,
    pub max_texel_offset: u32,
    pub min_texel_gather_offset: i32,
    pub max_texel_gather_offset: u32,
    pub min_interpolation_offset: f32,
    pub max_interpolation_offset: f32,
    pub sub_pixel_interpolation_offset_bits: u32,
    pub max_framebuffer_width: u32,
    pub max_framebuffer_height: u32,
    pub max_framebuffer_layers: u32,
    pub framebuffer_color_sample_counts: SampleCountFlags,
    pub framebuffer_depth_sample_counts: SampleCountFlags,
    pub framebuffer_stencil_sample_counts: SampleCountFlags,
    pub framebuffer_no_attachments_sample_counts: SampleCountFlags,
    pub max_color_attachments: u32,
    pub sampled_image_color_sample_counts: SampleCountFlags,
    pub sampled_image_integer_sample_counts: SampleCountFlags,
    pub sampled_image_depth_sample_counts: SampleCountFlags,
    pub sampled_image_stencil_sample_counts: SampleCountFlags,
    pub storage_image_sample_counts: SampleCountFlags,
    pub max_sample_mask_words: u32,
    pub timestamp_compute_and_graphics: bool,
    pub timestamp_period: f32,
    pub max_clip_distances: u32,
    pub max_cull_distances: u32,
    pub max_combined_clip_and_cull_distances: u32,
    pub discrete_queue_priorities: u32,
    pub point_size_range: [f32; 2],
    pub line_width_range: [f32; 2],
    pub point_size_granularity: f32,
    pub line_width_granularity: f32,
    pub strict_lines: bool,
    pub standard_sample_locations: bool,
    pub optimal_buffer_copy_offset_alignment: u64,
    pub optimal_buffer_copy_row_pitch_alignment: u64,
    pub non_coherent_atom_size: u64,
}

impl<'a> From<&'a vks::VkPhysicalDeviceLimits> for PhysicalDeviceLimits {
    fn from(limits: &'a vks::VkPhysicalDeviceLimits) -> Self {
        PhysicalDeviceLimits {
            max_image_dimension_1d: limits.maxImageDimension1D,
            max_image_dimension_2d: limits.maxImageDimension2D,
            max_image_dimension_3d: limits.maxImageDimension3D,
            max_image_dimension_cube: limits.maxImageDimensionCube,
            max_image_array_layers: limits.maxImageArrayLayers,
            max_texel_buffer_elements: limits.maxTexelBufferElements,
            max_uniform_buffer_range: limits.maxUniformBufferRange,
            max_storage_buffer_range: limits.maxStorageBufferRange,
            max_push_constants_size: limits.maxPushConstantsSize,
            max_memory_allocation_count: limits.maxMemoryAllocationCount,
            max_sampler_allocation_count: limits.maxSamplerAllocationCount,
            buffer_image_granularity: limits.bufferImageGranularity,
            sparse_address_space_size: limits.sparseAddressSpaceSize,
            max_bound_descriptor_sets: limits.maxBoundDescriptorSets,
            max_per_stage_descriptor_samplers: limits.maxPerStageDescriptorSamplers,
            max_per_stage_descriptor_uniform_buffers: limits.maxPerStageDescriptorUniformBuffers,
            max_per_stage_descriptor_storage_buffers: limits.maxPerStageDescriptorStorageBuffers,
            max_per_stage_descriptor_sampled_images: limits.maxPerStageDescriptorSampledImages,
            max_per_stage_descriptor_storage_images: limits.maxPerStageDescriptorStorageImages,
            max_per_stage_descriptor_input_attachments: limits.maxPerStageDescriptorInputAttachments,
            max_per_stage_resources: limits.maxPerStageResources,
            max_descriptor_set_samplers: limits.maxDescriptorSetSamplers,
            max_descriptor_set_uniform_buffers: limits.maxDescriptorSetUniformBuffers,
            max_descriptor_set_uniform_buffers_dynamic: limits.maxDescriptorSetUniformBuffersDynamic,
            max_descriptor_set_storage_buffers: limits.maxDescriptorSetStorageBuffers,
            max_descriptor_set_storage_buffers_dynamic: limits.maxDescriptorSetStorageBuffersDynamic,
            max_descriptor_set_sampled_images: limits.maxDescriptorSetSampledImages,
            max_descriptor_set_storage_images: limits.maxDescriptorSetStorageImages,
            max_descriptor_set_input_attachments: limits.maxDescriptorSetInputAttachments,
            max_vertex_input_attributes: limits.maxVertexInputAttributes,
            max_vertex_input_bindings: limits.maxVertexInputBindings,
            max_vertex_input_attribute_offset: limits.maxVertexInputAttributeOffset,
            max_vertex_input_binding_stride: limits.maxVertexInputBindingStride,
            max_vertex_output_components: limits.maxVertexOutputComponents,
            max_tessellation_generation_level: limits.maxTessellationGenerationLevel,
            max_tessellation_patch_size: limits.maxTessellationPatchSize,
            max_tessellation_control_per_vertex_input_components: limits.maxTessellationControlPerVertexInputComponents,
            max_tessellation_control_per_vertex_output_components: limits.maxTessellationControlPerVertexOutputComponents,
            max_tessellation_control_per_patch_output_components: limits.maxTessellationControlPerPatchOutputComponents,
            max_tessellation_control_total_output_components: limits.maxTessellationControlTotalOutputComponents,
            max_tessellation_evaluation_input_components: limits.maxTessellationEvaluationInputComponents,
            max_tessellation_evaluation_output_components: limits.maxTessellationEvaluationOutputComponents,
            max_geometry_shader_invocations: limits.maxGeometryShaderInvocations,
            max_geometry_input_components: limits.maxGeometryInputComponents,
            max_geometry_output_components: limits.maxGeometryOutputComponents,
            max_geometry_output_vertices: limits.maxGeometryOutputVertices,
            max_geometry_total_output_components: limits.maxGeometryTotalOutputComponents,
            max_fragment_input_components: limits.maxFragmentInputComponents,
            max_fragment_output_attachments: limits.maxFragmentOutputAttachments,
            max_fragment_dual_src_attachments: limits.maxFragmentDualSrcAttachments,
            max_fragment_combined_output_resources: limits.maxFragmentCombinedOutputResources,
            max_compute_shared_memory_size: limits.maxComputeSharedMemorySize,
            max_compute_work_group_count: limits.maxComputeWorkGroupCount,
            max_compute_work_group_invocations: limits.maxComputeWorkGroupInvocations,
            max_compute_work_group_size: limits.maxComputeWorkGroupSize,
            sub_pixel_precision_bits: limits.subPixelPrecisionBits,
            sub_texel_precision_bits: limits.subTexelPrecisionBits,
            mipmap_precision_bits: limits.mipmapPrecisionBits,
            max_draw_indexed_index_value: limits.maxDrawIndexedIndexValue,
            max_draw_indirect_count: limits.maxDrawIndirectCount,
            max_sampler_lod_bias: limits.maxSamplerLodBias,
            max_sampler_anisotropy: limits.maxSamplerAnisotropy,
            max_viewports: limits.maxViewports,
            max_viewport_dimensions: limits.maxViewportDimensions,
            viewport_bounds_range: limits.viewportBoundsRange,
            viewport_sub_pixel_bits: limits.viewportSubPixelBits,
            min_memory_map_alignment: limits.minMemoryMapAlignment,
            min_texel_buffer_offset_alignment: limits.minTexelBufferOffsetAlignment,
            min_uniform_buffer_offset_alignment: limits.minUniformBufferOffsetAlignment,
            min_storage_buffer_offset_alignment: limits.minStorageBufferOffsetAlignment,
            min_texel_offset: limits.minTexelOffset,
            max_texel_offset: limits.maxTexelOffset,
            min_texel_gather_offset: limits.minTexelGatherOffset,
            max_texel_gather_offset: limits.maxTexelGatherOffset,
            min_interpolation_offset: limits.minInterpolationOffset,
            max_interpolation_offset: limits.maxInterpolationOffset,
            sub_pixel_interpolation_offset_bits: limits.subPixelInterpolationOffsetBits,
            max_framebuffer_width: limits.maxFramebufferWidth,
            max_framebuffer_height: limits.maxFramebufferHeight,
            max_framebuffer_layers: limits.maxFramebufferLayers,
            framebuffer_color_sample_counts: SampleCountFlags::from_bits_truncate(limits.framebufferColorSampleCounts),
            framebuffer_depth_sample_counts: SampleCountFlags::from_bits_truncate(limits.framebufferDepthSampleCounts),
            framebuffer_stencil_sample_counts: SampleCountFlags::from_bits_truncate(limits.framebufferStencilSampleCounts),
            framebuffer_no_attachments_sample_counts: SampleCountFlags::from_bits_truncate(limits.framebufferNoAttachmentsSampleCounts),
            max_color_attachments: limits.maxColorAttachments,
            sampled_image_color_sample_counts: SampleCountFlags::from_bits_truncate(limits.sampledImageColorSampleCounts),
            sampled_image_integer_sample_counts: SampleCountFlags::from_bits_truncate(limits.sampledImageIntegerSampleCounts),
            sampled_image_depth_sample_counts: SampleCountFlags::from_bits_truncate(limits.sampledImageDepthSampleCounts),
            sampled_image_stencil_sample_counts: SampleCountFlags::from_bits_truncate(limits.sampledImageStencilSampleCounts),
            storage_image_sample_counts: SampleCountFlags::from_bits_truncate(limits.storageImageSampleCounts),
            max_sample_mask_words: limits.maxSampleMaskWords,
            timestamp_compute_and_graphics: utils::from_vk_bool(limits.timestampComputeAndGraphics),
            timestamp_period: limits.timestampPeriod,
            max_clip_distances: limits.maxClipDistances,
            max_cull_distances: limits.maxCullDistances,
            max_combined_clip_and_cull_distances: limits.maxCombinedClipAndCullDistances,
            discrete_queue_priorities: limits.discreteQueuePriorities,
            point_size_range: limits.pointSizeRange,
            line_width_range: limits.lineWidthRange,
            point_size_granularity: limits.pointSizeGranularity,
            line_width_granularity: limits.lineWidthGranularity,
            strict_lines: utils::from_vk_bool(limits.strictLines),
            standard_sample_locations: utils::from_vk_bool(limits.standardSampleLocations),
            optimal_buffer_copy_offset_alignment: limits.optimalBufferCopyOffsetAlignment,
            optimal_buffer_copy_row_pitch_alignment: limits.optimalBufferCopyRowPitchAlignment,
            non_coherent_atom_size: limits.nonCoherentAtomSize,
        }
    }
}

impl<'a> From<&'a PhysicalDeviceLimits> for vks::VkPhysicalDeviceLimits {
    fn from(limits: &'a PhysicalDeviceLimits) -> Self {
        vks::VkPhysicalDeviceLimits {
            maxImageDimension1D: limits.max_image_dimension_1d,
            maxImageDimension2D: limits.max_image_dimension_2d,
            maxImageDimension3D: limits.max_image_dimension_3d,
            maxImageDimensionCube: limits.max_image_dimension_cube,
            maxImageArrayLayers: limits.max_image_array_layers,
            maxTexelBufferElements: limits.max_texel_buffer_elements,
            maxUniformBufferRange: limits.max_uniform_buffer_range,
            maxStorageBufferRange: limits.max_storage_buffer_range,
            maxPushConstantsSize: limits.max_push_constants_size,
            maxMemoryAllocationCount: limits.max_memory_allocation_count,
            maxSamplerAllocationCount: limits.max_sampler_allocation_count,
            bufferImageGranularity: limits.buffer_image_granularity,
            sparseAddressSpaceSize: limits.sparse_address_space_size,
            maxBoundDescriptorSets: limits.max_bound_descriptor_sets,
            maxPerStageDescriptorSamplers: limits.max_per_stage_descriptor_samplers,
            maxPerStageDescriptorUniformBuffers: limits.max_per_stage_descriptor_uniform_buffers,
            maxPerStageDescriptorStorageBuffers: limits.max_per_stage_descriptor_storage_buffers,
            maxPerStageDescriptorSampledImages: limits.max_per_stage_descriptor_sampled_images,
            maxPerStageDescriptorStorageImages: limits.max_per_stage_descriptor_storage_images,
            maxPerStageDescriptorInputAttachments: limits.max_per_stage_descriptor_input_attachments,
            maxPerStageResources: limits.max_per_stage_resources,
            maxDescriptorSetSamplers: limits.max_descriptor_set_samplers,
            maxDescriptorSetUniformBuffers: limits.max_descriptor_set_uniform_buffers,
            maxDescriptorSetUniformBuffersDynamic: limits.max_descriptor_set_uniform_buffers_dynamic,
            maxDescriptorSetStorageBuffers: limits.max_descriptor_set_storage_buffers,
            maxDescriptorSetStorageBuffersDynamic: limits.max_descriptor_set_storage_buffers_dynamic,
            maxDescriptorSetSampledImages: limits.max_descriptor_set_sampled_images,
            maxDescriptorSetStorageImages: limits.max_descriptor_set_storage_images,
            maxDescriptorSetInputAttachments: limits.max_descriptor_set_input_attachments,
            maxVertexInputAttributes: limits.max_vertex_input_attributes,
            maxVertexInputBindings: limits.max_vertex_input_bindings,
            maxVertexInputAttributeOffset: limits.max_vertex_input_attribute_offset,
            maxVertexInputBindingStride: limits.max_vertex_input_binding_stride,
            maxVertexOutputComponents: limits.max_vertex_output_components,
            maxTessellationGenerationLevel: limits.max_tessellation_generation_level,
            maxTessellationPatchSize: limits.max_tessellation_patch_size,
            maxTessellationControlPerVertexInputComponents: limits.max_tessellation_control_per_vertex_input_components,
            maxTessellationControlPerVertexOutputComponents: limits.max_tessellation_control_per_vertex_output_components,
            maxTessellationControlPerPatchOutputComponents: limits.max_tessellation_control_per_patch_output_components,
            maxTessellationControlTotalOutputComponents: limits.max_tessellation_control_total_output_components,
            maxTessellationEvaluationInputComponents: limits.max_tessellation_evaluation_input_components,
            maxTessellationEvaluationOutputComponents: limits.max_tessellation_evaluation_output_components,
            maxGeometryShaderInvocations: limits.max_geometry_shader_invocations,
            maxGeometryInputComponents: limits.max_geometry_input_components,
            maxGeometryOutputComponents: limits.max_geometry_output_components,
            maxGeometryOutputVertices: limits.max_geometry_output_vertices,
            maxGeometryTotalOutputComponents: limits.max_geometry_total_output_components,
            maxFragmentInputComponents: limits.max_fragment_input_components,
            maxFragmentOutputAttachments: limits.max_fragment_output_attachments,
            maxFragmentDualSrcAttachments: limits.max_fragment_dual_src_attachments,
            maxFragmentCombinedOutputResources: limits.max_fragment_combined_output_resources,
            maxComputeSharedMemorySize: limits.max_compute_shared_memory_size,
            maxComputeWorkGroupCount: limits.max_compute_work_group_count,
            maxComputeWorkGroupInvocations: limits.max_compute_work_group_invocations,
            maxComputeWorkGroupSize: limits.max_compute_work_group_size,
            subPixelPrecisionBits: limits.sub_pixel_precision_bits,
            subTexelPrecisionBits: limits.sub_texel_precision_bits,
            mipmapPrecisionBits: limits.mipmap_precision_bits,
            maxDrawIndexedIndexValue: limits.max_draw_indexed_index_value,
            maxDrawIndirectCount: limits.max_draw_indirect_count,
            maxSamplerLodBias: limits.max_sampler_lod_bias,
            maxSamplerAnisotropy: limits.max_sampler_anisotropy,
            maxViewports: limits.max_viewports,
            maxViewportDimensions: limits.max_viewport_dimensions,
            viewportBoundsRange: limits.viewport_bounds_range,
            viewportSubPixelBits: limits.viewport_sub_pixel_bits,
            minMemoryMapAlignment: limits.min_memory_map_alignment,
            minTexelBufferOffsetAlignment: limits.min_texel_buffer_offset_alignment,
            minUniformBufferOffsetAlignment: limits.min_uniform_buffer_offset_alignment,
            minStorageBufferOffsetAlignment: limits.min_storage_buffer_offset_alignment,
            minTexelOffset: limits.min_texel_offset,
            maxTexelOffset: limits.max_texel_offset,
            minTexelGatherOffset: limits.min_texel_gather_offset,
            maxTexelGatherOffset: limits.max_texel_gather_offset,
            minInterpolationOffset: limits.min_interpolation_offset,
            maxInterpolationOffset: limits.max_interpolation_offset,
            subPixelInterpolationOffsetBits: limits.sub_pixel_interpolation_offset_bits,
            maxFramebufferWidth: limits.max_framebuffer_width,
            maxFramebufferHeight: limits.max_framebuffer_height,
            maxFramebufferLayers: limits.max_framebuffer_layers,
            framebufferColorSampleCounts: limits.framebuffer_color_sample_counts.bits(),
            framebufferDepthSampleCounts: limits.framebuffer_depth_sample_counts.bits(),
            framebufferStencilSampleCounts: limits.framebuffer_stencil_sample_counts.bits(),
            framebufferNoAttachmentsSampleCounts: limits.framebuffer_no_attachments_sample_counts.bits(),
            maxColorAttachments: limits.max_color_attachments,
            sampledImageColorSampleCounts: limits.sampled_image_color_sample_counts.bits(),
            sampledImageIntegerSampleCounts: limits.sampled_image_integer_sample_counts.bits(),
            sampledImageDepthSampleCounts: limits.sampled_image_depth_sample_counts.bits(),
            sampledImageStencilSampleCounts: limits.sampled_image_stencil_sample_counts.bits(),
            storageImageSampleCounts: limits.storage_image_sample_counts.bits(),
            maxSampleMaskWords: limits.max_sample_mask_words,
            timestampComputeAndGraphics: utils::to_vk_bool(limits.timestamp_compute_and_graphics),
            timestampPeriod: limits.timestamp_period,
            maxClipDistances: limits.max_clip_distances,
            maxCullDistances: limits.max_cull_distances,
            maxCombinedClipAndCullDistances: limits.max_combined_clip_and_cull_distances,
            discreteQueuePriorities: limits.discrete_queue_priorities,
            pointSizeRange: limits.point_size_range,
            lineWidthRange: limits.line_width_range,
            pointSizeGranularity: limits.point_size_granularity,
            lineWidthGranularity: limits.line_width_granularity,
            strictLines: utils::to_vk_bool(limits.strict_lines),
            standardSampleLocations: utils::to_vk_bool(limits.standard_sample_locations),
            optimalBufferCopyOffsetAlignment: limits.optimal_buffer_copy_offset_alignment,
            optimalBufferCopyRowPitchAlignment: limits.optimal_buffer_copy_row_pitch_alignment,
            nonCoherentAtomSize: limits.non_coherent_atom_size,
        }
    }
}

/// See [`VkPhysicalDeviceSparseProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceSparseProperties)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct PhysicalDeviceSparseProperties {
    pub residency_standard_2d_block_shape: bool,
    pub residency_standard_2d_multisample_block_shape: bool,
    pub residency_standard_3d_block_shape: bool,
    pub residency_aligned_mip_size: bool,
    pub residency_non_resident_strict: bool,
}

impl<'a> From<&'a vks::VkPhysicalDeviceSparseProperties> for PhysicalDeviceSparseProperties {
    fn from(properties: &'a vks::VkPhysicalDeviceSparseProperties) -> Self {
        PhysicalDeviceSparseProperties {
            residency_standard_2d_block_shape: utils::from_vk_bool(properties.residencyStandard2DBlockShape),
            residency_standard_2d_multisample_block_shape: utils::from_vk_bool(properties.residencyStandard2DMultisampleBlockShape),
            residency_standard_3d_block_shape: utils::from_vk_bool(properties.residencyStandard3DBlockShape),
            residency_aligned_mip_size: utils::from_vk_bool(properties.residencyAlignedMipSize),
            residency_non_resident_strict: utils::from_vk_bool(properties.residencyNonResidentStrict),
        }
    }
}

impl<'a> From<&'a PhysicalDeviceSparseProperties> for vks::VkPhysicalDeviceSparseProperties {
    fn from(properties: &'a PhysicalDeviceSparseProperties) -> Self {
        vks::VkPhysicalDeviceSparseProperties {
            residencyStandard2DBlockShape: utils::to_vk_bool(properties.residency_standard_2d_block_shape),
            residencyStandard2DMultisampleBlockShape: utils::to_vk_bool(properties.residency_standard_2d_multisample_block_shape),
            residencyStandard3DBlockShape: utils::to_vk_bool(properties.residency_standard_3d_block_shape),
            residencyAlignedMipSize: utils::to_vk_bool(properties.residency_aligned_mip_size),
            residencyNonResidentStrict: utils::to_vk_bool(properties.residency_non_resident_strict),
        }
    }
}

/// See [`VkPhysicalDeviceProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceProperties)
#[derive(Debug, Clone, PartialEq)]
pub struct PhysicalDeviceProperties {
    pub api_version: Version,
    pub driver_version: u32,
    pub vendor_id: u32,
    pub device_id: u32,
    pub device_type: PhysicalDeviceType,
    pub device_name: String,
    pub pipeline_cache_uuid: [u8; 16],
    pub limits: PhysicalDeviceLimits,
    pub sparse_properties: PhysicalDeviceSparseProperties,
}

impl<'a> From<&'a vks::VkPhysicalDeviceProperties> for PhysicalDeviceProperties {
    fn from(properties: &'a vks::VkPhysicalDeviceProperties) -> Self {
        let device_name = unsafe {
            CStr::from_ptr(properties.deviceName.as_ptr()).to_str().unwrap().to_owned()
        };

        PhysicalDeviceProperties {
            api_version: Version::from_api_version(properties.apiVersion),
            driver_version: properties.driverVersion,
            vendor_id: properties.vendorID,
            device_id: properties.deviceID,
            device_type: properties.deviceType.into(),
            device_name: device_name,
            pipeline_cache_uuid: properties.pipelineCacheUUID,
            limits: (&properties.limits).into(),
            sparse_properties: (&properties.sparseProperties).into(),
        }
    }
}

impl<'a> From<&'a PhysicalDeviceProperties> for vks::VkPhysicalDeviceProperties {
    fn from(properties: &'a PhysicalDeviceProperties) -> Self {
        debug_assert!(properties.device_name.len() < vks::VK_MAX_PHYSICAL_DEVICE_NAME_SIZE);

        let device_name = unsafe {
            let mut device_name: [c_char; vks::VK_MAX_PHYSICAL_DEVICE_NAME_SIZE] = mem::uninitialized();
            ptr::copy_nonoverlapping(properties.device_name.as_ptr(), device_name.as_mut_ptr() as *mut _, properties.device_name.len());
            device_name[properties.device_name.len()] = 0;
            device_name
        };

        vks::VkPhysicalDeviceProperties {
            apiVersion: properties.api_version.as_api_version(),
            driverVersion: properties.driver_version,
            vendorID: properties.vendor_id,
            deviceID: properties.device_id,
            deviceType: properties.device_type.into(),
            deviceName: device_name,
            pipelineCacheUUID: properties.pipeline_cache_uuid,
            limits: (&properties.limits).into(),
            sparseProperties: (&properties.sparse_properties).into(),
        }
    }
}

/// See [`VkQueueFamilyProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueueFamilyProperties)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct QueueFamilyProperties {
    pub queue_flags: QueueFlags,
    pub queue_count: u32,
    pub timestamp_valid_bits: u32,
    pub min_image_transfer_granularity: Extent3D,
}

impl<'a> From<&'a vks::VkQueueFamilyProperties> for QueueFamilyProperties {
    fn from(properties: &'a vks::VkQueueFamilyProperties) -> Self {
        QueueFamilyProperties {
            queue_flags: QueueFlags::from_bits_truncate(properties.queueFlags),
            queue_count: properties.queueCount,
            timestamp_valid_bits: properties.timestampValidBits,
            min_image_transfer_granularity: (&properties.minImageTransferGranularity).into(),
        }
    }
}

impl<'a> From<&'a QueueFamilyProperties> for vks::VkQueueFamilyProperties {
    fn from(properties: &'a QueueFamilyProperties) -> Self {
        vks::VkQueueFamilyProperties {
            queueFlags: properties.queue_flags.bits(),
            queueCount: properties.queue_count,
            timestampValidBits: properties.timestamp_valid_bits,
            minImageTransferGranularity: (&properties.min_image_transfer_granularity).into(),
        }
    }
}

/// See [`VkMemoryType`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryType)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct MemoryType {
    pub property_flags: MemoryPropertyFlags,
    pub heap_index: u32,
}

impl<'a> From<&'a vks::VkMemoryType> for MemoryType {
    fn from(memory_type: &'a vks::VkMemoryType) -> Self {
        MemoryType {
            property_flags: MemoryPropertyFlags::from_bits_truncate(memory_type.propertyFlags),
            heap_index: memory_type.heapIndex,
        }
    }
}

impl<'a> From<&'a MemoryType> for vks::VkMemoryType {
    fn from(memory_type: &'a MemoryType) -> Self {
        vks::VkMemoryType {
            propertyFlags: memory_type.property_flags.bits(),
            heapIndex: memory_type.heap_index,
        }
    }
}

/// See [`VkMemoryHeap`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryHeap)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct MemoryHeap {
    pub size: u64,
    pub flags: MemoryHeapFlags,
}

impl<'a> From<&'a vks::VkMemoryHeap> for MemoryHeap {
    fn from(heap: &'a vks::VkMemoryHeap) -> Self {
        MemoryHeap {
            size: heap.size,
            flags: MemoryHeapFlags::from_bits_truncate(heap.flags),
        }
    }
}

impl<'a> From<&'a MemoryHeap> for vks::VkMemoryHeap {
    fn from(heap: &'a MemoryHeap) -> Self {
        vks::VkMemoryHeap {
            size: heap.size,
            flags: heap.flags.bits(),
        }
    }
}

/// See [`VkPhysicalDeviceMemoryProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDeviceMemoryProperties)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhysicalDeviceMemoryProperties {
    pub memory_types: Vec<MemoryType>,
    pub memory_heaps: Vec<MemoryHeap>,
}

impl<'a> From<&'a vks::VkPhysicalDeviceMemoryProperties> for PhysicalDeviceMemoryProperties {
    fn from(properties: &'a vks::VkPhysicalDeviceMemoryProperties) -> Self {
        let memory_types = properties.memoryTypes[..properties.memoryTypeCount as usize]
            .iter()
            .map(From::from)
            .collect();

        let memory_heaps = properties.memoryHeaps[..properties.memoryHeapCount as usize]
            .iter()
            .map(From::from)
            .collect();

        PhysicalDeviceMemoryProperties {
            memory_types: memory_types,
            memory_heaps: memory_heaps,
        }
    }
}

impl<'a> From<&'a PhysicalDeviceMemoryProperties> for vks::VkPhysicalDeviceMemoryProperties {
    fn from(properties: &'a PhysicalDeviceMemoryProperties) -> Self {
        debug_assert!(properties.memory_types.len() <= vks::VK_MAX_MEMORY_TYPES);
        debug_assert!(properties.memory_heaps.len() <= vks::VK_MAX_MEMORY_HEAPS);

        let mut res = vks::VkPhysicalDeviceMemoryProperties {
            memoryTypeCount: properties.memory_types.len() as u32,
            memoryTypes: unsafe { mem::uninitialized() },
            memoryHeapCount: properties.memory_heaps.len() as u32,
            memoryHeaps: unsafe { mem::uninitialized() },
        };

        for (vk_memory_type, memory_type) in res.memoryTypes.iter_mut().zip(&properties.memory_types) {
            *vk_memory_type = memory_type.into();
        }

        for (vk_memory_heap, memory_heap) in res.memoryHeaps.iter_mut().zip(&properties.memory_heaps) {
            *vk_memory_heap = memory_heap.into();
        }

        res
    }
}

gen_chain_struct! {
    name: DeviceQueueCreateInfoChain [DeviceQueueCreateInfoChainWrapper],
    query: DeviceQueueCreateInfoChainQuery [DeviceQueueCreateInfoChainQueryWrapper],
    vks: VkDeviceQueueCreateInfo,
    input: true,
    output: false,
}

/// See [`VkDeviceQueueCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceQueueCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceQueueCreateInfo {
    pub flags: DeviceQueueCreateFlags,
    pub queue_family_index: u32,
    pub queue_priorities: Vec<f32>,
    pub chain: Option<DeviceQueueCreateInfoChain>,
}

#[derive(Debug)]
struct VkDeviceQueueCreateInfoWrapper {
    pub vks_struct: vks::VkDeviceQueueCreateInfo,
    queue_priorities: Vec<f32>,
    chain: Option<DeviceQueueCreateInfoChainWrapper>,
}

impl VkDeviceQueueCreateInfoWrapper {
    pub fn new(create_info: &DeviceQueueCreateInfo, with_chain: bool) -> Self {
        let queue_priorities = create_info.queue_priorities.clone();
        let (pnext, chain) = DeviceQueueCreateInfoChainWrapper::new_optional(&create_info.chain, with_chain);

        VkDeviceQueueCreateInfoWrapper {
            vks_struct: vks::VkDeviceQueueCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
                pNext: pnext,
                flags: create_info.flags.bits(),
                queueFamilyIndex: create_info.queue_family_index,
                queueCount: queue_priorities.len() as u32,
                pQueuePriorities: queue_priorities.as_ptr(),
            },
            queue_priorities: queue_priorities,
            chain: chain,
        }
    }
}

gen_chain_struct! {
    name: DeviceCreateInfoChain [DeviceCreateInfoChainWrapper],
    query: DeviceCreateInfoChainQuery [DeviceCreateInfoChainQueryWrapper],
    vks: VkDeviceCreateInfo,
    input: true,
    output: false,

    physical_device_features2_khr: PhysicalDeviceFeatures2Khr {
        mod: khr_get_physical_device_properties2,
        fn_add: add_physical_device_features2_khr,
        fn_has: has_physical_device_features2_khr,
        fn_get: get_physical_device_features2_khr,
        wrapper: VkPhysicalDeviceFeatures2KHRWrapper,
        vks: VkPhysicalDeviceFeatures2KHR,
        stype: vks::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2_KHR,
    }
}

/// See [`VkDeviceCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceCreateInfo {
    pub flags: DeviceCreateFlags,
    pub queue_create_infos: Vec<DeviceQueueCreateInfo>,
    pub enabled_layers: Vec<String>,
    pub enabled_extensions: DeviceExtensions,
    pub enabled_features: Option<PhysicalDeviceFeatures>,
    pub chain: Option<DeviceCreateInfoChain>,
}

#[derive(Debug)]
struct VkDeviceCreateInfoWrapper {
    pub vks_struct: vks::VkDeviceCreateInfo,
    queue_create_infos_wrappers: Vec<VkDeviceQueueCreateInfoWrapper>,
    queue_create_infos: Vec<vks::VkDeviceQueueCreateInfo>,
    enabled_layers: Vec<CString>,
    enabled_layers_ptrs: Vec<*const c_char>,
    enabled_extensions: Vec<CString>,
    enabled_extensions_ptrs: Vec<*const c_char>,
    enabled_features: Option<Box<vks::VkPhysicalDeviceFeatures>>,
    chain: Option<DeviceCreateInfoChainWrapper>,
}

impl VkDeviceCreateInfoWrapper {
    pub fn new(create_info: &DeviceCreateInfo, with_chain: bool) -> Self {
        let queue_create_infos_wrappers: Vec<_> = create_info.queue_create_infos
            .iter()
            .map(|q| VkDeviceQueueCreateInfoWrapper::new(q, true))
            .collect();

        let queue_create_infos: Vec<vks::VkDeviceQueueCreateInfo> = queue_create_infos_wrappers
            .iter()
            .map(|q| q.vks_struct)
            .collect();

        let enabled_layers: Vec<_> = create_info.enabled_layers
            .iter()
            .cloned()
            .map(CString::new)
            .map(Result::unwrap)
            .collect();
        let enabled_layers_ptrs: Vec<_> = enabled_layers
            .iter()
            .map(|l| l.as_ptr())
            .collect();
        let enabled_layers_ptr = if !enabled_layers_ptrs.is_empty() {
            enabled_layers_ptrs.as_ptr()
        }
        else {
            ptr::null()
        };

        let enabled_extensions = create_info.enabled_extensions.to_cstring_vec();
        let enabled_extensions_ptrs: Vec<_> = enabled_extensions
            .iter()
            .map(|l| l.as_ptr())
            .collect();
        let enabled_extensions_ptr = if !enabled_extensions_ptrs.is_empty() {
            enabled_extensions_ptrs.as_ptr()
        }
        else {
            ptr::null()
        };

        let enabled_features_ptr;
        let enabled_features = match create_info.enabled_features {
            Some(ref enabled_features) => {
                let enabled_features: Box<vks::VkPhysicalDeviceFeatures> = Box::new(enabled_features.into());
                enabled_features_ptr = &*enabled_features as *const _;
                Some(enabled_features)
            }

            None => {
                enabled_features_ptr = ptr::null();
                None
            }
        };

        let (pnext, chain) = DeviceCreateInfoChainWrapper::new_optional(&create_info.chain, with_chain);

        VkDeviceCreateInfoWrapper {
            vks_struct: vks::VkDeviceCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
                pNext: pnext,
                flags: create_info.flags.bits(),
                queueCreateInfoCount: queue_create_infos.len() as u32,
                pQueueCreateInfos: queue_create_infos.as_ptr(),
                enabledLayerCount: enabled_layers.len() as u32,
                ppEnabledLayerNames: enabled_layers_ptr,
                enabledExtensionCount: enabled_extensions.len() as u32,
                ppEnabledExtensionNames: enabled_extensions_ptr,
                pEnabledFeatures: enabled_features_ptr,
            },
            queue_create_infos_wrappers: queue_create_infos_wrappers,
            queue_create_infos: queue_create_infos,
            enabled_layers: enabled_layers,
            enabled_layers_ptrs: enabled_layers_ptrs,
            enabled_extensions: enabled_extensions,
            enabled_extensions_ptrs: enabled_extensions_ptrs,
            enabled_features: enabled_features,
            chain: chain,
        }
    }
}

gen_extension_structs!{
    pub struct InstanceExtensions;
    pub struct InstanceExtensionsProperties;

    ext_debug_report {
        name: vks::VK_EXT_DEBUG_REPORT_EXTENSION_NAME_STR,
        fn_add: add_ext_debug_report,
        fn_has: has_ext_debug_report,
        fn_get: get_ext_debug_report,
        load_instance: load_ext_debug_report,
    }

    khr_android_surface {
        name: vks::VK_KHR_ANDROID_SURFACE_EXTENSION_NAME_STR,
        fn_add: add_khr_android_surface,
        fn_has: has_khr_android_surface,
        fn_get: get_khr_android_surface,
        load_instance: load_khr_android_surface,
    }

    khr_display {
        name: vks::VK_KHR_DISPLAY_EXTENSION_NAME_STR,
        fn_add: add_khr_display,
        fn_has: has_khr_display,
        fn_get: get_khr_display,
        load_instance: load_khr_display,
    }

    khr_get_physical_device_properties2 {
        name: vks::VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME_STR,
        fn_add: add_khr_get_physical_device_properties2,
        fn_has: has_khr_get_physical_device_properties2,
        fn_get: get_khr_get_physical_device_properties2,
        load_instance: load_khr_get_physical_device_properties2,
    }

    khr_mir_surface {
        name: vks::VK_KHR_MIR_SURFACE_EXTENSION_NAME_STR,
        fn_add: add_khr_mir_surface,
        fn_has: has_khr_mir_surface,
        fn_get: get_khr_mir_surface,
        load_instance: load_khr_mir_surface,
    }

    khr_surface {
        name: vks::VK_KHR_SURFACE_EXTENSION_NAME_STR,
        fn_add: add_khr_surface,
        fn_has: has_khr_surface,
        fn_get: get_khr_surface,
        load_instance: load_khr_surface,
    }

    khr_wayland_surface {
        name: vks::VK_KHR_WAYLAND_SURFACE_EXTENSION_NAME_STR,
        fn_add: add_khr_wayland_surface,
        fn_has: has_khr_wayland_surface,
        fn_get: get_khr_wayland_surface,
        load_instance: load_khr_wayland_surface,
    }

    khr_win32_surface {
        name: vks::VK_KHR_WIN32_SURFACE_EXTENSION_NAME_STR,
        fn_add: add_khr_win32_surface,
        fn_has: has_khr_win32_surface,
        fn_get: get_khr_win32_surface,
        load_instance: load_khr_win32_surface,
    }

    khr_xcb_surface {
        name: vks::VK_KHR_XCB_SURFACE_EXTENSION_NAME_STR,
        fn_add: add_khr_xcb_surface,
        fn_has: has_khr_xcb_surface,
        fn_get: get_khr_xcb_surface,
        load_instance: load_khr_xcb_surface,
    }

    khr_xlib_surface {
        name: vks::VK_KHR_XLIB_SURFACE_EXTENSION_NAME_STR,
        fn_add: add_khr_xlib_surface,
        fn_has: has_khr_xlib_surface,
        fn_get: get_khr_xlib_surface,
        load_instance: load_khr_xlib_surface,
    }
}

gen_extension_structs!{
    pub struct DeviceExtensions;
    pub struct DeviceExtensionsProperties;

    amd_rasterization_order {
        name: vks::VK_AMD_RASTERIZATION_ORDER_EXTENSION_NAME_STR,
        fn_add: add_amd_rasterization_order,
        fn_has: has_amd_rasterization_order,
        fn_get: get_amd_rasterization_order,
    }

    img_filter_cubic {
        name: vks::VK_IMG_FILTER_CUBIC_EXTENSION_NAME_STR,
        fn_add: add_img_filter_cubic,
        fn_has: has_img_filter_cubic,
        fn_get: get_img_filter_cubic,
    }

    khr_swapchain {
        name: vks::VK_KHR_SWAPCHAIN_EXTENSION_NAME_STR,
        fn_add: add_khr_swapchain,
        fn_has: has_khr_swapchain,
        fn_get: get_khr_swapchain,
        load_device: load_khr_swapchain,
    }

    khr_display_swapchain {
        name: vks::VK_KHR_DISPLAY_SWAPCHAIN_EXTENSION_NAME_STR,
        fn_add: add_khr_display_swapchain,
        fn_has: has_khr_display_swapchain,
        fn_get: get_khr_display_swapchain,
        load_device: load_khr_display_swapchain,
    }

    khr_sampler_mirror_clamp_to_edge {
        name: vks::VK_KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_EXTENSION_NAME_STR,
        fn_add: add_khr_sampler_mirror_clamp_to_edge,
        fn_has: has_khr_sampler_mirror_clamp_to_edge,
        fn_get: get_khr_sampler_mirror_clamp_to_edge,
    }

    nv_glsl_shader {
        name: vks::VK_NV_GLSL_SHADER_EXTENSION_NAME_STR,
        fn_add: add_nv_glsl_shader,
        fn_has: has_nv_glsl_shader,
        fn_get: get_nv_glsl_shader,
    }
}

/// See [`VkLayerProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkLayerProperties)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LayerProperties {
    pub layer_name: String,
    pub spec_version: Version,
    pub implementation_version: u32,
    pub description: String,
}

impl<'a> From<&'a vks::VkLayerProperties> for LayerProperties {
    fn from(layer_properties: &'a vks::VkLayerProperties) -> Self {
        unsafe {
            LayerProperties {
                layer_name: CStr::from_ptr(layer_properties.layerName.as_ptr()).to_str().unwrap().to_owned(),
                spec_version: Version::from_api_version(layer_properties.specVersion),
                implementation_version: layer_properties.implementationVersion,
                description: CStr::from_ptr(layer_properties.description.as_ptr()).to_str().unwrap().to_owned(),
            }
        }
    }
}

gen_chain_struct! {
    name: SubmitInfoChain [SubmitInfoChainWrapper],
    query: SubmitInfoChainQuery [SubmitInfoChainQueryWrapper],
    vks: VkSubmitInfo,
    input: true,
    output: false,
}

/// See [`VkSubmitInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSubmitInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct SubmitInfo {
    pub wait_semaphores: Vec<Semaphore>,
    pub wait_dst_stage_mask: Vec<PipelineStageFlags>,
    pub command_buffers: Vec<CommandBuffer>,
    pub signal_semaphores: Vec<Semaphore>,
    pub chain: Option<SubmitInfoChain>,
}

#[derive(Debug)]
struct VkSubmitInfoWrapper {
    pub vks_struct: vks::VkSubmitInfo,
    wait_semaphores: Vec<Semaphore>,
    wait_vk_semaphores: Vec<vks::VkSemaphore>,
    wait_dst_stage_mask: Vec<vks::VkPipelineStageFlags>,
    command_buffers: Vec<CommandBuffer>,
    vk_command_buffers: Vec<vks::VkCommandBuffer>,
    signal_semaphores: Vec<Semaphore>,
    signal_vk_semaphores: Vec<vks::VkSemaphore>,
    chain: Option<SubmitInfoChainWrapper>,
}

impl VkSubmitInfoWrapper {
    pub fn new(info: &SubmitInfo, with_chain: bool) -> Self {
        let wait_semaphores = info.wait_semaphores.clone();
        let (wait_vk_semaphores_ptr, wait_vk_semaphores) = if !wait_semaphores.is_empty() {
            let wait_vk_semaphores: Vec<_> = wait_semaphores.iter().map(Semaphore::handle).collect();
            (wait_vk_semaphores.as_ptr(), wait_vk_semaphores)
        }
        else {
            (ptr::null(), vec![])
        };

        let wait_dst_stage_mask: Vec<_> = info.wait_dst_stage_mask.iter().map(PipelineStageFlags::bits).collect();

        let command_buffers = info.command_buffers.clone();
        let (vk_command_buffers_ptr, vk_command_buffers) = if !command_buffers.is_empty() {
            let vk_command_buffers: Vec<_> = command_buffers.iter().map(CommandBuffer::handle).collect();
            (vk_command_buffers.as_ptr(), vk_command_buffers)
        }
        else {
            (ptr::null(), vec![])
        };

        let signal_semaphores = info.signal_semaphores.clone();
        let (signal_vk_semaphores_ptr, signal_vk_semaphores) = if !signal_semaphores.is_empty() {
            let signal_vk_semaphores: Vec<_> = signal_semaphores.iter().map(Semaphore::handle).collect();
            (signal_vk_semaphores.as_ptr(), signal_vk_semaphores)
        }
        else {
            (ptr::null(), vec![])
        };

        let (pnext, chain) = SubmitInfoChainWrapper::new_optional(&info.chain, with_chain);

        VkSubmitInfoWrapper {
            vks_struct: vks::VkSubmitInfo {
                sType: vks::VK_STRUCTURE_TYPE_SUBMIT_INFO,
                pNext: pnext,
                waitSemaphoreCount: wait_semaphores.len() as u32,
                pWaitSemaphores: wait_vk_semaphores_ptr,
                pWaitDstStageMask: wait_dst_stage_mask.as_ptr(),
                commandBufferCount: command_buffers.len() as u32,
                pCommandBuffers: vk_command_buffers_ptr,
                signalSemaphoreCount: signal_semaphores.len() as u32,
                pSignalSemaphores: signal_vk_semaphores_ptr,
            },
            wait_semaphores: wait_semaphores,
            wait_vk_semaphores: wait_vk_semaphores,
            wait_dst_stage_mask: wait_dst_stage_mask,
            command_buffers: command_buffers,
            vk_command_buffers: vk_command_buffers,
            signal_semaphores: signal_semaphores,
            signal_vk_semaphores: signal_vk_semaphores,
            chain: chain,
        }
    }
}

gen_chain_struct! {
    name: MemoryAllocateInfoChain [MemoryAllocateInfoChainWrapper],
    query: MemoryAllocateInfoChainQuery [MemoryAllocateInfoChainQueryWrapper],
    vks: VkMemoryAllocateInfo,
    input: true,
    output: false,
}

/// See [`VkMemoryAllocateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryAllocateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct MemoryAllocateInfo {
    pub allocation_size: u64,
    pub memory_type_index: u32,
    pub chain: Option<MemoryAllocateInfoChain>,
}

#[derive(Debug)]
struct VkMemoryAllocateInfoWrapper {
    pub vks_struct: vks::VkMemoryAllocateInfo,
    chain: Option<MemoryAllocateInfoChainWrapper>,
}

impl VkMemoryAllocateInfoWrapper {
    pub fn new(info: &MemoryAllocateInfo, with_chain: bool) -> Self {
        let (pnext, chain) = MemoryAllocateInfoChainWrapper::new_optional(&info.chain, with_chain);

        VkMemoryAllocateInfoWrapper {
            vks_struct: vks::VkMemoryAllocateInfo {
                sType: vks::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
                pNext: pnext,
                allocationSize: info.allocation_size,
                memoryTypeIndex: info.memory_type_index,
            },
            chain: chain,
        }
    }
}

gen_chain_struct! {
    name: MappedMemoryRangeChain [MappedMemoryRangeChainWrapper],
    query: MappedMemoryRangeChainQuery [MappedMemoryRangeChainQueryWrapper],
    vks: VkMappedMemoryRange,
    input: true,
    output: false,
}

/// See [`VkMappedMemoryRange`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMappedMemoryRange)
#[derive(Debug, Clone, PartialEq)]
pub struct MappedMemoryRange {
    pub memory: DeviceMemory,
    pub offset: u64,
    pub size: OptionalDeviceSize,
    pub chain: Option<MappedMemoryRangeChain>,
}

#[derive(Debug)]
struct VkMappedMemoryRangeWrapper {
    pub vks_struct: vks::VkMappedMemoryRange,
    memory: DeviceMemory,
    chain: Option<MappedMemoryRangeChainWrapper>,
}

impl VkMappedMemoryRangeWrapper {
    pub fn new(range: &MappedMemoryRange, with_chain: bool) -> Self {
        let (pnext, chain) = MappedMemoryRangeChainWrapper::new_optional(&range.chain, with_chain);

        VkMappedMemoryRangeWrapper {
            vks_struct: vks::VkMappedMemoryRange {
                sType: vks::VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE,
                pNext: pnext,
                memory: range.memory.handle(),
                offset: range.offset,
                size: range.size.into(),
            },
            memory: range.memory.clone(),
            chain: chain,
        }
    }
}

/// See [`VkMemoryRequirements`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryRequirements)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct MemoryRequirements {
    pub size: u64,
    pub alignment: u64,
    pub memory_type_bits: u32,
}

impl<'a> From<&'a vks::VkMemoryRequirements> for MemoryRequirements {
    fn from(requirements: &'a vks::VkMemoryRequirements) -> Self {
        MemoryRequirements {
            size: requirements.size,
            alignment: requirements.alignment,
            memory_type_bits: requirements.memoryTypeBits,
        }
    }
}

/// See [`VkSparseImageFormatProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseImageFormatProperties)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SparseImageFormatProperties {
    pub aspect_mask: ImageAspectFlags,
    pub image_granularity: Extent3D,
    pub flags: SparseImageFormatFlags,
}

impl<'a> From<&'a vks::VkSparseImageFormatProperties> for SparseImageFormatProperties {
    fn from(properties: &'a vks::VkSparseImageFormatProperties) -> Self {
        SparseImageFormatProperties {
            aspect_mask: ImageAspectFlags::from_bits_truncate(properties.aspectMask),
            image_granularity: (&properties.imageGranularity).into(),
            flags: SparseImageFormatFlags::from_bits_truncate(properties.flags),
        }
    }
}

/// See [`VkSparseImageMemoryRequirements`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseImageMemoryRequirements)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SparseImageMemoryRequirements {
    pub format_properties: SparseImageFormatProperties,
    pub image_mip_tail_first_lod: u32,
    pub image_mip_tail_size: u64,
    pub image_mip_tail_offset: u64,
    pub image_mip_tail_stride: u64,
}

impl<'a> From<&'a vks::VkSparseImageMemoryRequirements> for SparseImageMemoryRequirements {
    fn from(requirements: &'a vks::VkSparseImageMemoryRequirements) -> Self {
        SparseImageMemoryRequirements {
            format_properties: (&requirements.formatProperties).into(),
            image_mip_tail_first_lod: requirements.imageMipTailFirstLod,
            image_mip_tail_size: requirements.imageMipTailSize,
            image_mip_tail_offset: requirements.imageMipTailOffset,
            image_mip_tail_stride: requirements.imageMipTailStride,
        }
    }
}

/// See [`VkSparseMemoryBind`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseMemoryBind)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SparseMemoryBind {
    pub resource_offset: u64,
    pub size: u64,
    pub memory: Option<DeviceMemory>,
    pub memory_offset: u64,
    pub flags: SparseMemoryBindFlags,
}

#[derive(Debug)]
struct VkSparseMemoryBindWrapper {
    pub vks_struct: vks::VkSparseMemoryBind,
    memory: Option<DeviceMemory>,
}

impl<'a> From<&'a SparseMemoryBind> for VkSparseMemoryBindWrapper {
    fn from(bind: &'a SparseMemoryBind) -> Self {
        let (vk_memory, memory) = match bind.memory {
            Some(ref memory) => (memory.handle(), Some(memory.clone())),
            None => (Default::default(), None),
        };

        VkSparseMemoryBindWrapper {
            vks_struct: vks::VkSparseMemoryBind {
                resourceOffset: bind.resource_offset,
                size: bind.size,
                memory: vk_memory,
                memoryOffset: bind.memory_offset,
                flags: bind.flags.bits(),
            },
            memory: memory,
        }
    }
}

/// See [`VkSparseBufferMemoryBindInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseBufferMemoryBindInfo)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SparseBufferMemoryBindInfo {
    pub buffer: Buffer,
    pub binds: Vec<SparseMemoryBind>,
}

#[derive(Debug)]
struct VkSparseBufferMemoryBindInfoWrapper {
    pub vks_struct: vks::VkSparseBufferMemoryBindInfo,
    buffer: Buffer,
    binds: Vec<VkSparseMemoryBindWrapper>,
    binds_vk: Vec<vks::VkSparseMemoryBind>,
}

impl<'a> From<&'a SparseBufferMemoryBindInfo> for VkSparseBufferMemoryBindInfoWrapper {
    fn from(info: &'a SparseBufferMemoryBindInfo) -> Self {
        let binds: Vec<VkSparseMemoryBindWrapper> = info.binds.iter().map(From::from).collect();
        let binds_vk: Vec<_> = binds.iter().map(|b| b.vks_struct).collect();

        VkSparseBufferMemoryBindInfoWrapper {
            vks_struct: vks::VkSparseBufferMemoryBindInfo {
                buffer: info.buffer.handle(),
                bindCount: binds.len() as u32,
                pBinds: binds_vk.as_ptr(),
            },
            buffer: info.buffer.clone(),
            binds: binds,
            binds_vk: binds_vk,
        }
    }
}

/// See [`VkSparseImageOpaqueMemoryBindInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseImageOpaqueMemoryBindInfo)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SparseImageOpaqueMemoryBindInfo {
    pub image: Image,
    pub binds: Vec<SparseMemoryBind>,
}

#[derive(Debug)]
struct VkSparseImageOpaqueMemoryBindInfoWrapper {
    pub vks_struct: vks::VkSparseImageOpaqueMemoryBindInfo,
    image: Image,
    binds: Vec<VkSparseMemoryBindWrapper>,
    binds_vk: Vec<vks::VkSparseMemoryBind>,
}

impl<'a> From<&'a SparseImageOpaqueMemoryBindInfo> for VkSparseImageOpaqueMemoryBindInfoWrapper {
    fn from(info: &'a SparseImageOpaqueMemoryBindInfo) -> Self {
        let binds: Vec<VkSparseMemoryBindWrapper> = info.binds.iter().map(From::from).collect();
        let binds_vk: Vec<_> = binds.iter().map(|b| b.vks_struct).collect();

        VkSparseImageOpaqueMemoryBindInfoWrapper {
            vks_struct: vks::VkSparseImageOpaqueMemoryBindInfo {
                image: info.image.handle(),
                bindCount: binds.len() as u32,
                pBinds: binds_vk.as_ptr(),
            },
            image: info.image.clone(),
            binds: binds,
            binds_vk: binds_vk,
        }
    }
}

/// See [`VkImageSubresource`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageSubresource)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ImageSubresource {
    pub aspect_mask: ImageAspectFlags,
    pub mip_level: u32,
    pub array_layer: u32,
}

impl<'a> From<&'a ImageSubresource> for vks::VkImageSubresource {
    fn from(subresource: &'a ImageSubresource) -> Self {
        vks::VkImageSubresource {
            aspectMask: subresource.aspect_mask.bits(),
            mipLevel: subresource.mip_level,
            arrayLayer: subresource.array_layer,
        }
    }
}

/// See [`VkOffset3D`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkOffset3D)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Offset3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl<'a> From<&'a Offset3D> for vks::VkOffset3D {
    fn from(offset: &'a Offset3D) -> Self {
        vks::VkOffset3D {
            x: offset.x,
            y: offset.y,
            z: offset.z,
        }
    }
}

impl Offset3D {
    /// Creates a new `Offset3D`.
    #[inline]
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Offset3D {
            x: x,
            y: y,
            z: z,
        }
    }

    /// Creates an `Offset3D` with all components set to 0.
    #[inline]
    pub fn zero() -> Self {
        Offset3D {
            x: 0,
            y: 0,
            z: 0,
        }
    }

    /// Creates an `Offset3D` from an `Offset2D` and the specified `z` component.
    #[inline]
    pub fn from_2d(offset: &Offset2D, z: i32) -> Self {
        Offset3D {
            x: offset.x,
            y: offset.y,
            z: z,
        }
    }
}

/// See [`VkSparseImageMemoryBind`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseImageMemoryBind)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SparseImageMemoryBind {
    pub subresource: ImageSubresource,
    pub offset: Offset3D,
    pub extent: Extent3D,
    pub memory: Option<DeviceMemory>,
    pub memory_offset: u64,
    pub flags: SparseMemoryBindFlags,
}

#[derive(Debug)]
struct VkSparseImageMemoryBindWrapper {
    pub vks_struct: vks::VkSparseImageMemoryBind,
    memory: Option<DeviceMemory>,
}

impl<'a> From<&'a SparseImageMemoryBind> for VkSparseImageMemoryBindWrapper {
    fn from(bind: &'a SparseImageMemoryBind) -> Self {
        let (vk_memory, memory) = match bind.memory {
            Some(ref memory) => (memory.handle(), Some(memory.clone())),
            None => (Default::default(), None),
        };

        VkSparseImageMemoryBindWrapper {
            vks_struct: vks::VkSparseImageMemoryBind {
                subresource: (&bind.subresource).into(),
                offset: (&bind.offset).into(),
                extent: (&bind.extent).into(),
                memory: vk_memory,
                memoryOffset: bind.memory_offset,
                flags: bind.flags.bits(),
            },
            memory: memory,
        }
    }
}

/// See [`VkSparseImageMemoryBindInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseImageMemoryBindInfo)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SparseImageMemoryBindInfo {
    pub image: Image,
    pub binds: Vec<SparseImageMemoryBind>,
}

#[derive(Debug)]
struct VkSparseImageMemoryBindInfoWrapper {
    pub vks_struct: vks::VkSparseImageMemoryBindInfo,
    image: Image,
    binds: Vec<VkSparseImageMemoryBindWrapper>,
    binds_vk: Vec<vks::VkSparseImageMemoryBind>,
}

impl<'a> From<&'a SparseImageMemoryBindInfo> for VkSparseImageMemoryBindInfoWrapper {
    fn from(info: &'a SparseImageMemoryBindInfo) -> Self {
        let binds: Vec<VkSparseImageMemoryBindWrapper> = info.binds.iter().map(From::from).collect();
        let binds_vk: Vec<_> = binds.iter().map(|b| b.vks_struct).collect();

        VkSparseImageMemoryBindInfoWrapper {
            vks_struct: vks::VkSparseImageMemoryBindInfo {
                image: info.image.handle(),
                bindCount: binds.len() as u32,
                pBinds: binds_vk.as_ptr(),
            },
            image: info.image.clone(),
            binds: binds,
            binds_vk: binds_vk,
        }
    }
}

gen_chain_struct! {
    name: BindSparseInfoChain [BindSparseInfoChainWrapper],
    query: BindSparseInfoChainQuery [BindSparseInfoChainQueryWrapper],
    vks: VkBindSparseInfo,
    input: true,
    output: false,
}

/// See [`VkBindSparseInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBindSparseInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct BindSparseInfo {
    pub wait_semaphores: Vec<Semaphore>,
    pub buffer_binds: Vec<SparseBufferMemoryBindInfo>,
    pub image_opaque_binds: Vec<SparseImageOpaqueMemoryBindInfo>,
    pub image_binds: Vec<SparseImageMemoryBindInfo>,
    pub signal_semaphores: Vec<Semaphore>,
    pub chain: Option<BindSparseInfoChain>,
}

#[derive(Debug)]
struct VkBindSparseInfoWrapper {
    pub vks_struct: vks::VkBindSparseInfo,
    wait_semaphores: Vec<Semaphore>,
    wait_vk_semaphores: Vec<vks::VkSemaphore>,
    buffer_binds: Vec<VkSparseBufferMemoryBindInfoWrapper>,
    vk_buffer_binds: Vec<vks::VkSparseBufferMemoryBindInfo>,
    image_opaque_binds: Vec<VkSparseImageOpaqueMemoryBindInfoWrapper>,
    vk_image_opaque_binds: Vec<vks::VkSparseImageOpaqueMemoryBindInfo>,
    image_binds: Vec<VkSparseImageMemoryBindInfoWrapper>,
    vk_image_binds: Vec<vks::VkSparseImageMemoryBindInfo>,
    signal_semaphores: Vec<Semaphore>,
    signal_vk_semaphores: Vec<vks::VkSemaphore>,
    chain: Option<BindSparseInfoChainWrapper>,
}

impl VkBindSparseInfoWrapper {
    pub fn new(info: &BindSparseInfo, with_chain: bool) -> Self {
        let wait_semaphores = info.wait_semaphores.clone();
        let (wait_vk_semaphores_ptr, wait_vk_semaphores) = if !wait_semaphores.is_empty() {
            let wait_vk_semaphores: Vec<_> = wait_semaphores.iter().map(Semaphore::handle).collect();
            (wait_vk_semaphores.as_ptr(), wait_vk_semaphores)
        }
        else {
            (ptr::null(), vec![])
        };

        let buffer_binds: Vec<VkSparseBufferMemoryBindInfoWrapper> = info.buffer_binds.iter().map(From::from).collect();
        let (vk_buffer_binds_ptr, vk_buffer_binds) = if !buffer_binds.is_empty() {
            let vk_buffer_binds: Vec<_> = buffer_binds.iter().map(|b| b.vks_struct).collect();
            (vk_buffer_binds.as_ptr(), vk_buffer_binds)
        }
        else {
            (ptr::null(), vec![])
        };

        let image_opaque_binds: Vec<VkSparseImageOpaqueMemoryBindInfoWrapper> = info.image_opaque_binds.iter().map(From::from).collect();
        let (vk_image_opaque_binds_ptr, vk_image_opaque_binds) = if !image_opaque_binds.is_empty() {
            let vk_image_opaque_binds: Vec<_> = image_opaque_binds.iter().map(|i| i.vks_struct).collect();
            (vk_image_opaque_binds.as_ptr(), vk_image_opaque_binds)
        }
        else {
            (ptr::null(), vec![])
        };

        let image_binds: Vec<VkSparseImageMemoryBindInfoWrapper> = info.image_binds.iter().map(From::from).collect();
        let (vk_image_binds_ptr, vk_image_binds) = if !image_binds.is_empty() {
            let vk_image_binds: Vec<_> = image_binds.iter().map(|i| i.vks_struct).collect();
            (vk_image_binds.as_ptr(), vk_image_binds)
        }
        else {
            (ptr::null(), vec![])
        };

        let signal_semaphores = info.signal_semaphores.clone();
        let (signal_vk_semaphores_ptr, signal_vk_semaphores) = if !signal_semaphores.is_empty() {
            let signal_vk_semaphores: Vec<_> = signal_semaphores.iter().map(Semaphore::handle).collect();
            (signal_vk_semaphores.as_ptr(), signal_vk_semaphores)
        }
        else {
            (ptr::null(), vec![])
        };

        let (pnext, chain) = BindSparseInfoChainWrapper::new_optional(&info.chain, with_chain);

        VkBindSparseInfoWrapper {
            vks_struct: vks::VkBindSparseInfo {
                sType: vks::VK_STRUCTURE_TYPE_BIND_SPARSE_INFO,
                pNext: pnext,
                waitSemaphoreCount: wait_semaphores.len() as u32,
                pWaitSemaphores: wait_vk_semaphores_ptr,
                bufferBindCount: buffer_binds.len() as u32,
                pBufferBinds: vk_buffer_binds_ptr,
                imageOpaqueBindCount: image_opaque_binds.len() as u32,
                pImageOpaqueBinds: vk_image_opaque_binds_ptr,
                imageBindCount: image_binds.len() as u32,
                pImageBinds: vk_image_binds_ptr,
                signalSemaphoreCount: signal_semaphores.len() as u32,
                pSignalSemaphores: signal_vk_semaphores_ptr,
            },
            wait_semaphores: wait_semaphores,
            wait_vk_semaphores: wait_vk_semaphores,
            buffer_binds: buffer_binds,
            vk_buffer_binds: vk_buffer_binds,
            image_opaque_binds: image_opaque_binds,
            vk_image_opaque_binds: vk_image_opaque_binds,
            image_binds: image_binds,
            vk_image_binds: vk_image_binds,
            signal_semaphores: signal_semaphores,
            signal_vk_semaphores: signal_vk_semaphores,
            chain: chain,
        }
    }
}

gen_chain_struct! {
    name: FenceCreateInfoChain [FenceCreateInfoChainWrapper],
    query: FenceCreateInfoChainQuery [FenceCreateInfoChainQueryWrapper],
    vks: VkFenceCreateInfo,
    input: true,
    output: false,
}

/// See [`VkFenceCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFenceCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct FenceCreateInfo {
    pub flags: FenceCreateFlags,
    pub chain: Option<FenceCreateInfoChain>,
}

#[derive(Debug)]
struct VkFenceCreateInfoWrapper {
    pub vks_struct: vks::VkFenceCreateInfo,
    chain: Option<FenceCreateInfoChainWrapper>,
}

impl VkFenceCreateInfoWrapper {
    pub fn new(create_info: &FenceCreateInfo, with_chain: bool) -> Self {
        let (pnext, chain) = FenceCreateInfoChainWrapper::new_optional(&create_info.chain, with_chain);

        VkFenceCreateInfoWrapper {
            vks_struct: vks::VkFenceCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
                pNext: pnext,
                flags: create_info.flags.bits(),
            },
            chain: chain,
        }
    }
}

gen_chain_struct! {
    name: SemaphoreCreateInfoChain [SemaphoreCreateInfoChainWrapper],
    query: SemaphoreCreateInfoChainQuery [SemaphoreCreateInfoChainQueryWrapper],
    vks: VkSemaphoreCreateInfo,
    input: true,
    output: false,
}

/// See [`VkSemaphoreCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSemaphoreCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct SemaphoreCreateInfo {
    pub flags: SemaphoreCreateFlags,
    pub chain: Option<SemaphoreCreateInfoChain>,
}

#[derive(Debug)]
struct VkSemaphoreCreateInfoWrapper {
    pub vks_struct: vks::VkSemaphoreCreateInfo,
    chain: Option<SemaphoreCreateInfoChainWrapper>,
}

impl VkSemaphoreCreateInfoWrapper {
    pub fn new(create_info: &SemaphoreCreateInfo, with_chain: bool) -> Self {
        let (pnext, chain) = SemaphoreCreateInfoChainWrapper::new_optional(&create_info.chain, with_chain);

        VkSemaphoreCreateInfoWrapper {
            vks_struct: vks::VkSemaphoreCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO,
                pNext: pnext,
                flags: create_info.flags.bits(),
            },
            chain: chain,
        }
    }
}

gen_chain_struct! {
    name: EventCreateInfoChain [EventCreateInfoChainWrapper],
    query: EventCreateInfoChainQuery [EventCreateInfoChainQueryWrapper],
    vks: VkEventCreateInfo,
    input: true,
    output: false,
}

/// See [`VkEventCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkEventCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct EventCreateInfo {
    pub flags: EventCreateFlags,
    pub chain: Option<EventCreateInfoChain>,
}

#[derive(Debug)]
struct VkEventCreateInfoWrapper {
    pub vks_struct: vks::VkEventCreateInfo,
    chain: Option<EventCreateInfoChainWrapper>,
}

impl VkEventCreateInfoWrapper {
    pub fn new(create_info: &EventCreateInfo, with_chain: bool) -> Self {
        let (pnext, chain) = EventCreateInfoChainWrapper::new_optional(&create_info.chain, with_chain);

        VkEventCreateInfoWrapper {
            vks_struct: vks::VkEventCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
                pNext: pnext,
                flags: create_info.flags.bits(),
            },
            chain: chain,
        }
    }
}

gen_chain_struct! {
    name: QueryPoolCreateInfoChain [QueryPoolCreateInfoChainWrapper],
    query: QueryPoolCreateInfoChainQuery [QueryPoolCreateInfoChainQueryWrapper],
    vks: VkQueryPoolCreateInfo,
    input: true,
    output: false,
}

/// See [`VkQueryPoolCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPoolCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct QueryPoolCreateInfo {
    pub flags: QueryPoolCreateFlags,
    pub query_type: QueryType,
    pub query_count: u32,
    pub pipeline_statistics: QueryPipelineStatisticFlags,
    pub chain: Option<QueryPoolCreateInfoChain>,
}

#[derive(Debug)]
struct VkQueryPoolCreateInfoWrapper {
    pub vks_struct: vks::VkQueryPoolCreateInfo,
    chain: Option<QueryPoolCreateInfoChainWrapper>,
}

impl VkQueryPoolCreateInfoWrapper {
    pub fn new(create_info: &QueryPoolCreateInfo, with_chain: bool) -> Self {
        let (pnext, chain) = QueryPoolCreateInfoChainWrapper::new_optional(&create_info.chain, with_chain);

        VkQueryPoolCreateInfoWrapper {
            vks_struct: vks::VkQueryPoolCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO,
                pNext: pnext,
                flags: create_info.flags.bits(),
                queryType: create_info.query_type.into(),
                queryCount: create_info.query_count,
                pipelineStatistics: create_info.pipeline_statistics.bits(),
            },
            chain: chain,
        }
    }
}

gen_chain_struct! {
    name: BufferCreateInfoChain [BufferCreateInfoChainWrapper],
    query: BufferCreateInfoChainQuery [BufferCreateInfoChainQueryWrapper],
    vks: VkBufferCreateInfo,
    input: true,
    output: false,
}

/// See [`VkBufferCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct BufferCreateInfo {
    pub flags: BufferCreateFlags,
    pub size: u64,
    pub usage: BufferUsageFlags,
    pub sharing_mode: SharingMode,
    pub queue_family_indices: Vec<u32>,
    pub chain: Option<BufferCreateInfoChain>,
}

#[derive(Debug)]
struct VkBufferCreateInfoWrapper {
    pub vks_struct: vks::VkBufferCreateInfo,
    queue_family_indices: Vec<u32>,
    chain: Option<BufferCreateInfoChainWrapper>,
}

impl VkBufferCreateInfoWrapper {
    pub fn new(create_info: &BufferCreateInfo, with_chain: bool) -> Self {
        let queue_family_indices = create_info.queue_family_indices.clone();
        let queue_family_indices_ptr = if !queue_family_indices.is_empty() {
            queue_family_indices.as_ptr()
        }
        else {
            ptr::null()
        };

        let (pnext, chain) = BufferCreateInfoChainWrapper::new_optional(&create_info.chain, with_chain);

        VkBufferCreateInfoWrapper {
            vks_struct: vks::VkBufferCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
                pNext: pnext,
                flags: create_info.flags.bits(),
                size: create_info.size,
                usage: create_info.usage.bits(),
                sharingMode: create_info.sharing_mode.into(),
                queueFamilyIndexCount: queue_family_indices.len() as u32,
                pQueueFamilyIndices: queue_family_indices_ptr,
            },
            queue_family_indices: queue_family_indices,
            chain: chain,
        }
    }
}

gen_chain_struct! {
    name: BufferViewCreateInfoChain [BufferViewCreateInfoChainWrapper],
    query: BufferViewCreateInfoChainQuery [BufferViewCreateInfoChainQueryWrapper],
    vks: VkBufferViewCreateInfo,
    input: true,
    output: false,
}

/// See [`VkBufferViewCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferViewCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct BufferViewCreateInfo {
    pub flags: BufferViewCreateFlags,
    pub buffer: Buffer,
    pub format: Format,
    pub offset: u64,
    pub range: OptionalDeviceSize,
    pub chain: Option<BufferViewCreateInfoChain>,
}

#[derive(Debug)]
struct VkBufferViewCreateInfoWrapper {
    pub vks_struct: vks::VkBufferViewCreateInfo,
    buffer: Buffer,
    chain: Option<BufferViewCreateInfoChainWrapper>,
}

impl VkBufferViewCreateInfoWrapper {
    pub fn new(create_info: &BufferViewCreateInfo, with_chain: bool) -> Self {
        let (pnext, chain) = BufferViewCreateInfoChainWrapper::new_optional(&create_info.chain, with_chain);

        VkBufferViewCreateInfoWrapper {
            vks_struct: vks::VkBufferViewCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO,
                pNext: pnext,
                flags: create_info.flags.bits(),
                buffer: create_info.buffer.handle(),
                format: create_info.format.into(),
                offset: create_info.offset,
                range: create_info.range.into(),
            },
            buffer: create_info.buffer.clone(),
            chain: chain,
        }
    }
}

gen_chain_struct! {
    name: ImageCreateInfoChain [ImageCreateInfoChainWrapper],
    query: ImageCreateInfoChainQuery [ImageCreateInfoChainQueryWrapper],
    vks: VkImageCreateInfo,
    input: true,
    output: false,
}

/// See [`VkImageCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct ImageCreateInfo {
    pub flags: ImageCreateFlags,
    pub image_type: ImageType,
    pub format: Format,
    pub extent: Extent3D,
    pub mip_levels: u32,
    pub array_layers: u32,
    pub samples: SampleCountFlagBits,
    pub tiling: ImageTiling,
    pub usage: ImageUsageFlags,
    pub sharing_mode: SharingMode,
    pub queue_family_indices: Vec<u32>,
    pub initial_layout: ImageLayout,
    pub chain: Option<ImageCreateInfoChain>,
}

#[derive(Debug)]
struct VkImageCreateInfoWrapper {
    pub vks_struct: vks::VkImageCreateInfo,
    queue_family_indices: Vec<u32>,
    chain: Option<ImageCreateInfoChainWrapper>,
}

impl VkImageCreateInfoWrapper {
    pub fn new(create_info: &ImageCreateInfo, with_chain: bool) -> Self {
        let queue_family_indices = create_info.queue_family_indices.clone();
        let queue_family_indices_ptr = if !queue_family_indices.is_empty() {
            queue_family_indices.as_ptr()
        }
        else {
            ptr::null()
        };

        let (pnext, chain) = ImageCreateInfoChainWrapper::new_optional(&create_info.chain, with_chain);

        VkImageCreateInfoWrapper {
            vks_struct: vks::VkImageCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO,
                pNext: pnext,
                flags: create_info.flags.bits(),
                imageType: create_info.image_type.into(),
                format: create_info.format.into(),
                extent: (&create_info.extent).into(),
                mipLevels: create_info.mip_levels,
                arrayLayers: create_info.array_layers,
                samples: create_info.samples.bits(),
                tiling: create_info.tiling.into(),
                usage: create_info.usage.bits(),
                sharingMode: create_info.sharing_mode.into(),
                queueFamilyIndexCount: queue_family_indices.len() as u32,
                pQueueFamilyIndices: queue_family_indices_ptr,
                initialLayout: create_info.initial_layout.into(),
            },
            queue_family_indices: queue_family_indices,
            chain: chain,
        }
    }
}

/// See [`VkSubresourceLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSubresourceLayout)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SubresourceLayout {
    pub offset: u64,
    pub size: u64,
    pub row_pitch: u64,
    pub array_pitch: u64,
    pub depth_pitch: u64,
}

impl<'a> From<&'a vks::VkSubresourceLayout> for SubresourceLayout {
    fn from(layout: &'a vks::VkSubresourceLayout) -> Self {
        SubresourceLayout {
            offset: layout.offset,
            size: layout.size,
            row_pitch: layout.rowPitch,
            array_pitch: layout.arrayPitch,
            depth_pitch: layout.depthPitch,
        }
    }
}

/// See [`VkComponentMapping`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkComponentMapping)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ComponentMapping {
    pub r: ComponentSwizzle,
    pub g: ComponentSwizzle,
    pub b: ComponentSwizzle,
    pub a: ComponentSwizzle,
}

impl<'a> From<&'a ComponentMapping> for vks::VkComponentMapping {
    fn from(mapping: &'a ComponentMapping) -> Self {
        vks::VkComponentMapping {
            r: mapping.r.into(),
            g: mapping.g.into(),
            b: mapping.b.into(),
            a: mapping.a.into(),
        }
    }
}

impl ComponentMapping {
    /// Creates a `ComponentMapping` with all components initialized to
    /// `ComponentSwizzle::Identity`.
    #[inline]
    pub fn identity() -> Self {
        ComponentMapping {
            r: ComponentSwizzle::Identity,
            g: ComponentSwizzle::Identity,
            b: ComponentSwizzle::Identity,
            a: ComponentSwizzle::Identity,
        }
    }
}

/// See [`VkImageSubresourceRange`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageSubresourceRange)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ImageSubresourceRange {
    pub aspect_mask: ImageAspectFlags,
    pub base_mip_level: u32,
    pub level_count: OptionalMipLevels,
    pub base_array_layer: u32,
    pub layer_count: OptionalArrayLayers,
}

impl<'a> From<&'a ImageSubresourceRange> for vks::VkImageSubresourceRange {
    fn from(range: &'a ImageSubresourceRange) -> Self {
        vks::VkImageSubresourceRange {
            aspectMask: range.aspect_mask.bits(),
            baseMipLevel: range.base_mip_level,
            levelCount: range.level_count.into(),
            baseArrayLayer: range.base_array_layer,
            layerCount: range.layer_count.into(),
        }
    }
}

gen_chain_struct! {
    name: ImageViewCreateInfoChain [ImageViewCreateInfoChainWrapper],
    query: ImageViewCreateInfoChainQuery [ImageViewCreateInfoChainQueryWrapper],
    vks: VkImageViewCreateInfo,
    input: true,
    output: false,
}

/// See [`VkImageViewCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageViewCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct ImageViewCreateInfo {
    pub flags: ImageViewCreateFlags,
    pub image: Image,
    pub view_type: ImageViewType,
    pub format: Format,
    pub components: ComponentMapping,
    pub subresource_range: ImageSubresourceRange,
    pub chain: Option<ImageViewCreateInfoChain>,
}

#[derive(Debug)]
struct VkImageViewCreateInfoWrapper {
    pub vks_struct: vks::VkImageViewCreateInfo,
    image: Image,
    chain: Option<ImageViewCreateInfoChainWrapper>,
}

impl VkImageViewCreateInfoWrapper {
    pub fn new(create_info: &ImageViewCreateInfo, with_chain: bool) -> Self {
        let (pnext, chain) = ImageViewCreateInfoChainWrapper::new_optional(&create_info.chain, with_chain);

        VkImageViewCreateInfoWrapper {
            vks_struct: vks::VkImageViewCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
                pNext: pnext,
                flags: create_info.flags.bits(),
                image: create_info.image.handle(),
                viewType: create_info.view_type.into(),
                format: create_info.format.into(),
                components: (&create_info.components).into(),
                subresourceRange: (&create_info.subresource_range).into(),
            },
            image: create_info.image.clone(),
            chain: chain,
        }
    }
}

gen_chain_struct! {
    name: ShaderModuleCreateInfoChain [ShaderModuleCreateInfoChainWrapper],
    query: ShaderModuleCreateInfoChainQuery [ShaderModuleCreateInfoChainQueryWrapper],
    vks: VkShaderModuleCreateInfo,
    input: true,
    output: false,
}

/// See [`VkShaderModuleCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderModuleCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct ShaderModuleCreateInfo {
    pub flags: ShaderModuleCreateFlags,
    pub code: Vec<u8>,
    pub chain: Option<ShaderModuleCreateInfoChain>,
}

#[derive(Debug)]
struct VkShaderModuleCreateInfoWrapper {
    pub vks_struct: vks::VkShaderModuleCreateInfo,
    code: Vec<u32>,
    chain: Option<ShaderModuleCreateInfoChainWrapper>,
}

impl VkShaderModuleCreateInfoWrapper {
    pub fn new(create_info: &ShaderModuleCreateInfo, with_chain: bool) -> Self {
        let size_u32 = if create_info.code.len() % mem::size_of::<u32>() == 0 {
            create_info.code.len() / mem::size_of::<u32>()
        }
        else {
            1 + create_info.code.len() / mem::size_of::<u32>()
        };

        let mut code = Vec::with_capacity(size_u32);
        unsafe {
            code.set_len(size_u32);
            ptr::copy_nonoverlapping(create_info.code.as_ptr(), code.as_mut_ptr() as *mut u8, create_info.code.len());
        }

        let (pnext, chain) = ShaderModuleCreateInfoChainWrapper::new_optional(&create_info.chain, with_chain);

        VkShaderModuleCreateInfoWrapper {
            vks_struct: vks::VkShaderModuleCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
                pNext: pnext,
                flags: create_info.flags.bits(),
                codeSize: create_info.code.len(),
                pCode: code.as_ptr(),
            },
            code: code,
            chain: chain,
        }
    }
}

gen_chain_struct! {
    name: PipelineCacheCreateInfoChain [PipelineCacheCreateInfoChainWrapper],
    query: PipelineCacheCreateInfoChainQuery [PipelineCacheCreateInfoChainQueryWrapper],
    vks: VkPipelineCacheCreateInfo,
    input: true,
    output: false,
}

/// See [`VkPipelineCacheCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCacheCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct PipelineCacheCreateInfo {
    pub flags: PipelineCacheCreateFlags,
    pub initial_data: Option<Vec<u8>>,
    pub chain: Option<PipelineCacheCreateInfoChain>,
}

#[derive(Debug)]
struct VkPipelineCacheCreateInfoWrapper {
    pub vks_struct: vks::VkPipelineCacheCreateInfo,
    initial_data: Option<Vec<u8>>,
    chain: Option<PipelineCacheCreateInfoChainWrapper>,
}

impl VkPipelineCacheCreateInfoWrapper {
    pub fn new(create_info: &PipelineCacheCreateInfo, with_chain: bool) -> Self {
        let (initial_data, initial_data_size, initial_data_ptr) = match create_info.initial_data {
            Some(ref initial_data) => {
                let initial_data = initial_data.clone();
                let initial_data_size = initial_data.len();
                let initial_data_ptr = initial_data.as_ptr() as *const c_void;
                (Some(initial_data), initial_data_size, initial_data_ptr)
            }

            None => (None, 0, ptr::null()),
        };

        let (pnext, chain) = PipelineCacheCreateInfoChainWrapper::new_optional(&create_info.chain, with_chain);

        VkPipelineCacheCreateInfoWrapper {
            vks_struct: vks::VkPipelineCacheCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO,
                pNext: pnext,
                flags: create_info.flags.bits(),
                initialDataSize: initial_data_size,
                pInitialData: initial_data_ptr,
            },
            initial_data: initial_data,
            chain: chain,
        }
    }
}

/// See [`VkSpecializationMapEntry`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSpecializationMapEntry)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SpecializationMapEntry {
    pub constant_id: u32,
    pub offset: u32,
    pub size: usize,
}

impl<'a> From<&'a SpecializationMapEntry> for vks::VkSpecializationMapEntry {
    fn from(entry: &'a SpecializationMapEntry) -> Self {
        vks::VkSpecializationMapEntry {
            constantID: entry.constant_id,
            offset: entry.offset,
            size: entry.size,
        }
    }
}

/// See [`VkSpecializationInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSpecializationInfo)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpecializationInfo {
    pub map_entries: Option<Vec<SpecializationMapEntry>>,
    pub data: Option<Vec<u8>>,
}

/// See [`VkSpecializationInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSpecializationInfo)
#[derive(Debug, Clone, Default)]
pub struct SpecializationInfoBuilder {
    map_entries: Vec<SpecializationMapEntry>,
    data: Vec<u8>,
}

impl SpecializationInfoBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn entry<T: Copy>(&mut self, constant_id: u32, value: T) -> &mut Self {
        let entry = SpecializationMapEntry {
            constant_id: constant_id,
            offset: self.data.len() as u32,
            size: mem::size_of::<T>(),
        };

        self.data.reserve(entry.size);
        unsafe {
            self.data.set_len(entry.offset as usize + entry.size);
            ptr::copy_nonoverlapping(&value as *const T as *const u8, self.data.as_mut_ptr().offset(entry.offset as isize), entry.size);
        }

        self.map_entries.push(entry);

        self
    }

    pub fn done(self) -> SpecializationInfo {
        if !self.map_entries.is_empty() {
            SpecializationInfo {
                map_entries: Some(self.map_entries),
                data: Some(self.data),
            }
        }
        else {
            SpecializationInfo {
                map_entries: None,
                data: None,
            }
        }
    }
}

#[derive(Debug)]
struct VkSpecializationInfoWrapper {
    pub vks_struct: vks::VkSpecializationInfo,
    map_entries: Option<Vec<vks::VkSpecializationMapEntry>>,
    data: Option<Vec<u8>>,
}

impl<'a> From<&'a SpecializationInfo> for VkSpecializationInfoWrapper {
    fn from(info: &'a SpecializationInfo) -> Self {
        let (map_entries_count, map_entries_ptr, map_entries) = match info.map_entries {
            Some(ref map_entries) => {
                let map_entries: Vec<_> = map_entries.iter().map(From::from).collect();
                (map_entries.len() as u32, map_entries.as_ptr(), Some(map_entries))
            }

            None => (0, ptr::null(), None),
        };

        let (data_size, data_ptr, data) = match info.data {
            Some(ref data) => {
                let data = data.clone();
                (data.len(), data.as_ptr() as *const c_void, Some(data))
            }

            None => (0, ptr::null(), None),
        };

        VkSpecializationInfoWrapper {
            vks_struct: vks::VkSpecializationInfo {
                mapEntryCount: map_entries_count,
                pMapEntries: map_entries_ptr,
                dataSize: data_size,
                pData: data_ptr,
            },
            map_entries: map_entries,
            data: data,
        }
    }
}

gen_chain_struct! {
    name: PipelineShaderStageCreateInfoChain [PipelineShaderStageCreateInfoChainWrapper],
    query: PipelineShaderStageCreateInfoChainQuery [PipelineShaderStageCreateInfoChainQueryWrapper],
    vks: VkPipelineShaderStageCreateInfo,
    input: true,
    output: false,
}

/// See [`VkPipelineShaderStageCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineShaderStageCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct PipelineShaderStageCreateInfo {
    pub flags: PipelineShaderStageCreateFlags,
    pub stage: ShaderStageFlagBits,
    pub module: ShaderModule,
    pub name: String,
    pub specialization_info: Option<SpecializationInfo>,
    pub chain: Option<PipelineShaderStageCreateInfoChain>,
}

#[derive(Debug)]
struct VkPipelineShaderStageCreateInfoWrapper {
    pub vks_struct: vks::VkPipelineShaderStageCreateInfo,
    module: ShaderModule,
    name: CString,
    specialization_info: Option<Box<VkSpecializationInfoWrapper>>,
    chain: Option<PipelineShaderStageCreateInfoChainWrapper>,
}

impl VkPipelineShaderStageCreateInfoWrapper {
    pub fn new(create_info: &PipelineShaderStageCreateInfo, with_chain: bool) -> Self {
        let name = CString::new(create_info.name.clone()).unwrap();

        let (specialization_info_ptr, specialization_info) = match create_info.specialization_info {
            Some(ref specialization_info) => {
                let specialization_info: Box<VkSpecializationInfoWrapper> = Box::new(specialization_info.into());
                (&specialization_info.vks_struct as *const _, Some(specialization_info))
            }

            None => (ptr::null(), None),
        };

        let (pnext, chain) = PipelineShaderStageCreateInfoChainWrapper::new_optional(&create_info.chain, with_chain);

        VkPipelineShaderStageCreateInfoWrapper {
            vks_struct: vks::VkPipelineShaderStageCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
                pNext: pnext,
                flags: create_info.flags.bits(),
                stage: create_info.stage.bits(),
                module: create_info.module.handle(),
                pName: name.as_ptr(),
                pSpecializationInfo: specialization_info_ptr,
            },
            module: create_info.module.clone(),
            name: name,
            specialization_info: specialization_info,
            chain: chain,
        }
    }
}

/// See [`VkVertexInputBindingDescription`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkVertexInputBindingDescription)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct VertexInputBindingDescription {
    pub binding: u32,
    pub stride: u32,
    pub input_rate: VertexInputRate,
}

impl<'a> From<&'a VertexInputBindingDescription> for vks::VkVertexInputBindingDescription {
    fn from(description: &'a VertexInputBindingDescription) -> Self {
        vks::VkVertexInputBindingDescription {
            binding: description.binding,
            stride: description.stride,
            inputRate: description.input_rate.into(),
        }
    }
}

/// See [`VkVertexInputAttributeDescription`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkVertexInputAttributeDescription)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct VertexInputAttributeDescription {
    pub location: u32,
    pub binding: u32,
    pub format: Format,
    pub offset: u32,
}

impl<'a> From<&'a VertexInputAttributeDescription> for vks::VkVertexInputAttributeDescription {
    fn from(description: &'a VertexInputAttributeDescription) -> Self {
        vks::VkVertexInputAttributeDescription {
            location: description.location,
            binding: description.binding,
            format: description.format.into(),
            offset: description.offset,
        }
    }
}

gen_chain_struct! {
    name: PipelineVertexInputStateCreateInfoChain [PipelineVertexInputStateCreateInfoChainWrapper],
    query: PipelineVertexInputStateCreateInfoChainQuery [PipelineVertexInputStateCreateInfoChainQueryWrapper],
    vks: VkPipelineVertexInputStateCreateInfo,
    input: true,
    output: false,
}

/// See [`VkPipelineVertexInputStateCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineVertexInputStateCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct PipelineVertexInputStateCreateInfo {
    pub flags: PipelineVertexInputStateCreateFlags,
    pub vertex_binding_descriptions: Option<Vec<VertexInputBindingDescription>>,
    pub vertex_attribute_descriptions: Option<Vec<VertexInputAttributeDescription>>,
    pub chain: Option<PipelineVertexInputStateCreateInfoChain>,
}

#[derive(Debug)]
struct VkPipelineVertexInputStateCreateInfoWrapper {
    pub vks_struct: vks::VkPipelineVertexInputStateCreateInfo,
    vertex_binding_descriptions: Option<Vec<vks::VkVertexInputBindingDescription>>,
    vertex_attribute_descriptions: Option<Vec<vks::VkVertexInputAttributeDescription>>,
    chain: Option<PipelineVertexInputStateCreateInfoChainWrapper>,
}

impl VkPipelineVertexInputStateCreateInfoWrapper {
    pub fn new(create_info: &PipelineVertexInputStateCreateInfo, with_chain: bool) -> Self {
        let (vertex_binding_descriptions_count, vertex_binding_descriptions_ptr, vertex_binding_descriptions) = match create_info.vertex_binding_descriptions {
            Some(ref vertex_binding_descriptions) => {
                let vertex_binding_descriptions: Vec<_> = vertex_binding_descriptions.iter().map(From::from).collect();
                (vertex_binding_descriptions.len() as u32, vertex_binding_descriptions.as_ptr(), Some(vertex_binding_descriptions))
            }

            None => (0, ptr::null(), None),
        };

        let (vertex_attribute_descriptions_count, vertex_attribute_descriptions_ptr, vertex_attribute_descriptions) = match create_info.vertex_attribute_descriptions {
            Some(ref vertex_attribute_descriptions) => {
                let vertex_attribute_descriptions: Vec<_> = vertex_attribute_descriptions.iter().map(From::from).collect();
                (vertex_attribute_descriptions.len() as u32, vertex_attribute_descriptions.as_ptr(), Some(vertex_attribute_descriptions))
            }

            None => (0, ptr::null(), None),
        };

        let (pnext, chain) = PipelineVertexInputStateCreateInfoChainWrapper::new_optional(&create_info.chain, with_chain);

        VkPipelineVertexInputStateCreateInfoWrapper {
            vks_struct: vks::VkPipelineVertexInputStateCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
                pNext: pnext,
                flags: create_info.flags.bits(),
                vertexBindingDescriptionCount: vertex_binding_descriptions_count,
                pVertexBindingDescriptions: vertex_binding_descriptions_ptr,
                vertexAttributeDescriptionCount: vertex_attribute_descriptions_count,
                pVertexAttributeDescriptions: vertex_attribute_descriptions_ptr,
            },
            vertex_binding_descriptions: vertex_binding_descriptions,
            vertex_attribute_descriptions: vertex_attribute_descriptions,
            chain: chain,
        }
    }
}

gen_chain_struct! {
    name: PipelineInputAssemblyStateCreateInfoChain [PipelineInputAssemblyStateCreateInfoChainWrapper],
    query: PipelineInputAssemblyStateCreateInfoChainQuery [PipelineInputAssemblyStateCreateInfoChainQueryWrapper],
    vks: VkPipelineInputAssemblyStateCreateInfo,
    input: true,
    output: false,
}

/// See [`VkPipelineInputAssemblyStateCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineInputAssemblyStateCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct PipelineInputAssemblyStateCreateInfo {
    pub flags: PipelineInputAssemblyStateCreateFlags,
    pub topology: PrimitiveTopology,
    pub primitive_restart_enable: bool,
    pub chain: Option<PipelineInputAssemblyStateCreateInfoChain>,
}

#[derive(Debug)]
struct VkPipelineInputAssemblyStateCreateInfoWrapper {
    pub vks_struct: vks::VkPipelineInputAssemblyStateCreateInfo,
    chain: Option<PipelineInputAssemblyStateCreateInfoChainWrapper>,
}

impl VkPipelineInputAssemblyStateCreateInfoWrapper {
    pub fn new(create_info: &PipelineInputAssemblyStateCreateInfo, with_chain: bool) -> Self {
        let (pnext, chain) = PipelineInputAssemblyStateCreateInfoChainWrapper::new_optional(&create_info.chain, with_chain);

        VkPipelineInputAssemblyStateCreateInfoWrapper {
            vks_struct: vks::VkPipelineInputAssemblyStateCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
                pNext: pnext,
                flags: create_info.flags.bits(),
                topology: create_info.topology.into(),
                primitiveRestartEnable: utils::to_vk_bool(create_info.primitive_restart_enable),
            },
            chain: chain,
        }
    }
}

gen_chain_struct! {
    name: PipelineTessellationStateCreateInfoChain [PipelineTessellationStateCreateInfoChainWrapper],
    query: PipelineTessellationStateCreateInfoChainQuery [PipelineTessellationStateCreateInfoChainQueryWrapper],
    vks: VkPipelineTessellationStateCreateInfo,
    input: true,
    output: false,
}

/// See [`VkPipelineTessellationStateCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineTessellationStateCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct PipelineTessellationStateCreateInfo {
    pub flags: PipelineTessellationStateCreateFlags,
    pub patch_control_points: u32,
    pub chain: Option<PipelineTessellationStateCreateInfoChain>,
}

#[derive(Debug)]
struct VkPipelineTessellationStateCreateInfoWrapper {
    pub vks_struct: vks::VkPipelineTessellationStateCreateInfo,
    chain: Option<PipelineTessellationStateCreateInfoChainWrapper>,
}

impl VkPipelineTessellationStateCreateInfoWrapper {
    pub fn new(create_info: &PipelineTessellationStateCreateInfo, with_chain: bool) -> Self {
        let (pnext, chain) = PipelineTessellationStateCreateInfoChainWrapper::new_optional(&create_info.chain, with_chain);

        VkPipelineTessellationStateCreateInfoWrapper {
            vks_struct: vks::VkPipelineTessellationStateCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO,
                pNext: pnext,
                flags: create_info.flags.bits(),
                patchControlPoints: create_info.patch_control_points,
            },
            chain: chain,
        }
    }
}

/// See [`VkViewport`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkViewport)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub min_depth: f32,
    pub max_depth: f32,
}

impl<'a> From<&'a Viewport> for vks::VkViewport {
    fn from(viewport: &'a Viewport) -> Self {
        vks::VkViewport {
            x: viewport.x,
            y: viewport.y,
            width: viewport.width,
            height: viewport.height,
            minDepth: viewport.min_depth,
            maxDepth: viewport.max_depth,
        }
    }
}

/// See [`VkOffset2D`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkOffset2D)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Offset2D {
    pub x: i32,
    pub y: i32,
}

impl<'a> From<&'a vks::VkOffset2D> for Offset2D {
    fn from(offset: &'a vks::VkOffset2D) -> Self {
        Offset2D {
            x: offset.x,
            y: offset.y,
        }
    }
}

impl<'a> From<&'a Offset2D> for vks::VkOffset2D {
    fn from(offset: &'a Offset2D) -> Self {
        vks::VkOffset2D {
            x: offset.x,
            y: offset.y,
        }
    }
}

impl Offset2D {
    /// Creates a new `Offset2D`.
    #[inline]
    pub fn new(x: i32, y: i32) -> Self {
        Offset2D {
            x: x,
            y: y,
        }
    }

    /// Creates an `Offset2D` with all components set to 0.
    #[inline]
    pub fn zero() -> Self {
        Offset2D {
            x: 0,
            y: 0,
        }
    }

    /// Creates an `Offset2D` from an `Offset3D` by discarding the `z` component.
    #[inline]
    pub fn from_3d(offset: &Offset3D) -> Self {
        Offset2D {
            x: offset.x,
            y: offset.y,
        }
    }
}

/// See [`VkExtent2D`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExtent2D)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Extent2D {
    pub width: u32,
    pub height: u32,
}

impl<'a> From<&'a vks::VkExtent2D> for Extent2D {
    fn from(extent: &'a vks::VkExtent2D) -> Self {
        Extent2D {
            width: extent.width,
            height: extent.height,
        }
    }
}

impl<'a> From<&'a Extent2D> for vks::VkExtent2D {
    fn from(extent: &'a Extent2D) -> Self {
        vks::VkExtent2D {
            width: extent.width,
            height: extent.height,
        }
    }
}

impl Extent2D {
    /// Creates a new `Extent2D`.
    #[inline]
    pub fn new(width: u32, height: u32) -> Self {
        Extent2D {
            width: width,
            height: height,
        }
    }

    /// Creates an `Extent2D` with all components set to 0.
    #[inline]
    pub fn zero() -> Self {
        Extent2D {
            width: 0,
            height: 0,
        }
    }

    /// Creates an `Extent2D` from an `Extent3D` by discarding the `depth` component.
    #[inline]
    pub fn from_3d(extent: &Extent3D) -> Self {
        Extent2D {
            width: extent.width,
            height: extent.height,
        }
    }
}

/// See [`VkRect2D`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkRect2D)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Rect2D {
    pub offset: Offset2D,
    pub extent: Extent2D,
}

impl<'a> From<&'a Rect2D> for vks::VkRect2D {
    fn from(rect: &'a Rect2D) -> Self {
        vks::VkRect2D {
            offset: (&rect.offset).into(),
            extent: (&rect.extent).into(),
        }
    }
}

impl Rect2D {
    /// Creates a new `Rect2D`.
    #[inline]
    pub fn new(offset: Offset2D, extent: Extent2D) -> Self {
        Rect2D {
            offset: offset,
            extent: extent,
        }
    }
}

gen_chain_struct! {
    name: PipelineViewportStateCreateInfoChain [PipelineViewportStateCreateInfoChainWrapper],
    query: PipelineViewportStateCreateInfoChainQuery [PipelineViewportStateCreateInfoChainQueryWrapper],
    vks: VkPipelineViewportStateCreateInfo,
    input: true,
    output: false,
}

/// See [`VkPipelineViewportStateCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineViewportStateCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct PipelineViewportStateCreateInfo {
    pub flags: PipelineViewportStateCreateFlags,
    pub viewports: Vec<Viewport>,
    pub scissors: Vec<Rect2D>,
    pub chain: Option<PipelineViewportStateCreateInfoChain>,
}

#[derive(Debug)]
struct VkPipelineViewportStateCreateInfoWrapper {
    pub vks_struct: vks::VkPipelineViewportStateCreateInfo,
    viewports: Vec<vks::VkViewport>,
    scissors: Vec<vks::VkRect2D>,
    chain: Option<PipelineViewportStateCreateInfoChainWrapper>,
}

impl VkPipelineViewportStateCreateInfoWrapper {
    pub fn new(create_info: &PipelineViewportStateCreateInfo, with_chain: bool) -> Self {
        let viewports: Vec<_> = create_info.viewports.iter().map(From::from).collect();
        let scissors: Vec<_> = create_info.scissors.iter().map(From::from).collect();
        let (pnext, chain) = PipelineViewportStateCreateInfoChainWrapper::new_optional(&create_info.chain, with_chain);

        VkPipelineViewportStateCreateInfoWrapper {
            vks_struct: vks::VkPipelineViewportStateCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO,
                pNext: pnext,
                flags: create_info.flags.bits(),
                viewportCount: viewports.len() as u32,
                pViewports: viewports.as_ptr(),
                scissorCount: scissors.len() as u32,
                pScissors: scissors.as_ptr(),
            },
            viewports: viewports,
            scissors: scissors,
            chain: chain,
        }
    }
}

gen_chain_struct! {
    name: PipelineRasterizationStateCreateInfoChain [PipelineRasterizationStateCreateInfoChainWrapper],
    query: PipelineRasterizationStateCreateInfoChainQuery [PipelineRasterizationStateCreateInfoChainQueryWrapper],
    vks: VkPipelineRasterizationStateCreateInfo,
    input: true,
    output: false,

    pipeline_rasterization_state_rasterization_order_amd: PipelineRasterizationStateRasterizationOrderAmd {
        mod: amd_rasterization_order,
        fn_add: add_pipeline_rasterization_state_rasterization_order_amd,
        fn_has: has_pipeline_rasterization_state_rasterization_order_amd,
        fn_get: get_pipeline_rasterization_state_rasterization_order_amd,
        wrapper: VkPipelineRasterizationStateRasterizationOrderAMDWrapper,
        vks: VkPipelineRasterizationStateRasterizationOrderAMD,
        stype: vks::VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD,
    }
}

/// See [`VkPipelineRasterizationStateCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineRasterizationStateCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct PipelineRasterizationStateCreateInfo {
    pub flags: PipelineRasterizationStateCreateFlags,
    pub depth_clamp_enable: bool,
    pub rasterizer_discard_enable: bool,
    pub polygon_mode: PolygonMode,
    pub cull_mode: CullModeFlags,
    pub front_face: FrontFace,
    pub depth_bias_enable: bool,
    pub depth_bias_constant_factor: f32,
    pub depth_bias_clamp: f32,
    pub depth_bias_slope_factor: f32,
    pub line_width: f32,
    pub chain: Option<PipelineRasterizationStateCreateInfoChain>,
}

#[derive(Debug)]
struct VkPipelineRasterizationStateCreateInfoWrapper {
    pub vks_struct: vks::VkPipelineRasterizationStateCreateInfo,
    chain: Option<PipelineRasterizationStateCreateInfoChainWrapper>,
}

impl VkPipelineRasterizationStateCreateInfoWrapper {
    pub fn new(create_info: &PipelineRasterizationStateCreateInfo, with_chain: bool) -> Self {
        let (pnext, chain) = PipelineRasterizationStateCreateInfoChainWrapper::new_optional(&create_info.chain, with_chain);

        VkPipelineRasterizationStateCreateInfoWrapper {
            vks_struct: vks::VkPipelineRasterizationStateCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
                pNext: pnext,
                flags: create_info.flags.bits(),
                depthClampEnable: utils::to_vk_bool(create_info.depth_clamp_enable),
                rasterizerDiscardEnable: utils::to_vk_bool(create_info.rasterizer_discard_enable),
                polygonMode: create_info.polygon_mode.into(),
                cullMode: create_info.cull_mode.bits(),
                frontFace: create_info.front_face.into(),
                depthBiasEnable: utils::to_vk_bool(create_info.depth_bias_enable),
                depthBiasConstantFactor: create_info.depth_bias_constant_factor,
                depthBiasClamp: create_info.depth_bias_clamp,
                depthBiasSlopeFactor: create_info.depth_bias_slope_factor,
                lineWidth: create_info.line_width,
            },
            chain: chain,
        }
    }
}

gen_chain_struct! {
    name: PipelineMultisampleStateCreateInfoChain [PipelineMultisampleStateCreateInfoChainWrapper],
    query: PipelineMultisampleStateCreateInfoChainQuery [PipelineMultisampleStateCreateInfoChainQueryWrapper],
    vks: VkPipelineMultisampleStateCreateInfo,
    input: true,
    output: false,
}

/// See [`VkPipelineMultisampleStateCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineMultisampleStateCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct PipelineMultisampleStateCreateInfo {
    pub flags: PipelineMultisampleStateCreateFlags,
    pub rasterization_samples: SampleCountFlagBits,
    pub sample_shading_enable: bool,
    pub min_sample_shading: f32,
    pub sample_mask: Option<Vec<u32>>,
    pub alpha_to_coverage_enable: bool,
    pub alpha_to_one_enable: bool,
    pub chain: Option<PipelineMultisampleStateCreateInfoChain>,
}

#[derive(Debug)]
struct VkPipelineMultisampleStateCreateInfoWrapper {
    pub vks_struct: vks::VkPipelineMultisampleStateCreateInfo,
    sample_mask: Option<Vec<u32>>,
    chain: Option<PipelineMultisampleStateCreateInfoChainWrapper>,
}

impl VkPipelineMultisampleStateCreateInfoWrapper {
    pub fn new(create_info: &PipelineMultisampleStateCreateInfo, with_chain: bool) -> Self {
        let (sample_mask_ptr, sample_mask) = match create_info.sample_mask {
            Some(ref sample_mask) => {
                let sample_mask = sample_mask.clone();
                (sample_mask.as_ptr(), Some(sample_mask))
            }

            None => (ptr::null(), None),
        };

        let (pnext, chain) = PipelineMultisampleStateCreateInfoChainWrapper::new_optional(&create_info.chain, with_chain);

        VkPipelineMultisampleStateCreateInfoWrapper {
            vks_struct: vks::VkPipelineMultisampleStateCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
                pNext: pnext,
                flags: create_info.flags.bits(),
                rasterizationSamples: create_info.rasterization_samples.bits(),
                sampleShadingEnable: utils::to_vk_bool(create_info.sample_shading_enable),
                minSampleShading: create_info.min_sample_shading,
                pSampleMask: sample_mask_ptr,
                alphaToCoverageEnable: utils::to_vk_bool(create_info.alpha_to_coverage_enable),
                alphaToOneEnable: utils::to_vk_bool(create_info.alpha_to_one_enable),
            },
            sample_mask: sample_mask,
            chain: chain,
        }
    }
}

/// See [`VkStencilOpState`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStencilOpState)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct StencilOpState {
    pub fail_op: StencilOp,
    pub pass_op: StencilOp,
    pub depth_fail_op: StencilOp,
    pub compare_op: CompareOp,
    pub compare_mask: u32,
    pub write_mask: u32,
    pub reference: u32,
}

impl<'a> From<&'a StencilOpState> for vks::VkStencilOpState {
    fn from(state: &'a StencilOpState) -> Self {
        vks::VkStencilOpState {
            failOp: state.fail_op.into(),
            passOp: state.pass_op.into(),
            depthFailOp: state.depth_fail_op.into(),
            compareOp: state.compare_op.into(),
            compareMask: state.compare_mask,
            writeMask: state.write_mask,
            reference: state.reference,
        }
    }
}

gen_chain_struct! {
    name: PipelineDepthStencilStateCreateInfoChain [PipelineDepthStencilStateCreateInfoChainWrapper],
    query: PipelineDepthStencilStateCreateInfoChainQuery [PipelineDepthStencilStateCreateInfoChainQueryWrapper],
    vks: VkPipelineDepthStencilStateCreateInfo,
    input: true,
    output: false,
}

/// See [`VkPipelineDepthStencilStateCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineDepthStencilStateCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct PipelineDepthStencilStateCreateInfo {
    pub flags: PipelineDepthStencilStateCreateFlags,
    pub depth_test_enable: bool,
    pub depth_write_enable: bool,
    pub depth_compare_op: CompareOp,
    pub depth_bounds_test_enable: bool,
    pub stencil_test_enable: bool,
    pub front: StencilOpState,
    pub back: StencilOpState,
    pub min_depth_bounds: f32,
    pub max_depth_bounds: f32,
    pub chain: Option<PipelineDepthStencilStateCreateInfoChain>,
}

#[derive(Debug)]
struct VkPipelineDepthStencilStateCreateInfoWrapper {
    pub vks_struct: vks::VkPipelineDepthStencilStateCreateInfo,
    chain: Option<PipelineDepthStencilStateCreateInfoChainWrapper>,
}

impl VkPipelineDepthStencilStateCreateInfoWrapper {
    pub fn new(create_info: &PipelineDepthStencilStateCreateInfo, with_chain: bool) -> Self {
        let (pnext, chain) = PipelineDepthStencilStateCreateInfoChainWrapper::new_optional(&create_info.chain, with_chain);

        VkPipelineDepthStencilStateCreateInfoWrapper {
            vks_struct: vks::VkPipelineDepthStencilStateCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
                pNext: pnext,
                flags: create_info.flags.bits(),
                depthTestEnable: utils::to_vk_bool(create_info.depth_test_enable),
                depthWriteEnable: utils::to_vk_bool(create_info.depth_write_enable),
                depthCompareOp: create_info.depth_compare_op.into(),
                depthBoundsTestEnable: utils::to_vk_bool(create_info.depth_bounds_test_enable),
                stencilTestEnable: utils::to_vk_bool(create_info.stencil_test_enable),
                front: (&create_info.front).into(),
                back: (&create_info.back).into(),
                minDepthBounds: create_info.min_depth_bounds,
                maxDepthBounds: create_info.max_depth_bounds,
            },
            chain: chain,
        }
    }
}

/// See [`VkPipelineColorBlendAttachmentState`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineColorBlendAttachmentState)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct PipelineColorBlendAttachmentState {
    pub blend_enable: bool,
    pub src_color_blend_factor: BlendFactor,
    pub dst_color_blend_factor: BlendFactor,
    pub color_blend_op: BlendOp,
    pub src_alpha_blend_factor: BlendFactor,
    pub dst_alpha_blend_factor: BlendFactor,
    pub alpha_blend_op: BlendOp,
    pub color_write_mask: ColorComponentFlags,
}

impl<'a> From<&'a PipelineColorBlendAttachmentState> for vks::VkPipelineColorBlendAttachmentState {
    fn from(state: &'a PipelineColorBlendAttachmentState) -> Self {
        vks::VkPipelineColorBlendAttachmentState {
            blendEnable: utils::to_vk_bool(state.blend_enable),
            srcColorBlendFactor: state.src_color_blend_factor.into(),
            dstColorBlendFactor: state.dst_color_blend_factor.into(),
            colorBlendOp: state.color_blend_op.into(),
            srcAlphaBlendFactor: state.src_alpha_blend_factor.into(),
            dstAlphaBlendFactor: state.dst_alpha_blend_factor.into(),
            alphaBlendOp: state.alpha_blend_op.into(),
            colorWriteMask: state.color_write_mask.bits(),
        }
    }
}

gen_chain_struct! {
    name: PipelineColorBlendStateCreateInfoChain [PipelineColorBlendStateCreateInfoChainWrapper],
    query: PipelineColorBlendStateCreateInfoChainQuery [PipelineColorBlendStateCreateInfoChainQueryWrapper],
    vks: VkPipelineColorBlendStateCreateInfo,
    input: true,
    output: false,
}

/// See [`VkPipelineColorBlendStateCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineColorBlendStateCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct PipelineColorBlendStateCreateInfo {
    pub flags: PipelineColorBlendStateCreateFlags,
    pub logic_op_enable: bool,
    pub logic_op: LogicOp,
    pub attachments: Option<Vec<PipelineColorBlendAttachmentState>>,
    pub blend_constants: [f32; 4],
    pub chain: Option<PipelineColorBlendStateCreateInfoChain>,
}

#[derive(Debug)]
struct VkPipelineColorBlendStateCreateInfoWrapper {
    pub vks_struct: vks::VkPipelineColorBlendStateCreateInfo,
    attachments: Option<Vec<vks::VkPipelineColorBlendAttachmentState>>,
    chain: Option<PipelineColorBlendStateCreateInfoChainWrapper>,
}

impl VkPipelineColorBlendStateCreateInfoWrapper {
    pub fn new(create_info: &PipelineColorBlendStateCreateInfo, with_chain: bool) -> Self {
        let (attachments_count, attachments_ptr, attachments) = match create_info.attachments {
            Some(ref attachments) => {
                let attachments: Vec<_> = attachments.iter().map(From::from).collect();
                (attachments.len() as u32, attachments.as_ptr(), Some(attachments))
            }

            None => (0, ptr::null(), None),
        };

        let (pnext, chain) = PipelineColorBlendStateCreateInfoChainWrapper::new_optional(&create_info.chain, with_chain);

        VkPipelineColorBlendStateCreateInfoWrapper {
            vks_struct: vks::VkPipelineColorBlendStateCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
                pNext: pnext,
                flags: create_info.flags.bits(),
                logicOpEnable: utils::to_vk_bool(create_info.logic_op_enable),
                logicOp: create_info.logic_op.into(),
                attachmentCount: attachments_count,
                pAttachments: attachments_ptr,
                blendConstants: create_info.blend_constants,
            },
            attachments: attachments,
            chain: chain,
        }
    }
}

gen_chain_struct! {
    name: PipelineDynamicStateCreateInfoChain [PipelineDynamicStateCreateInfoChainWrapper],
    query: PipelineDynamicStateCreateInfoChainQuery [PipelineDynamicStateCreateInfoChainQueryWrapper],
    vks: VkPipelineDynamicStateCreateInfo,
    input: true,
    output: false,
}

/// See [`VkPipelineDynamicStateCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineDynamicStateCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct PipelineDynamicStateCreateInfo {
    pub flags: PipelineDynamicStateCreateFlags,
    pub dynamic_states: Vec<DynamicState>,
    pub chain: Option<PipelineDynamicStateCreateInfoChain>,
}

#[derive(Debug)]
struct VkPipelineDynamicStateCreateInfoWrapper {
    pub vks_struct: vks::VkPipelineDynamicStateCreateInfo,
    dynamic_states: Vec<vks::VkDynamicState>,
    chain: Option<PipelineDynamicStateCreateInfoChainWrapper>,
}

impl VkPipelineDynamicStateCreateInfoWrapper {
    pub fn new(create_info: &PipelineDynamicStateCreateInfo, with_chain: bool) -> Self {
        let dynamic_states: Vec<_> = create_info.dynamic_states
            .iter()
            .cloned()
            .map(From::from)
            .collect();

        let (pnext, chain) = PipelineDynamicStateCreateInfoChainWrapper::new_optional(&create_info.chain, with_chain);

        VkPipelineDynamicStateCreateInfoWrapper {
            vks_struct: vks::VkPipelineDynamicStateCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO,
                pNext: pnext,
                flags: create_info.flags.bits(),
                dynamicStateCount: dynamic_states.len() as u32,
                pDynamicStates: dynamic_states.as_ptr(),
            },
            dynamic_states: dynamic_states,
            chain: chain,
        }
    }
}

gen_chain_struct! {
    name: GraphicsPipelineCreateInfoChain [GraphicsPipelineCreateInfoChainWrapper],
    query: GraphicsPipelineCreateInfoChainQuery [GraphicsPipelineCreateInfoChainQueryWrapper],
    vks: VkGraphicsPipelineCreateInfo,
    input: true,
    output: false,
}

/// See [`VkGraphicsPipelineCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkGraphicsPipelineCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct GraphicsPipelineCreateInfo {
    pub flags: PipelineCreateFlags,
    pub stages: Vec<PipelineShaderStageCreateInfo>,
    pub vertex_input_state: PipelineVertexInputStateCreateInfo,
    pub input_assembly_state: PipelineInputAssemblyStateCreateInfo,
    pub tessellation_state: Option<PipelineTessellationStateCreateInfo>,
    pub viewport_state: Option<PipelineViewportStateCreateInfo>,
    pub rasterization_state: PipelineRasterizationStateCreateInfo,
    pub multisample_state: Option<PipelineMultisampleStateCreateInfo>,
    pub depth_stencil_state: Option<PipelineDepthStencilStateCreateInfo>,
    pub color_blend_state: Option<PipelineColorBlendStateCreateInfo>,
    pub dynamic_state: Option<PipelineDynamicStateCreateInfo>,
    pub layout: PipelineLayout,
    pub render_pass: RenderPass,
    pub subpass: u32,
    pub base_pipeline: Option<Pipeline>,
    pub base_pipeline_index: Option<u32>,
    pub chain: Option<GraphicsPipelineCreateInfoChain>,
}

#[derive(Debug)]
struct VkGraphicsPipelineCreateInfoWrapper {
    pub vks_struct: vks::VkGraphicsPipelineCreateInfo,
    stages: Vec<VkPipelineShaderStageCreateInfoWrapper>,
    vk_stages: Vec<vks::VkPipelineShaderStageCreateInfo>,
    vertex_input_state: Box<VkPipelineVertexInputStateCreateInfoWrapper>,
    input_assembly_state: Box<VkPipelineInputAssemblyStateCreateInfoWrapper>,
    tessellation_state: Option<Box<VkPipelineTessellationStateCreateInfoWrapper>>,
    viewport_state: Option<Box<VkPipelineViewportStateCreateInfoWrapper>>,
    rasterization_state: Box<VkPipelineRasterizationStateCreateInfoWrapper>,
    multisample_state: Option<Box<VkPipelineMultisampleStateCreateInfoWrapper>>,
    depth_stencil_state: Option<Box<VkPipelineDepthStencilStateCreateInfoWrapper>>,
    color_blend_state: Option<Box<VkPipelineColorBlendStateCreateInfoWrapper>>,
    dynamic_state: Option<Box<VkPipelineDynamicStateCreateInfoWrapper>>,
    layout: PipelineLayout,
    render_pass: RenderPass,
    base_pipeline: Option<Pipeline>,
    chain: Option<GraphicsPipelineCreateInfoChainWrapper>,
}

impl VkGraphicsPipelineCreateInfoWrapper {
    pub fn new(create_info: &GraphicsPipelineCreateInfo, with_chain: bool) -> Self {
        let stages: Vec<_> = create_info.stages.iter().map(|c| VkPipelineShaderStageCreateInfoWrapper::new(c, true)).collect();
        let vk_stages: Vec<_> = stages.iter().map(|s| s.vks_struct).collect();
        let vertex_input_state: Box<_> = Box::new(VkPipelineVertexInputStateCreateInfoWrapper::new(&create_info.vertex_input_state, true));
        let input_assembly_state: Box<_> = Box::new(VkPipelineInputAssemblyStateCreateInfoWrapper::new(&create_info.input_assembly_state, true));

        let (tessellation_state_ptr, tessellation_state) = match create_info.tessellation_state {
            Some(ref tessellation_state) => {
                let tessellation_state: Box<_> = Box::new(VkPipelineTessellationStateCreateInfoWrapper::new(tessellation_state, true));
                (&tessellation_state.vks_struct as *const _, Some(tessellation_state))
            }

            None => (ptr::null(), None),
        };

        let (viewport_state_ptr, viewport_state) = match create_info.viewport_state {
            Some(ref viewport_state) => {
                let viewport_state: Box<_> = Box::new(VkPipelineViewportStateCreateInfoWrapper::new(viewport_state, true));
                (&viewport_state.vks_struct as *const _, Some(viewport_state))
            }

            None => (ptr::null(), None),
        };

        let rasterization_state: Box<_> = Box::new(VkPipelineRasterizationStateCreateInfoWrapper::new(&create_info.rasterization_state, true));

        let (multisample_state_ptr, multisample_state) = match create_info.multisample_state {
            Some(ref multisample_state) => {
                let multisample_state: Box<_> = Box::new(VkPipelineMultisampleStateCreateInfoWrapper::new(multisample_state, true));
                (&multisample_state.vks_struct as *const _, Some(multisample_state))
            }

            None => (ptr::null(), None),
        };

        let (depth_stencil_state_ptr, depth_stencil_state) = match create_info.depth_stencil_state {
            Some(ref depth_stencil_state) => {
                let depth_stencil_state: Box<_> = Box::new(VkPipelineDepthStencilStateCreateInfoWrapper::new(depth_stencil_state, true));
                (&depth_stencil_state.vks_struct as *const _, Some(depth_stencil_state))
            }

            None => (ptr::null(), None),
        };

        let (color_blend_state_ptr, color_blend_state) = match create_info.color_blend_state {
            Some(ref color_blend_state) => {
                let color_blend_state: Box<_> = Box::new(VkPipelineColorBlendStateCreateInfoWrapper::new(color_blend_state, true));
                (&color_blend_state.vks_struct as *const _, Some(color_blend_state))
            }

            None => (ptr::null(), None),
        };

        let (dynamic_state_ptr, dynamic_state) = match create_info.dynamic_state {
            Some(ref dynamic_state) => {
                let dynamic_state: Box<_> = Box::new(VkPipelineDynamicStateCreateInfoWrapper::new(dynamic_state, true));
                (&dynamic_state.vks_struct as *const _, Some(dynamic_state))
            }

            None => (ptr::null(), None),
        };

        let (base_pipeline_handle, base_pipeline) = match create_info.base_pipeline {
            Some(ref base_pipeline) => (base_pipeline.handle(), Some(base_pipeline.clone())),
            None => (Default::default(), None),
        };

        let base_pipeline_index = match create_info.base_pipeline_index {
            Some(base_pipeline_index) => base_pipeline_index as i32,
            None => -1,
        };

        let (pnext, chain) = GraphicsPipelineCreateInfoChainWrapper::new_optional(&create_info.chain, with_chain);

        VkGraphicsPipelineCreateInfoWrapper {
            vks_struct: vks::VkGraphicsPipelineCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO,
                pNext: pnext,
                flags: create_info.flags.bits(),
                stageCount: stages.len() as u32,
                pStages: vk_stages.as_ptr(),
                pVertexInputState: &vertex_input_state.vks_struct,
                pInputAssemblyState: &input_assembly_state.vks_struct,
                pTessellationState: tessellation_state_ptr,
                pViewportState: viewport_state_ptr,
                pRasterizationState: &rasterization_state.vks_struct,
                pMultisampleState: multisample_state_ptr,
                pDepthStencilState: depth_stencil_state_ptr,
                pColorBlendState: color_blend_state_ptr,
                pDynamicState: dynamic_state_ptr,
                layout: create_info.layout.handle(),
                renderPass: create_info.render_pass.handle(),
                subpass: create_info.subpass,
                basePipelineHandle: base_pipeline_handle,
                basePipelineIndex: base_pipeline_index,
            },
            stages: stages,
            vk_stages: vk_stages,
            vertex_input_state: vertex_input_state,
            input_assembly_state: input_assembly_state,
            tessellation_state: tessellation_state,
            viewport_state: viewport_state,
            rasterization_state: rasterization_state,
            multisample_state: multisample_state,
            depth_stencil_state: depth_stencil_state,
            color_blend_state: color_blend_state,
            dynamic_state: dynamic_state,
            layout: create_info.layout.clone(),
            render_pass: create_info.render_pass.clone(),
            base_pipeline: base_pipeline,
            chain: chain,
        }
    }
}

gen_chain_struct! {
    name: ComputePipelineCreateInfoChain [ComputePipelineCreateInfoChainWrapper],
    query: ComputePipelineCreateInfoChainQuery [ComputePipelineCreateInfoChainQueryWrapper],
    vks: VkComputePipelineCreateInfo,
    input: true,
    output: false,
}

/// See [`VkComputePipelineCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkComputePipelineCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct ComputePipelineCreateInfo {
    pub flags: PipelineCreateFlags,
    pub stage: PipelineShaderStageCreateInfo,
    pub layout: PipelineLayout,
    pub base_pipeline: Option<Pipeline>,
    pub base_pipeline_index: Option<u32>,
    pub chain: Option<ComputePipelineCreateInfoChain>,
}

#[derive(Debug)]
struct VkComputePipelineCreateInfoWrapper {
    pub vks_struct: vks::VkComputePipelineCreateInfo,
    stage: VkPipelineShaderStageCreateInfoWrapper,
    layout: PipelineLayout,
    base_pipeline: Option<Pipeline>,
    chain: Option<ComputePipelineCreateInfoChainWrapper>,
}

impl VkComputePipelineCreateInfoWrapper {
    pub fn new(create_info: &ComputePipelineCreateInfo, with_chain: bool) -> Self {
        let stage = VkPipelineShaderStageCreateInfoWrapper::new(&create_info.stage, true);

        let (base_pipeline_handle, base_pipeline) = match create_info.base_pipeline {
            Some(ref base_pipeline) => (base_pipeline.handle(), Some(base_pipeline.clone())),
            None => (Default::default(), None),
        };

        let base_pipeline_index = match create_info.base_pipeline_index {
            Some(base_pipeline_index) => base_pipeline_index as i32,
            None => -1,
        };

        let (pnext, chain) = ComputePipelineCreateInfoChainWrapper::new_optional(&create_info.chain, with_chain);

        VkComputePipelineCreateInfoWrapper {
            vks_struct: vks::VkComputePipelineCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO,
                pNext: pnext,
                flags: create_info.flags.bits(),
                stage: stage.vks_struct,
                layout: create_info.layout.handle(),
                basePipelineHandle: base_pipeline_handle,
                basePipelineIndex: base_pipeline_index,
            },
            stage: stage,
            layout: create_info.layout.clone(),
            base_pipeline: base_pipeline,
            chain: chain,
        }
    }
}

/// See [`VkPushConstantRange`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPushConstantRange)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct PushConstantRange {
    pub stage_flags: ShaderStageFlags,
    pub offset: u32,
    pub size: u32,
}

impl<'a> From<&'a PushConstantRange> for vks::VkPushConstantRange {
    fn from(range: &'a PushConstantRange) -> Self {
        vks::VkPushConstantRange {
            stageFlags: range.stage_flags.bits(),
            offset: range.offset,
            size: range.size,
        }
    }
}

gen_chain_struct! {
    name: PipelineLayoutCreateInfoChain [PipelineLayoutCreateInfoChainWrapper],
    query: PipelineLayoutCreateInfoChainQuery [PipelineLayoutCreateInfoChainQueryWrapper],
    vks: VkPipelineLayoutCreateInfo,
    input: true,
    output: false,
}

/// See [`VkPipelineLayoutCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineLayoutCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct PipelineLayoutCreateInfo {
    pub flags: PipelineLayoutCreateFlags,
    pub set_layouts: Option<Vec<DescriptorSetLayout>>,
    pub push_constant_ranges: Option<Vec<PushConstantRange>>,
    pub chain: Option<PipelineLayoutCreateInfoChain>,
}

#[derive(Debug)]
struct VkPipelineLayoutCreateInfoWrapper {
    pub vks_struct: vks::VkPipelineLayoutCreateInfo,
    set_layouts: Option<Vec<DescriptorSetLayout>>,
    vk_set_layouts: Option<Vec<vks::VkDescriptorSetLayout>>,
    push_constant_ranges: Option<Vec<vks::VkPushConstantRange>>,
    chain: Option<PipelineLayoutCreateInfoChainWrapper>,
}

impl VkPipelineLayoutCreateInfoWrapper {
    pub fn new(create_info: &PipelineLayoutCreateInfo, with_chain: bool) -> Self {
        let (vk_set_layouts_ptr, set_layout_count, set_layouts, vk_set_layouts) = match create_info.set_layouts {
            Some(ref set_layouts) => {
                let set_layouts = set_layouts.clone();
                let vk_set_layouts: Vec<_> = set_layouts.iter().map(DescriptorSetLayout::handle).collect();
                (vk_set_layouts.as_ptr(), set_layouts.len() as u32, Some(set_layouts), Some(vk_set_layouts))
            }

            None => (ptr::null(), 0, None, None),
        };

        let (push_constant_ranges_count, push_constant_ranges_ptr, push_constant_ranges) = match create_info.push_constant_ranges {
            Some(ref push_constant_ranges) => {
                let push_constant_ranges: Vec<_> = push_constant_ranges.iter().map(From::from).collect();
                (push_constant_ranges.len() as u32, push_constant_ranges.as_ptr(), Some(push_constant_ranges))
            }

            None => (0, ptr::null(), None),
        };

        let (pnext, chain) = PipelineLayoutCreateInfoChainWrapper::new_optional(&create_info.chain, with_chain);

        VkPipelineLayoutCreateInfoWrapper {
            vks_struct: vks::VkPipelineLayoutCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
                pNext: pnext,
                flags: create_info.flags.bits(),
                setLayoutCount: set_layout_count,
                pSetLayouts: vk_set_layouts_ptr,
                pushConstantRangeCount: push_constant_ranges_count,
                pPushConstantRanges: push_constant_ranges_ptr,
            },
            set_layouts: set_layouts,
            vk_set_layouts: vk_set_layouts,
            push_constant_ranges: push_constant_ranges,
            chain: chain,
        }
    }
}

gen_chain_struct! {
    name: SamplerCreateInfoChain [SamplerCreateInfoChainWrapper],
    query: SamplerCreateInfoChainQuery [SamplerCreateInfoChainQueryWrapper],
    vks: VkSamplerCreateInfo,
    input: true,
    output: false,
}

/// See [`VkSamplerCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSamplerCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct SamplerCreateInfo {
    pub flags: SamplerCreateFlags,
    pub mag_filter: Filter,
    pub min_filter: Filter,
    pub mipmap_mode: SamplerMipmapMode,
    pub address_mode_u: SamplerAddressMode,
    pub address_mode_v: SamplerAddressMode,
    pub address_mode_w: SamplerAddressMode,
    pub mip_lod_bias: f32,
    pub anisotropy_enable: bool,
    pub max_anisotropy: f32,
    pub compare_enable: bool,
    pub compare_op: CompareOp,
    pub min_lod: f32,
    pub max_lod: f32,
    pub border_color: BorderColor,
    pub unnormalized_coordinates: bool,
    pub chain: Option<SamplerCreateInfoChain>,
}

#[derive(Debug)]
struct VkSamplerCreateInfoWrapper {
    pub vks_struct: vks::VkSamplerCreateInfo,
    chain: Option<SamplerCreateInfoChainWrapper>,
}

impl VkSamplerCreateInfoWrapper {
    pub fn new(create_info: &SamplerCreateInfo, with_chain: bool) -> Self {
        let (pnext, chain) = SamplerCreateInfoChainWrapper::new_optional(&create_info.chain, with_chain);

        VkSamplerCreateInfoWrapper {
            vks_struct: vks::VkSamplerCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO,
                pNext: pnext,
                flags: create_info.flags.bits(),
                magFilter: create_info.mag_filter.into(),
                minFilter: create_info.min_filter.into(),
                mipmapMode: create_info.mipmap_mode.into(),
                addressModeU: create_info.address_mode_u.into(),
                addressModeV: create_info.address_mode_v.into(),
                addressModeW: create_info.address_mode_w.into(),
                mipLodBias: create_info.mip_lod_bias,
                anisotropyEnable: utils::to_vk_bool(create_info.anisotropy_enable),
                maxAnisotropy: create_info.max_anisotropy,
                compareEnable: utils::to_vk_bool(create_info.compare_enable),
                compareOp: create_info.compare_op.into(),
                minLod: create_info.min_lod,
                maxLod: create_info.max_lod,
                borderColor: create_info.border_color.into(),
                unnormalizedCoordinates: utils::to_vk_bool(create_info.unnormalized_coordinates),
            },
            chain: chain,
        }
    }
}

/// See [`VkDescriptorSetLayoutBinding`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorSetLayoutBinding)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DescriptorSetLayoutBinding {
    pub binding: u32,
    pub descriptor_type: DescriptorType,
    pub descriptor_count: u32,
    pub stage_flags: ShaderStageFlags,
    pub immutable_samplers: Option<Vec<Sampler>>,
}

#[derive(Debug)]
struct VkDescriptorSetLayoutBindingWrapper {
    pub vks_struct: vks::VkDescriptorSetLayoutBinding,
    immutable_samplers: Option<Vec<Sampler>>,
    immutable_vk_samplers: Option<Vec<vks::VkSampler>>,
}

impl<'a> From<&'a DescriptorSetLayoutBinding> for VkDescriptorSetLayoutBindingWrapper {
    fn from(binding: &'a DescriptorSetLayoutBinding) -> Self {
        let immutable_samplers = binding.immutable_samplers.clone();

        let mut immutable_vk_samplers_ptr = ptr::null();
        let immutable_vk_samplers = immutable_samplers.as_ref().map(|s| {
            let immutable_vk_samplers: Vec<_> = s.iter().map(Sampler::handle).collect();
            immutable_vk_samplers_ptr = immutable_vk_samplers.as_ptr();
            immutable_vk_samplers
        });

        VkDescriptorSetLayoutBindingWrapper {
            vks_struct: vks::VkDescriptorSetLayoutBinding {
                binding: binding.binding,
                descriptorType: binding.descriptor_type.into(),
                descriptorCount: binding.descriptor_count,
                stageFlags: binding.stage_flags.bits(),
                pImmutableSamplers: immutable_vk_samplers_ptr,
            },
            immutable_samplers: immutable_samplers,
            immutable_vk_samplers: immutable_vk_samplers,
        }
    }
}

gen_chain_struct! {
    name: DescriptorSetLayoutCreateInfoChain [DescriptorSetLayoutCreateInfoChainWrapper],
    query: DescriptorSetLayoutCreateInfoChainQuery [DescriptorSetLayoutCreateInfoChainQueryWrapper],
    vks: VkDescriptorSetLayoutCreateInfo,
    input: true,
    output: false,
}

/// See [`VkDescriptorSetLayoutCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorSetLayoutCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct DescriptorSetLayoutCreateInfo {
    pub flags: DescriptorSetLayoutCreateFlags,
    pub bindings: Option<Vec<DescriptorSetLayoutBinding>>,
    pub chain: Option<DescriptorSetLayoutCreateInfoChain>,
}

#[derive(Debug)]
struct VkDescriptorSetLayoutCreateInfoWrapper {
    pub vks_struct: vks::VkDescriptorSetLayoutCreateInfo,
    bindings: Option<Vec<VkDescriptorSetLayoutBindingWrapper>>,
    vk_bindings: Option<Vec<vks::VkDescriptorSetLayoutBinding>>,
    chain: Option<DescriptorSetLayoutCreateInfoChainWrapper>,
}

impl VkDescriptorSetLayoutCreateInfoWrapper {
    pub fn new(create_info: &DescriptorSetLayoutCreateInfo, with_chain: bool) -> Self {
        let (vk_bindings_ptr, binding_count, bindings, vk_bindings) = match create_info.bindings {
            Some(ref bindings) => {
                let bindings: Vec<VkDescriptorSetLayoutBindingWrapper> = bindings.iter().map(From::from).collect();
                let vk_bindings: Vec<_> = bindings.iter().map(|b| b.vks_struct).collect();
                (vk_bindings.as_ptr(), bindings.len() as u32, Some(bindings), Some(vk_bindings))
            }

            None => (ptr::null(), 0, None, None),
        };

        let (pnext, chain) = DescriptorSetLayoutCreateInfoChainWrapper::new_optional(&create_info.chain, with_chain);

        VkDescriptorSetLayoutCreateInfoWrapper {
            vks_struct: vks::VkDescriptorSetLayoutCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
                pNext: pnext,
                flags: create_info.flags.bits(),
                bindingCount: binding_count,
                pBindings: vk_bindings_ptr,
            },
            bindings: bindings,
            vk_bindings: vk_bindings,
            chain: chain,
        }
    }
}

/// See [`VkDescriptorPoolSize`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorPoolSize)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DescriptorPoolSize {
    pub descriptor_type: DescriptorType,
    pub descriptor_count: u32,
}

impl<'a> From<&'a DescriptorPoolSize> for vks::VkDescriptorPoolSize {
    fn from(size: &'a DescriptorPoolSize) -> Self {
        vks::VkDescriptorPoolSize {
            type_: size.descriptor_type.into(),
            descriptorCount: size.descriptor_count,
        }
    }
}

gen_chain_struct! {
    name: DescriptorPoolCreateInfoChain [DescriptorPoolCreateInfoChainWrapper],
    query: DescriptorPoolCreateInfoChainQuery [DescriptorPoolCreateInfoChainQueryWrapper],
    vks: VkDescriptorPoolCreateInfo,
    input: true,
    output: false,
}

/// See [`VkDescriptorPoolCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorPoolCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct DescriptorPoolCreateInfo {
    pub flags: DescriptorPoolCreateFlags,
    pub max_sets: u32,
    pub pool_sizes: Vec<DescriptorPoolSize>,
    pub chain: Option<DescriptorPoolCreateInfoChain>,
}

#[derive(Debug)]
struct VkDescriptorPoolCreateInfoWrapper {
    pub vks_struct: vks::VkDescriptorPoolCreateInfo,
    pool_sizes: Vec<vks::VkDescriptorPoolSize>,
    chain: Option<DescriptorPoolCreateInfoChainWrapper>,
}

impl VkDescriptorPoolCreateInfoWrapper {
    pub fn new(create_info: &DescriptorPoolCreateInfo, with_chain: bool) -> Self {
        let pool_sizes: Vec<_> = create_info.pool_sizes
            .iter()
            .map(From::from)
            .collect();

        let (pnext, chain) = DescriptorPoolCreateInfoChainWrapper::new_optional(&create_info.chain, with_chain);

        VkDescriptorPoolCreateInfoWrapper {
            vks_struct: vks::VkDescriptorPoolCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
                pNext: pnext,
                flags: create_info.flags.bits(),
                maxSets: create_info.max_sets,
                poolSizeCount: pool_sizes.len() as u32,
                pPoolSizes: pool_sizes.as_ptr(),
            },
            pool_sizes: pool_sizes,
            chain: chain,
        }
    }
}

gen_chain_struct! {
    name: DescriptorSetAllocateInfoChain [DescriptorSetAllocateInfoChainWrapper],
    query: DescriptorSetAllocateInfoChainQuery [DescriptorSetAllocateInfoChainQueryWrapper],
    vks: VkDescriptorSetAllocateInfo,
    input: true,
    output: false,
}

/// See [`VkDescriptorSetAllocateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorSetAllocateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct DescriptorSetAllocateInfo {
    pub descriptor_pool: DescriptorPool,
    pub set_layouts: Vec<DescriptorSetLayout>,
    pub chain: Option<DescriptorSetAllocateInfoChain>,
}

#[derive(Debug)]
struct VkDescriptorSetAllocateInfoWrapper {
    pub vks_struct: vks::VkDescriptorSetAllocateInfo,
    descriptor_pool: DescriptorPool,
    set_layouts: Vec<DescriptorSetLayout>,
    vk_set_layouts: Vec<vks::VkDescriptorSetLayout>,
    chain: Option<DescriptorSetAllocateInfoChainWrapper>,
}

impl VkDescriptorSetAllocateInfoWrapper {
    pub fn new(allocate_info: &DescriptorSetAllocateInfo, with_chain: bool) -> Self {
        let vk_set_layouts: Vec<_> = allocate_info.set_layouts.iter().map(DescriptorSetLayout::handle).collect();
        let (pnext, chain) = DescriptorSetAllocateInfoChainWrapper::new_optional(&allocate_info.chain, with_chain);

        VkDescriptorSetAllocateInfoWrapper {
            vks_struct: vks::VkDescriptorSetAllocateInfo {
                sType: vks::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,
                pNext: pnext,
                descriptorPool: allocate_info.descriptor_pool.handle(),
                descriptorSetCount: vk_set_layouts.len() as u32,
                pSetLayouts: vk_set_layouts.as_ptr(),
            },
            descriptor_pool: allocate_info.descriptor_pool.clone(),
            set_layouts: allocate_info.set_layouts.clone(),
            vk_set_layouts: vk_set_layouts,
            chain: chain,
        }
    }
}

/// See [`VkDescriptorImageInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorImageInfo)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DescriptorImageInfo {
    pub sampler: Option<Sampler>,
    pub image_view: Option<ImageView>,
    pub image_layout: ImageLayout,
}

#[derive(Debug)]
struct VkDescriptorImageInfoWrapper {
    pub vks_struct: vks::VkDescriptorImageInfo,
    sampler: Option<Sampler>,
    image_view: Option<ImageView>,
}

impl<'a> From<&'a DescriptorImageInfo> for VkDescriptorImageInfoWrapper {
    fn from(info: &'a DescriptorImageInfo) -> Self {
        let (vk_sampler, sampler) = match info.sampler {
            Some(ref sampler) => (sampler.handle(), Some(sampler.clone())),
            None => (Default::default(), None),
        };

        let (vk_image_view, image_view) = match info.image_view {
            Some(ref image_view) => (image_view.handle(), Some(image_view.clone())),
            None => (Default::default(), None),
        };

        VkDescriptorImageInfoWrapper {
            vks_struct: vks::VkDescriptorImageInfo {
                sampler: vk_sampler,
                imageView: vk_image_view,
                imageLayout: info.image_layout.into(),
            },
            sampler: sampler,
            image_view: image_view,
        }
    }
}

/// See [`VkDescriptorBufferInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorBufferInfo)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DescriptorBufferInfo {
    pub buffer: Buffer,
    pub offset: u64,
    pub range: OptionalDeviceSize,
}

#[derive(Debug)]
struct VkDescriptorBufferInfoWrapper {
    pub vks_struct: vks::VkDescriptorBufferInfo,
    buffer: Buffer,
}

impl<'a> From<&'a DescriptorBufferInfo> for VkDescriptorBufferInfoWrapper {
    fn from(info: &'a DescriptorBufferInfo) -> Self {
        VkDescriptorBufferInfoWrapper {
            vks_struct: vks::VkDescriptorBufferInfo {
                buffer: info.buffer.handle(),
                offset: info.offset,
                range: info.range.into(),
            },
            buffer: info.buffer.clone(),
        }
    }
}

gen_chain_struct! {
    name: WriteDescriptorSetChain [WriteDescriptorSetChainWrapper],
    query: WriteDescriptorSetChainQuery [WriteDescriptorSetChainQueryWrapper],
    vks: VkWriteDescriptorSet,
    input: true,
    output: false,
}

/// See [`VkWriteDescriptorSet`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkWriteDescriptorSet)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WriteDescriptorSetElements {
    ImageInfo(Vec<DescriptorImageInfo>),
    BufferInfo(Vec<DescriptorBufferInfo>),
    TexelBufferView(Vec<BufferView>),
}

/// See [`VkWriteDescriptorSet`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkWriteDescriptorSet)
#[derive(Debug, Clone, PartialEq)]
pub struct WriteDescriptorSet {
    pub dst_set: DescriptorSet,
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_type: DescriptorType,
    pub elements: WriteDescriptorSetElements,
    pub chain: Option<WriteDescriptorSetChain>,
}

#[derive(Debug)]
struct VkWriteDescriptorSetWrapper {
    pub vks_struct: vks::VkWriteDescriptorSet,
    dst_set: DescriptorSet,
    image_info: Option<Vec<VkDescriptorImageInfoWrapper>>,
    vk_image_info: Option<Vec<vks::VkDescriptorImageInfo>>,
    buffer_info: Option<Vec<VkDescriptorBufferInfoWrapper>>,
    vk_buffer_info: Option<Vec<vks::VkDescriptorBufferInfo>>,
    texel_buffer_view: Option<Vec<BufferView>>,
    vk_texel_buffer_view: Option<Vec<vks::VkBufferView>>,
    chain: Option<WriteDescriptorSetChainWrapper>,
}

impl VkWriteDescriptorSetWrapper {
    pub fn new(write: &WriteDescriptorSet, with_chain: bool) -> Self {
        let (count,
             vk_image_info_ptr,
             vk_image_info,
             image_info,
             vk_buffer_info_ptr,
             vk_buffer_info,
             buffer_info,
             vk_texel_buffer_view_ptr,
             vk_texel_buffer_view,
             texel_buffer_view) = match write.elements {
            WriteDescriptorSetElements::ImageInfo(ref image_info) => {
                let image_info: Vec<VkDescriptorImageInfoWrapper> = image_info.iter().map(From::from).collect();
                let vk_image_info: Vec<_> = image_info.iter().map(|i| i.vks_struct).collect();
                (image_info.len() as u32, vk_image_info.as_ptr(), Some(vk_image_info), Some(image_info), ptr::null(), None, None, ptr::null(), None, None)
            },

            WriteDescriptorSetElements::BufferInfo(ref buffer_info) => {
                let buffer_info: Vec<VkDescriptorBufferInfoWrapper> = buffer_info.iter().map(From::from).collect();
                let vk_buffer_info: Vec<_> = buffer_info.iter().map(|b| b.vks_struct).collect();
                (buffer_info.len() as u32, ptr::null(), None, None, vk_buffer_info.as_ptr(), Some(vk_buffer_info), Some(buffer_info), ptr::null(), None, None)
            },

            WriteDescriptorSetElements::TexelBufferView(ref texel_buffer_view) => {
                let texel_buffer_view = texel_buffer_view.clone();
                let vk_texel_buffer_view: Vec<_> = texel_buffer_view.iter().map(BufferView::handle).collect();
                (texel_buffer_view.len() as u32, ptr::null(), None, None, ptr::null(), None, None, vk_texel_buffer_view.as_ptr(), Some(vk_texel_buffer_view), Some(texel_buffer_view))
            },
        };

        let (pnext, chain) = WriteDescriptorSetChainWrapper::new_optional(&write.chain, with_chain);

        VkWriteDescriptorSetWrapper {
            vks_struct: vks::VkWriteDescriptorSet {
                sType: vks::VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
                pNext: pnext,
                dstSet: write.dst_set.handle(),
                dstBinding: write.dst_binding,
                dstArrayElement: write.dst_array_element,
                descriptorCount: count,
                descriptorType: write.descriptor_type.into(),
                pImageInfo: vk_image_info_ptr,
                pBufferInfo: vk_buffer_info_ptr,
                pTexelBufferView: vk_texel_buffer_view_ptr,
            },
            dst_set: write.dst_set.clone(),
            image_info: image_info,
            vk_image_info: vk_image_info,
            buffer_info: buffer_info,
            vk_buffer_info: vk_buffer_info,
            texel_buffer_view: texel_buffer_view,
            vk_texel_buffer_view: vk_texel_buffer_view,
            chain: chain,
        }
    }
}

gen_chain_struct! {
    name: CopyDescriptorSetChain [CopyDescriptorSetChainWrapper],
    query: CopyDescriptorSetChainQuery [CopyDescriptorSetChainQueryWrapper],
    vks: VkCopyDescriptorSet,
    input: true,
    output: false,
}

/// See [`VkCopyDescriptorSet`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCopyDescriptorSet)
#[derive(Debug, Clone, PartialEq)]
pub struct CopyDescriptorSet {
    pub src_set: DescriptorSet,
    pub src_binding: u32,
    pub src_array_element: u32,
    pub dst_set: DescriptorSet,
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_count: u32,
    pub chain: Option<CopyDescriptorSetChain>,
}

#[derive(Debug)]
struct VkCopyDescriptorSetWrapper {
    pub vks_struct: vks::VkCopyDescriptorSet,
    src_set: DescriptorSet,
    dst_set: DescriptorSet,
    chain: Option<CopyDescriptorSetChainWrapper>,
}

impl VkCopyDescriptorSetWrapper {
    pub fn new(copy: &CopyDescriptorSet, with_chain: bool) -> Self {
        let (pnext, chain) = CopyDescriptorSetChainWrapper::new_optional(&copy.chain, with_chain);

        VkCopyDescriptorSetWrapper {
            vks_struct: vks::VkCopyDescriptorSet {
                sType: vks::VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET,
                pNext: pnext,
                srcSet: copy.src_set.handle(),
                srcBinding: copy.src_binding,
                srcArrayElement: copy.src_array_element,
                dstSet: copy.dst_set.handle(),
                dstBinding: copy.dst_binding,
                dstArrayElement: copy.dst_array_element,
                descriptorCount: copy.descriptor_count,
            },
            src_set: copy.src_set.clone(),
            dst_set: copy.dst_set.clone(),
            chain: chain,
        }
    }
}

gen_chain_struct! {
    name: FramebufferCreateInfoChain [FramebufferCreateInfoChainWrapper],
    query: FramebufferCreateInfoChainQuery [FramebufferCreateInfoChainQueryWrapper],
    vks: VkFramebufferCreateInfo,
    input: true,
    output: false,
}

/// See [`VkFramebufferCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFramebufferCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct FramebufferCreateInfo {
    pub flags: FramebufferCreateFlags,
    pub render_pass: RenderPass,
    pub attachments: Option<Vec<ImageView>>,
    pub width: u32,
    pub height: u32,
    pub layers: u32,
    pub chain: Option<FramebufferCreateInfoChain>,
}

#[derive(Debug)]
struct VkFramebufferCreateInfoWrapper {
    pub vks_struct: vks::VkFramebufferCreateInfo,
    render_pass: RenderPass,
    attachments: Option<Vec<ImageView>>,
    vk_attachments: Option<Vec<vks::VkImageView>>,
    chain: Option<FramebufferCreateInfoChainWrapper>,
}

impl VkFramebufferCreateInfoWrapper {
    pub fn new(create_info: &FramebufferCreateInfo, with_chain: bool) -> Self {
        let (attachments_count, vk_attachments_ptr, attachments, vk_attachments) = match create_info.attachments {
            Some(ref attachments) => {
                let attachments = attachments.clone();
                let vk_attachments: Vec<_> = attachments.iter().map(ImageView::handle).collect();
                (attachments.len() as u32, vk_attachments.as_ptr(), Some(attachments), Some(vk_attachments))
            }

            None => (0, ptr::null(), None, None),
        };

        let (pnext, chain) = FramebufferCreateInfoChainWrapper::new_optional(&create_info.chain, with_chain);

        VkFramebufferCreateInfoWrapper {
            vks_struct: vks::VkFramebufferCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
                pNext: pnext,
                flags: create_info.flags.bits(),
                renderPass: create_info.render_pass.handle(),
                attachmentCount: attachments_count,
                pAttachments: vk_attachments_ptr,
                width: create_info.width,
                height: create_info.height,
                layers: create_info.layers,
            },
            render_pass: create_info.render_pass.clone(),
            attachments: attachments,
            vk_attachments: vk_attachments,
            chain: chain,
        }
    }
}

/// See [`VkAttachmentDescription`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAttachmentDescription)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct AttachmentDescription {
    pub flags: AttachmentDescriptionFlags,
    pub format: Format,
    pub samples: SampleCountFlagBits,
    pub load_op: AttachmentLoadOp,
    pub store_op: AttachmentStoreOp,
    pub stencil_load_op: AttachmentLoadOp,
    pub stencil_store_op: AttachmentStoreOp,
    pub initial_layout: ImageLayout,
    pub final_layout: ImageLayout,
}

impl<'a> From<&'a AttachmentDescription> for vks::VkAttachmentDescription {
    fn from(description: &'a AttachmentDescription) -> Self {
        vks::VkAttachmentDescription {
            flags: description.flags.bits(),
            format: description.format.into(),
            samples: description.samples.bits(),
            loadOp: description.load_op.into(),
            storeOp: description.store_op.into(),
            stencilLoadOp: description.stencil_load_op.into(),
            stencilStoreOp: description.stencil_store_op.into(),
            initialLayout: description.initial_layout.into(),
            finalLayout: description.final_layout.into(),
        }
    }
}

impl<'a> From<&'a vks::VkAttachmentDescription> for AttachmentDescription {
    fn from(description: &'a vks::VkAttachmentDescription) -> Self {
        AttachmentDescription {
            flags: AttachmentDescriptionFlags::from_bits_truncate(description.flags),
            format: description.format.into(),
            samples: SampleCountFlags::from_bits_truncate(description.samples),
            load_op: description.loadOp.into(),
            store_op: description.storeOp.into(),
            stencil_load_op: description.stencilLoadOp.into(),
            stencil_store_op: description.stencilStoreOp.into(),
            initial_layout: description.initialLayout.into(),
            final_layout: description.finalLayout.into(),
        }
    }
}

/// See [`VkAttachmentReference`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAttachmentReference)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct AttachmentReference {
    pub attachment: AttachmentIndex,
    pub layout: ImageLayout,
}

impl<'a> From<&'a AttachmentReference> for vks::VkAttachmentReference {
    fn from(reference: &'a AttachmentReference) -> Self {
        vks::VkAttachmentReference {
            attachment: reference.attachment.into(),
            layout: reference.layout.into(),
        }
    }
}

impl<'a> From<&'a vks::VkAttachmentReference> for AttachmentReference {
    fn from(reference: &'a vks::VkAttachmentReference) -> Self {
        AttachmentReference {
            attachment: reference.attachment.into(),
            layout: reference.layout.into(),
        }
    }
}

/// See [`VkSubpassDescription`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSubpassDescription)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SubpassDescription {
    pub flags: SubpassDescriptionFlags,
    pub pipeline_bind_point: PipelineBindPoint,
    pub input_attachments: Option<Vec<AttachmentReference>>,
    pub color_attachments: Option<Vec<AttachmentReference>>,
    pub resolve_attachments: Option<Vec<AttachmentReference>>,
    pub depth_stencil_attachment: Option<AttachmentReference>,
    pub preserve_attachments: Option<Vec<u32>>,
}

impl<'a> From<&'a vks::VkSubpassDescription> for SubpassDescription {
    fn from(description: &'a vks::VkSubpassDescription) -> Self {
        let input_attachments = if !description.pInputAttachments.is_null() {
            let input_attachments = unsafe { slice::from_raw_parts(description.pInputAttachments, description.inputAttachmentCount as usize) };
            Some(input_attachments.iter().map(AttachmentReference::from).collect())
        }
        else {
            None
        };

        let color_attachments = if !description.pColorAttachments.is_null() {
            let color_attachments = unsafe { slice::from_raw_parts(description.pColorAttachments, description.colorAttachmentCount as usize) };
            Some(color_attachments.iter().map(AttachmentReference::from).collect())
        }
        else {
            None
        };

        let resolve_attachments = if !description.pResolveAttachments.is_null() {
            let resolve_attachments = unsafe { slice::from_raw_parts(description.pResolveAttachments, description.colorAttachmentCount as usize) };
            Some(resolve_attachments.iter().map(AttachmentReference::from).collect())
        }
        else {
            None
        };

        let depth_stencil_attachment = if !description.pDepthStencilAttachment.is_null() {
            unsafe { Some((&*description.pDepthStencilAttachment).into()) }
        }
        else {
            None
        };

        let preserve_attachments = if !description.pPreserveAttachments.is_null() {
            unsafe { Some(slice::from_raw_parts(description.pPreserveAttachments, description.preserveAttachmentCount as usize).to_vec()) }
        }
        else {
            None
        };

        SubpassDescription {
            flags: SubpassDescriptionFlags::from_bits_truncate(description.flags),
            pipeline_bind_point: description.pipelineBindPoint.into(),
            input_attachments: input_attachments,
            color_attachments: color_attachments,
            resolve_attachments: resolve_attachments,
            depth_stencil_attachment: depth_stencil_attachment,
            preserve_attachments: preserve_attachments,
        }
    }
}

#[derive(Debug)]
struct VkSubpassDescriptionWrapper {
    pub vks_struct: vks::VkSubpassDescription,
    input_attachments: Option<Vec<vks::VkAttachmentReference>>,
    color_attachments: Option<Vec<vks::VkAttachmentReference>>,
    resolve_attachments: Option<Vec<vks::VkAttachmentReference>>,
    depth_stencil_attachment: Option<Box<vks::VkAttachmentReference>>,
    preserve_attachments: Option<Vec<u32>>,
}

impl<'a> From<&'a SubpassDescription> for VkSubpassDescriptionWrapper {
    fn from(description: &'a SubpassDescription) -> Self {
        let (input_attachments_count, input_attachments_ptr, input_attachments) = match description.input_attachments {
            Some(ref input_attachments) => {
                let input_attachments: Vec<_> = input_attachments.iter().map(From::from).collect();
                (input_attachments.len() as u32, input_attachments.as_ptr(), Some(input_attachments))
            }

            None => (0, ptr::null(), None),
        };

        let (color_attachments_count, color_attachments_ptr, color_attachments) = match description.color_attachments {
            Some(ref color_attachments) => {
                let color_attachments: Vec<_> = color_attachments.iter().map(From::from).collect();
                (color_attachments.len() as u32, color_attachments.as_ptr(), Some(color_attachments))
            }

            None => (0, ptr::null(), None),
        };

        let (resolve_attachments_ptr, resolve_attachments) = match description.resolve_attachments {
            Some(ref resolve_attachments) => {
                let resolve_attachments: Vec<_> = resolve_attachments.iter().map(From::from).collect();
                (resolve_attachments.as_ptr(), Some(resolve_attachments))
            }

            None => (ptr::null(), None),
        };

        let (depth_stencil_attachment_ptr, depth_stencil_attachment) = match description.depth_stencil_attachment {
            Some(ref depth_stencil_attachment) => {
                let depth_stencil_attachment = Box::new(depth_stencil_attachment.into());
                (&*depth_stencil_attachment as *const _, Some(depth_stencil_attachment))
            }

            None => (ptr::null(), None),
        };

        let (preserve_attachments_count, preserve_attachments_ptr, preserve_attachments) = match description.preserve_attachments {
            Some(ref preserve_attachments) => {
                let preserve_attachments = preserve_attachments.clone();
                (preserve_attachments.len() as u32, preserve_attachments.as_ptr(), Some(preserve_attachments))
            }

            None => (0, ptr::null(), None),
        };

        VkSubpassDescriptionWrapper {
            vks_struct: vks::VkSubpassDescription {
                flags: description.flags.bits(),
                pipelineBindPoint: description.pipeline_bind_point.into(),
                inputAttachmentCount: input_attachments_count,
                pInputAttachments: input_attachments_ptr,
                colorAttachmentCount: color_attachments_count,
                pColorAttachments: color_attachments_ptr,
                pResolveAttachments: resolve_attachments_ptr,
                pDepthStencilAttachment: depth_stencil_attachment_ptr,
                preserveAttachmentCount: preserve_attachments_count,
                pPreserveAttachments: preserve_attachments_ptr,
            },
            input_attachments: input_attachments,
            color_attachments: color_attachments,
            resolve_attachments: resolve_attachments,
            depth_stencil_attachment: depth_stencil_attachment,
            preserve_attachments: preserve_attachments,
        }
    }
}

/// See [`VkSubpassDependency`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSubpassDependency)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SubpassDependency {
    pub src_subpass: SubpassIndex,
    pub dst_subpass: SubpassIndex,
    pub src_stage_mask: PipelineStageFlags,
    pub dst_stage_mask: PipelineStageFlags,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub dependency_flags: DependencyFlags,
}

impl<'a> From<&'a SubpassDependency> for vks::VkSubpassDependency {
    fn from(dependency: &'a SubpassDependency) -> Self {
        vks::VkSubpassDependency {
            srcSubpass: dependency.src_subpass.into(),
            dstSubpass: dependency.dst_subpass.into(),
            srcStageMask: dependency.src_stage_mask.bits(),
            dstStageMask: dependency.dst_stage_mask.bits(),
            srcAccessMask: dependency.src_access_mask.bits(),
            dstAccessMask: dependency.dst_access_mask.bits(),
            dependencyFlags: dependency.dependency_flags.bits(),
        }
    }
}

impl<'a> From<&'a vks::VkSubpassDependency> for SubpassDependency {
    fn from(dependency: &'a vks::VkSubpassDependency) -> Self {
        SubpassDependency {
            src_subpass: dependency.srcSubpass.into(),
            dst_subpass: dependency.dstSubpass.into(),
            src_stage_mask: PipelineStageFlags::from_bits_truncate(dependency.srcStageMask),
            dst_stage_mask: PipelineStageFlags::from_bits_truncate(dependency.dstStageMask),
            src_access_mask: AccessFlags::from_bits_truncate(dependency.srcAccessMask),
            dst_access_mask: AccessFlags::from_bits_truncate(dependency.dstAccessMask),
            dependency_flags: DependencyFlags::from_bits_truncate(dependency.dependencyFlags),
        }
    }
}

gen_chain_struct! {
    name: RenderPassCreateInfoChain [RenderPassCreateInfoChainWrapper],
    query: RenderPassCreateInfoChainQuery [RenderPassCreateInfoChainQueryWrapper],
    vks: VkRenderPassCreateInfo,
    input: true,
    output: false,
}

/// See [`VkRenderPassCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkRenderPassCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct RenderPassCreateInfo {
    pub flags: RenderPassCreateFlags,
    pub attachments: Option<Vec<AttachmentDescription>>,
    pub subpasses: Vec<SubpassDescription>,
    pub dependencies: Option<Vec<SubpassDependency>>,
    pub chain: Option<RenderPassCreateInfoChain>,
}

#[derive(Debug)]
struct VkRenderPassCreateInfoWrapper {
    pub vks_struct: vks::VkRenderPassCreateInfo,
    attachments: Option<Vec<vks::VkAttachmentDescription>>,
    subpasses: Vec<VkSubpassDescriptionWrapper>,
    vk_subpasses: Vec<vks::VkSubpassDescription>,
    dependencies: Option<Vec<vks::VkSubpassDependency>>,
    chain: Option<RenderPassCreateInfoChainWrapper>,
}

impl VkRenderPassCreateInfoWrapper {
    pub fn new(create_info: &RenderPassCreateInfo, with_chain: bool) -> Self {
        let (attachments_count, attachments_ptr, attachments) = match create_info.attachments {
            Some(ref attachments) => {
                let attachments: Vec<_> = attachments.iter().map(From::from).collect();
                (attachments.len() as u32, attachments.as_ptr(), Some(attachments))
            }

            None => (0, ptr::null(), None),
        };

        let subpasses: Vec<VkSubpassDescriptionWrapper> = create_info.subpasses.iter().map(From::from).collect();
        let vk_subpasses: Vec<vks::VkSubpassDescription> = subpasses.iter().map(|s| s.vks_struct).collect();

        let (dependencies_count, dependencies_ptr, dependencies) = match create_info.dependencies {
            Some(ref dependencies) => {
                let dependencies: Vec<_> = dependencies.iter().map(From::from).collect();
                (dependencies.len() as u32, dependencies.as_ptr(), Some(dependencies))
            }

            None => (0, ptr::null(), None),
        };

        let (pnext, chain) = RenderPassCreateInfoChainWrapper::new_optional(&create_info.chain, with_chain);

        VkRenderPassCreateInfoWrapper {
            vks_struct: vks::VkRenderPassCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO,
                pNext: pnext,
                flags: create_info.flags.bits(),
                attachmentCount: attachments_count,
                pAttachments: attachments_ptr,
                subpassCount: subpasses.len() as u32,
                pSubpasses: vk_subpasses.as_ptr(),
                dependencyCount: dependencies_count,
                pDependencies: dependencies_ptr,
            },
            attachments: attachments,
            subpasses: subpasses,
            vk_subpasses: vk_subpasses,
            dependencies: dependencies,
            chain: chain,
        }
    }
}

gen_chain_struct! {
    name: CommandPoolCreateInfoChain [CommandPoolCreateInfoChainWrapper],
    query: CommandPoolCreateInfoChainQuery [CommandPoolCreateInfoChainQueryWrapper],
    vks: VkCommandPoolCreateInfo,
    input: true,
    output: false,
}

/// See [`VkCommandPoolCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandPoolCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct CommandPoolCreateInfo {
    pub flags: CommandPoolCreateFlags,
    pub queue_family_index: u32,
    pub chain: Option<CommandPoolCreateInfoChain>,
}

#[derive(Debug)]
struct VkCommandPoolCreateInfoWrapper {
    pub vks_struct: vks::VkCommandPoolCreateInfo,
    chain: Option<CommandPoolCreateInfoChainWrapper>,
}

impl VkCommandPoolCreateInfoWrapper {
    pub fn new(create_info: &CommandPoolCreateInfo, with_chain: bool) -> Self {
        let (pnext, chain) = CommandPoolCreateInfoChainWrapper::new_optional(&create_info.chain, with_chain);

        VkCommandPoolCreateInfoWrapper {
            vks_struct: vks::VkCommandPoolCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
                pNext: pnext,
                flags: create_info.flags.bits(),
                queueFamilyIndex: create_info.queue_family_index,
            },
            chain: chain,
        }
    }
}

gen_chain_struct! {
    name: CommandBufferAllocateInfoChain [CommandBufferAllocateInfoChainWrapper],
    query: CommandBufferAllocateInfoChainQuery [CommandBufferAllocateInfoChainQueryWrapper],
    vks: VkCommandBufferAllocateInfo,
    input: true,
    output: false,
}

/// See [`VkCommandBufferAllocateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferAllocateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct CommandBufferAllocateInfo {
    pub command_pool: CommandPool,
    pub level: CommandBufferLevel,
    pub command_buffer_count: u32,
    pub chain: Option<CommandBufferAllocateInfoChain>,
}

#[derive(Debug)]
struct VkCommandBufferAllocateInfoWrapper {
    pub vks_struct: vks::VkCommandBufferAllocateInfo,
    command_pool: CommandPool,
    chain: Option<CommandBufferAllocateInfoChainWrapper>,
}

impl VkCommandBufferAllocateInfoWrapper {
    pub fn new(info: &CommandBufferAllocateInfo, with_chain: bool) -> Self {
        let (pnext, chain) = CommandBufferAllocateInfoChainWrapper::new_optional(&info.chain, with_chain);

        VkCommandBufferAllocateInfoWrapper {
            vks_struct: vks::VkCommandBufferAllocateInfo {
                sType: vks::VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
                pNext: pnext,
                commandPool: info.command_pool.handle(),
                level: info.level.into(),
                commandBufferCount: info.command_buffer_count,
            },
            command_pool: info.command_pool.clone(),
            chain: chain,
        }
    }
}

gen_chain_struct! {
    name: CommandBufferInheritanceInfoChain [CommandBufferInheritanceInfoChainWrapper],
    query: CommandBufferInheritanceInfoChainQuery [CommandBufferInheritanceInfoChainQueryWrapper],
    vks: VkCommandBufferInheritanceInfo,
    input: true,
    output: false,
}

/// See [`VkCommandBufferInheritanceInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferInheritanceInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct CommandBufferInheritanceInfo {
    pub render_pass: Option<RenderPass>,
    pub subpass: u32,
    pub framebuffer: Option<Framebuffer>,
    pub occlusion_query_enable: bool,
    pub query_flags: QueryControlFlags,
    pub pipeline_statistics: QueryPipelineStatisticFlags,
    pub chain: Option<CommandBufferInheritanceInfoChain>,
}

#[derive(Debug)]
struct VkCommandBufferInheritanceInfoWrapper {
    pub vks_struct: vks::VkCommandBufferInheritanceInfo,
    render_pass: Option<RenderPass>,
    framebuffer: Option<Framebuffer>,
    chain: Option<CommandBufferInheritanceInfoChainWrapper>,
}

impl VkCommandBufferInheritanceInfoWrapper {
    pub fn new(info: &CommandBufferInheritanceInfo, with_chain: bool) -> Self {
        let (render_pass_handle, render_pass) = match info.render_pass {
            Some(ref render_pass) => (render_pass.handle(), Some(render_pass.clone())),
            None => (Default::default(), None),
        };

        let (framebuffer_handle, framebuffer) = match info.framebuffer {
            Some(ref framebuffer) => (framebuffer.handle(), Some(framebuffer.clone())),
            None => (Default::default(), None),
        };

        let (pnext, chain) = CommandBufferInheritanceInfoChainWrapper::new_optional(&info.chain, with_chain);

        VkCommandBufferInheritanceInfoWrapper {
            vks_struct: vks::VkCommandBufferInheritanceInfo {
                sType: vks::VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO,
                pNext: pnext,
                renderPass: render_pass_handle,
                subpass: info.subpass,
                framebuffer: framebuffer_handle,
                occlusionQueryEnable: utils::to_vk_bool(info.occlusion_query_enable),
                queryFlags: info.query_flags.bits(),
                pipelineStatistics: info.pipeline_statistics.bits(),
            },
            render_pass: render_pass,
            framebuffer: framebuffer,
            chain: chain,
        }
    }
}

gen_chain_struct! {
    name: CommandBufferBeginInfoChain [CommandBufferBeginInfoChainWrapper],
    query: CommandBufferBeginInfoChainQuery [CommandBufferBeginInfoChainQueryWrapper],
    vks: VkCommandBufferBeginInfo,
    input: true,
    output: false,
}

/// See [`VkCommandBufferBeginInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferBeginInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct CommandBufferBeginInfo {
    pub flags: CommandBufferUsageFlags,
    pub inheritance_info: Option<CommandBufferInheritanceInfo>,
    pub chain: Option<CommandBufferBeginInfoChain>,
}

#[derive(Debug)]
struct VkCommandBufferBeginInfoWrapper {
    pub vks_struct: vks::VkCommandBufferBeginInfo,
    inheritance_info: Option<Box<VkCommandBufferInheritanceInfoWrapper>>,
    chain: Option<CommandBufferBeginInfoChainWrapper>,
}

impl VkCommandBufferBeginInfoWrapper {
    pub fn new(begin_info: &CommandBufferBeginInfo, with_chain: bool) -> Self {
        let (inheritance_info_ptr, inheritance_info) = match begin_info.inheritance_info {
            Some(ref inheritance_info) => {
                let inheritance_info: Box<_> = Box::new(VkCommandBufferInheritanceInfoWrapper::new(inheritance_info, true));
                (&inheritance_info.vks_struct as *const _, Some(inheritance_info))
            }

            None => (ptr::null(), None),
        };

        let (pnext, chain) = CommandBufferBeginInfoChainWrapper::new_optional(&begin_info.chain, with_chain);

        VkCommandBufferBeginInfoWrapper {
            vks_struct: vks::VkCommandBufferBeginInfo {
                sType: vks::VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
                pNext: pnext,
                flags: begin_info.flags.bits(),
                pInheritanceInfo: inheritance_info_ptr,
            },
            inheritance_info: inheritance_info,
            chain: chain,
        }
    }
}

/// See [`VkBufferCopy`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferCopy)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct BufferCopy {
    pub src_offset: u64,
    pub dst_offset: u64,
    pub size: u64,
}

impl<'a> From<&'a BufferCopy> for vks::VkBufferCopy {
    fn from(copy: &'a BufferCopy) -> Self {
        vks::VkBufferCopy {
            srcOffset: copy.src_offset,
            dstOffset: copy.dst_offset,
            size: copy.size,
        }
    }
}

/// See [`VkImageSubresourceLayers`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageSubresourceLayers)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ImageSubresourceLayers {
    pub aspect_mask: ImageAspectFlags,
    pub mip_level: u32,
    pub base_array_layer: u32,
    pub layer_count: u32,
}

impl<'a> From<&'a ImageSubresourceLayers> for vks::VkImageSubresourceLayers {
    fn from(layers: &'a ImageSubresourceLayers) -> Self {
        vks::VkImageSubresourceLayers {
            aspectMask: layers.aspect_mask.bits(),
            mipLevel: layers.mip_level,
            baseArrayLayer: layers.base_array_layer,
            layerCount: layers.layer_count,
        }
    }
}

/// See [`VkImageCopy`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageCopy)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ImageCopy {
    pub src_subresource: ImageSubresourceLayers,
    pub src_offset: Offset3D,
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offset: Offset3D,
    pub extent: Extent3D,
}

impl<'a> From<&'a ImageCopy> for vks::VkImageCopy {
    fn from(copy: &'a ImageCopy) -> Self {
        vks::VkImageCopy {
            srcSubresource: (&copy.src_subresource).into(),
            srcOffset: (&copy.src_offset).into(),
            dstSubresource: (&copy.dst_subresource).into(),
            dstOffset: (&copy.dst_offset).into(),
            extent: (&copy.extent).into(),
        }
    }
}

/// See [`VkImageBlit`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageBlit)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ImageBlit {
    pub src_subresource: ImageSubresourceLayers,
    pub src_offsets: [Offset3D; 2],
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offsets: [Offset3D; 2],
}

impl<'a> From<&'a ImageBlit> for vks::VkImageBlit {
    fn from(blit: &'a ImageBlit) -> Self {
        let src_offsets = [(&blit.src_offsets[0]).into(), (&blit.src_offsets[1]).into()];
        let dst_offsets = [(&blit.dst_offsets[0]).into(), (&blit.dst_offsets[1]).into()];

        vks::VkImageBlit {
            srcSubresource: (&blit.src_subresource).into(),
            srcOffsets: src_offsets,
            dstSubresource: (&blit.dst_subresource).into(),
            dstOffsets: dst_offsets,
        }
    }
}

/// See [`VkBufferImageCopy`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferImageCopy)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct BufferImageCopy {
    pub buffer_offset: u64,
    pub buffer_row_length: u32,
    pub buffer_image_height: u32,
    pub image_subresource: ImageSubresourceLayers,
    pub image_offset: Offset3D,
    pub image_extent: Extent3D,
}

impl<'a> From<&'a BufferImageCopy> for vks::VkBufferImageCopy {
    fn from(copy: &'a BufferImageCopy) -> Self {
        vks::VkBufferImageCopy {
            bufferOffset: copy.buffer_offset,
            bufferRowLength: copy.buffer_row_length,
            bufferImageHeight: copy.buffer_image_height,
            imageSubresource: (&copy.image_subresource).into(),
            imageOffset: (&copy.image_offset).into(),
            imageExtent: (&copy.image_extent).into(),
        }
    }
}

/// See [`VkClearColorValue`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkClearValue)
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ClearColorValue {
    Float32([f32; 4]),
    Int32([i32; 4]),
    UInt32([u32; 4]),
}

impl<'a> From<&'a ClearColorValue> for vks::VkClearColorValue {
    fn from(value: &'a ClearColorValue) -> Self {
        let mut res = vks::VkClearColorValue::default();

        unsafe {
            match *value {
                ClearColorValue::Float32(ref value) => { *res.float32.as_mut() = *value; }
                ClearColorValue::Int32(ref value) => { *res.int32.as_mut() = *value; }
                ClearColorValue::UInt32(ref value) => { *res.uint32.as_mut() = *value; }
            }
        }

        res
    }
}

/// See [`VkClearDepthStencilValue`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkClearDepthStencilValue)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ClearDepthStencilValue {
    pub depth: f32,
    pub stencil: u32,
}

impl<'a> From<&'a ClearDepthStencilValue> for vks::VkClearDepthStencilValue {
    fn from(value: &'a ClearDepthStencilValue) -> Self {
        vks::VkClearDepthStencilValue {
            depth: value.depth,
            stencil: value.stencil,
        }
    }
}

/// See [`VkClearValue`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkClearValue)
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ClearValue {
    Color(ClearColorValue),
    DepthStencil(ClearDepthStencilValue),
}

impl<'a> From<&'a ClearValue> for vks::VkClearValue {
    fn from(value: &'a ClearValue) -> Self {
        let mut res = vks::VkClearValue::default();

        unsafe {
            match *value {
                ClearValue::Color(ref value) => { *res.color.as_mut() = value.into(); }
                ClearValue::DepthStencil(ref value) => { *res.depthStencil.as_mut() = value.into(); }
            }
        }

        res
    }
}

/// See [`VkClearAttachment`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkClearAttachment)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ClearAttachment {
    pub aspect_mask: ImageAspectFlags,
    pub color_attachment: AttachmentIndex,
    pub clear_value: ClearValue,
}

impl<'a> From<&'a ClearAttachment> for vks::VkClearAttachment {
    fn from(foobar: &'a ClearAttachment) -> Self {
        vks::VkClearAttachment {
            aspectMask: foobar.aspect_mask.bits(),
            colorAttachment: foobar.color_attachment.into(),
            clearValue: (&foobar.clear_value).into(),
        }
    }
}

/// See [`VkClearRect`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkClearRect)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ClearRect {
    pub rect: Rect2D,
    pub base_array_layer: u32,
    pub layer_count: u32,
}

impl<'a> From<&'a ClearRect> for vks::VkClearRect {
    fn from(rect: &'a ClearRect) -> Self {
        vks::VkClearRect {
            rect: (&rect.rect).into(),
            baseArrayLayer: rect.base_array_layer,
            layerCount: rect.layer_count,
        }
    }
}

/// See [`VkImageResolve`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageResolve)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ImageResolve {
    pub src_subresource: ImageSubresourceLayers,
    pub src_offset: Offset3D,
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offset: Offset3D,
    pub extent: Extent3D,
}

impl<'a> From<&'a ImageResolve> for vks::VkImageResolve {
    fn from(resolve: &'a ImageResolve) -> Self {
        vks::VkImageResolve {
            srcSubresource: (&resolve.src_subresource).into(),
            srcOffset: (&resolve.src_offset).into(),
            dstSubresource: (&resolve.dst_subresource).into(),
            dstOffset: (&resolve.dst_offset).into(),
            extent: (&resolve.extent).into(),
        }
    }
}

gen_chain_struct! {
    name: MemoryBarrierChain [MemoryBarrierChainWrapper],
    query: MemoryBarrierChainQuery [MemoryBarrierChainQueryWrapper],
    vks: VkMemoryBarrier,
    input: true,
    output: false,
}

/// See [`VkMemoryBarrier`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryBarrier)
#[derive(Debug, Clone, PartialEq)]
pub struct MemoryBarrier {
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub chain: Option<MemoryBarrierChain>,
}

#[derive(Debug)]
struct VkMemoryBarrierWrapper {
    pub vks_struct: vks::VkMemoryBarrier,
    chain: Option<MemoryBarrierChainWrapper>,
}

impl VkMemoryBarrierWrapper {
    pub fn new(barrier: &MemoryBarrier, with_chain: bool) -> Self {
        let (pnext, chain) = MemoryBarrierChainWrapper::new_optional(&barrier.chain, with_chain);

        VkMemoryBarrierWrapper {
            vks_struct: vks::VkMemoryBarrier {
                sType: vks::VK_STRUCTURE_TYPE_MEMORY_BARRIER,
                pNext: pnext,
                srcAccessMask: barrier.src_access_mask.bits(),
                dstAccessMask: barrier.dst_access_mask.bits(),
            },
            chain: chain,
        }
    }
}

gen_chain_struct! {
    name: BufferMemoryBarrierChain [BufferMemoryBarrierChainWrapper],
    query: BufferMemoryBarrierChainQuery [BufferMemoryBarrierChainQueryWrapper],
    vks: VkBufferMemoryBarrier,
    input: true,
    output: false,
}

/// See [`VkBufferMemoryBarrier`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferMemoryBarrier)
#[derive(Debug, Clone, PartialEq)]
pub struct BufferMemoryBarrier {
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub src_queue_family_index: QueueFamilyIndex,
    pub dst_queue_family_index: QueueFamilyIndex,
    pub buffer: Buffer,
    pub offset: u64,
    pub size: OptionalDeviceSize,
    pub chain: Option<BufferMemoryBarrierChain>,
}

#[derive(Debug)]
struct VkBufferMemoryBarrierWrapper {
    pub vks_struct: vks::VkBufferMemoryBarrier,
    buffer: Buffer,
    chain: Option<BufferMemoryBarrierChainWrapper>,
}

impl VkBufferMemoryBarrierWrapper {
    pub fn new(barrier: &BufferMemoryBarrier, with_chain: bool) -> Self {
        let (pnext, chain) = BufferMemoryBarrierChainWrapper::new_optional(&barrier.chain, with_chain);

        VkBufferMemoryBarrierWrapper {
            vks_struct: vks::VkBufferMemoryBarrier {
                sType: vks::VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER,
                pNext: pnext,
                srcAccessMask: barrier.src_access_mask.bits(),
                dstAccessMask: barrier.dst_access_mask.bits(),
                srcQueueFamilyIndex: barrier.src_queue_family_index.into(),
                dstQueueFamilyIndex: barrier.dst_queue_family_index.into(),
                buffer: barrier.buffer.handle(),
                offset: barrier.offset,
                size: barrier.size.into(),
            },
            buffer: barrier.buffer.clone(),
            chain: chain,
        }
    }
}

gen_chain_struct! {
    name: ImageMemoryBarrierChain [ImageMemoryBarrierChainWrapper],
    query: ImageMemoryBarrierChainQuery [ImageMemoryBarrierChainQueryWrapper],
    vks: VkImageMemoryBarrier,
    input: true,
    output: false,
}

/// See [`VkImageMemoryBarrier`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageMemoryBarrier)
#[derive(Debug, Clone, PartialEq)]
pub struct ImageMemoryBarrier {
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub old_layout: ImageLayout,
    pub new_layout: ImageLayout,
    pub src_queue_family_index: QueueFamilyIndex,
    pub dst_queue_family_index: QueueFamilyIndex,
    pub image: Image,
    pub subresource_range: ImageSubresourceRange,
    pub chain: Option<ImageMemoryBarrierChain>,
}

#[derive(Debug)]
struct VkImageMemoryBarrierWrapper {
    pub vks_struct: vks::VkImageMemoryBarrier,
    image: Image,
    chain: Option<ImageMemoryBarrierChainWrapper>,
}

impl VkImageMemoryBarrierWrapper {
    pub fn new(barrier: &ImageMemoryBarrier, with_chain: bool) -> Self {
        let (pnext, chain) = ImageMemoryBarrierChainWrapper::new_optional(&barrier.chain, with_chain);

        VkImageMemoryBarrierWrapper {
            vks_struct: vks::VkImageMemoryBarrier {
                sType: vks::VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER,
                pNext: pnext,
                srcAccessMask: barrier.src_access_mask.bits(),
                dstAccessMask: barrier.dst_access_mask.bits(),
                oldLayout: barrier.old_layout.into(),
                newLayout: barrier.new_layout.into(),
                srcQueueFamilyIndex: barrier.src_queue_family_index.into(),
                dstQueueFamilyIndex: barrier.dst_queue_family_index.into(),
                image: barrier.image.handle(),
                subresourceRange: (&barrier.subresource_range).into(),
            },
            image: barrier.image.clone(),
            chain: chain,
        }
    }
}

gen_chain_struct! {
    name: RenderPassBeginInfoChain [RenderPassBeginInfoChainWrapper],
    query: RenderPassBeginInfoChainQuery [RenderPassBeginInfoChainQueryWrapper],
    vks: VkRenderPassBeginInfo,
    input: true,
    output: false,
}

/// See [`VkRenderPassBeginInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkRenderPassBeginInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct RenderPassBeginInfo {
    pub render_pass: RenderPass,
    pub framebuffer: Framebuffer,
    pub render_area: Rect2D,
    pub clear_values: Option<Vec<ClearValue>>,
    pub chain: Option<RenderPassBeginInfoChain>,
}

#[derive(Debug)]
struct VkRenderPassBeginInfoWrapper {
    pub vks_struct: vks::VkRenderPassBeginInfo,
    render_pass: RenderPass,
    framebuffer: Framebuffer,
    clear_values: Option<Vec<vks::VkClearValue>>,
    chain: Option<RenderPassBeginInfoChainWrapper>,
}

impl VkRenderPassBeginInfoWrapper {
    pub fn new(begin_info: &RenderPassBeginInfo, with_chain: bool) -> Self {
        let (clear_values_count, clear_values_ptr, clear_values) = match begin_info.clear_values {
            Some(ref clear_values) => {
                let clear_values: Vec<_> = clear_values.iter().map(From::from).collect();
                (clear_values.len() as u32, clear_values.as_ptr(), Some(clear_values))
            }

            None => (0, ptr::null(), None),
        };

        let (pnext, chain) = RenderPassBeginInfoChainWrapper::new_optional(&begin_info.chain, with_chain);

        VkRenderPassBeginInfoWrapper {
            vks_struct: vks::VkRenderPassBeginInfo {
                sType: vks::VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,
                pNext: pnext,
                renderPass: begin_info.render_pass.handle(),
                framebuffer: begin_info.framebuffer.handle(),
                renderArea: (&begin_info.render_area).into(),
                clearValueCount: clear_values_count,
                pClearValues: clear_values_ptr,
            },
            render_pass: begin_info.render_pass.clone(),
            framebuffer: begin_info.framebuffer.clone(),
            clear_values: clear_values,
            chain: chain,
        }
    }
}

/// See [`VkDispatchIndirectCommand`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDispatchIndirectCommand)
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DispatchIndirectCommand {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

/// See [`VkDrawIndexedIndirectCommand`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDrawIndexedIndirectCommand)
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DrawIndexedIndirectCommand {
    pub index_count: u32,
    pub instance_count: u32,
    pub first_index: u32,
    pub vertex_offset: i32,
    pub first_instance: u32,
}

/// See [`VkDrawIndirectCommand`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDrawIndirectCommand)
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DrawIndirectCommand {
    pub vertex_count: u32,
    pub instance_count: u32,
    pub first_vertex: u32,
    pub first_instance: u32,
}
