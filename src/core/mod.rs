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
use std::ffi::{CStr, CString};
use std::fmt;
use std::mem;
use std::ops::Deref;
use std::ptr;
use std::slice;
use utils;
use vks;

pub use self::buffer::Buffer;
pub use self::buffer_view::BufferView;
pub use self::command_buffer::CommandBuffer;
pub use self::command_pool::CommandPool;
pub use self::descriptor_pool::DescriptorPool;
pub use self::descriptor_set::DescriptorSet;
pub use self::descriptor_set_layout::DescriptorSetLayout;
pub use self::device::Device;
pub use self::device_memory::{DeviceMemory, MappedMemory};
pub use self::event::Event;
pub use self::fence::Fence;
pub use self::framebuffer::Framebuffer;
pub use self::image::Image;
pub use self::image_view::ImageView;
pub use self::instance::Instance;
pub use self::physical_device::PhysicalDevice;
pub use self::pipeline::Pipeline;
pub use self::pipeline_cache::PipelineCache;
pub use self::pipeline_layout::PipelineLayout;
pub use self::query_pool::QueryPool;
pub use self::queue::Queue;
pub use self::render_pass::RenderPass;
pub use self::sampler::Sampler;
pub use self::semaphore::Semaphore;
pub use self::shader_module::ShaderModule;

/// See [`VkInstanceCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkInstanceCreateFlags)
pub type InstanceCreateFlags = vks::VkInstanceCreateFlags;

/// See [`VkInstanceCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkInstanceCreateFlags)
pub type InstanceCreateFlagBits = vks::VkInstanceCreateFlagBits;

/// See [`VkFormatFeatureFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlagBits)
pub type FormatFeatureFlags = vks::VkFormatFeatureFlags;

/// See [`VkFormatFeatureFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlagBits)
pub type FormatFeatureFlagBits = vks::VkFormatFeatureFlagBits;

/// See [`VkFormatFeatureFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlagBits)
pub const FORMAT_FEATURE_SAMPLED_IMAGE_BIT: FormatFeatureFlagBits = vks::VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT;

/// See [`VkFormatFeatureFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlagBits)
pub const FORMAT_FEATURE_STORAGE_IMAGE_BIT: FormatFeatureFlagBits = vks::VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT;

/// See [`VkFormatFeatureFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlagBits)
pub const FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT: FormatFeatureFlagBits = vks::VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT;

/// See [`VkFormatFeatureFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlagBits)
pub const FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT: FormatFeatureFlagBits = vks::VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT;

/// See [`VkFormatFeatureFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlagBits)
pub const FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT: FormatFeatureFlagBits = vks::VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT;

/// See [`VkFormatFeatureFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlagBits)
pub const FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_ATOMIC_BIT: FormatFeatureFlagBits = vks::VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_ATOMIC_BIT;

/// See [`VkFormatFeatureFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlagBits)
pub const FORMAT_FEATURE_VERTEX_BUFFER_BIT: FormatFeatureFlagBits = vks::VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT;

/// See [`VkFormatFeatureFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlagBits)
pub const FORMAT_FEATURE_COLOR_ATTACHMENT_BIT: FormatFeatureFlagBits = vks::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT;

/// See [`VkFormatFeatureFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlagBits)
pub const FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT: FormatFeatureFlagBits = vks::VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT;

/// See [`VkFormatFeatureFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlagBits)
pub const FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT: FormatFeatureFlagBits = vks::VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT;

/// See [`VkFormatFeatureFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlagBits)
pub const FORMAT_FEATURE_BLIT_SRC_BIT: FormatFeatureFlagBits = vks::VK_FORMAT_FEATURE_BLIT_SRC_BIT;

/// See [`VkFormatFeatureFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlagBits)
pub const FORMAT_FEATURE_BLIT_DST_BIT: FormatFeatureFlagBits = vks::VK_FORMAT_FEATURE_BLIT_DST_BIT;

/// See [`VkFormatFeatureFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFormatFeatureFlagBits)
pub const FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT: FormatFeatureFlagBits = vks::VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT;

/// See [`VkImageUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageUsageFlagBits)
pub type ImageUsageFlags = vks::VkImageUsageFlags;

/// See [`VkImageUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageUsageFlagBits)
pub type ImageUsageFlagBits = vks::VkImageUsageFlagBits;

/// See [`VkImageUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageUsageFlagBits)
pub const IMAGE_USAGE_TRANSFER_SRC_BIT: ImageUsageFlagBits = vks::VK_IMAGE_USAGE_TRANSFER_SRC_BIT;

/// See [`VkImageUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageUsageFlagBits)
pub const IMAGE_USAGE_TRANSFER_DST_BIT: ImageUsageFlagBits = vks::VK_IMAGE_USAGE_TRANSFER_DST_BIT;

/// See [`VkImageUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageUsageFlagBits)
pub const IMAGE_USAGE_SAMPLED_BIT: ImageUsageFlagBits = vks::VK_IMAGE_USAGE_SAMPLED_BIT;

/// See [`VkImageUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageUsageFlagBits)
pub const IMAGE_USAGE_STORAGE_BIT: ImageUsageFlagBits = vks::VK_IMAGE_USAGE_STORAGE_BIT;

/// See [`VkImageUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageUsageFlagBits)
pub const IMAGE_USAGE_COLOR_ATTACHMENT_BIT: ImageUsageFlagBits = vks::VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT;

/// See [`VkImageUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageUsageFlagBits)
pub const IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT: ImageUsageFlagBits = vks::VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT;

/// See [`VkImageUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageUsageFlagBits)
pub const IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT: ImageUsageFlagBits = vks::VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT;

/// See [`VkImageUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageUsageFlagBits)
pub const IMAGE_USAGE_INPUT_ATTACHMENT_BIT: ImageUsageFlagBits = vks::VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT;

/// See [`VkImageCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageCreateFlagBits)
pub type ImageCreateFlags = vks::VkImageCreateFlags;

/// See [`VkImageCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageCreateFlagBits)
pub type ImageCreateFlagBits = vks::VkImageCreateFlagBits;

/// See [`VkImageCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageCreateFlagBits)
pub const IMAGE_CREATE_SPARSE_BINDING_BIT: ImageCreateFlagBits = vks::VK_IMAGE_CREATE_SPARSE_BINDING_BIT;

/// See [`VkImageCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageCreateFlagBits)
pub const IMAGE_CREATE_SPARSE_RESIDENCY_BIT: ImageCreateFlagBits = vks::VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT;

/// See [`VkImageCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageCreateFlagBits)
pub const IMAGE_CREATE_SPARSE_ALIASED_BIT: ImageCreateFlagBits = vks::VK_IMAGE_CREATE_SPARSE_ALIASED_BIT;

/// See [`VkImageCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageCreateFlagBits)
pub const IMAGE_CREATE_MUTABLE_FORMAT_BIT: ImageCreateFlagBits = vks::VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT;

/// See [`VkImageCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageCreateFlagBits)
pub const IMAGE_CREATE_CUBE_COMPATIBLE_BIT: ImageCreateFlagBits = vks::VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT;

/// See [`VkSampleCountFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSampleCountFlagBits)
pub type SampleCountFlags = vks::VkSampleCountFlags;

/// See [`VkSampleCountFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSampleCountFlagBits)
pub type SampleCountFlagBits = vks::VkSampleCountFlagBits;

/// See [`VkSampleCountFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSampleCountFlagBits)
pub const SAMPLE_COUNT_1_BIT: SampleCountFlagBits = vks::VK_SAMPLE_COUNT_1_BIT;

/// See [`VkSampleCountFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSampleCountFlagBits)
pub const SAMPLE_COUNT_2_BIT: SampleCountFlagBits = vks::VK_SAMPLE_COUNT_2_BIT;

/// See [`VkSampleCountFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSampleCountFlagBits)
pub const SAMPLE_COUNT_4_BIT: SampleCountFlagBits = vks::VK_SAMPLE_COUNT_4_BIT;

/// See [`VkSampleCountFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSampleCountFlagBits)
pub const SAMPLE_COUNT_8_BIT: SampleCountFlagBits = vks::VK_SAMPLE_COUNT_8_BIT;

/// See [`VkSampleCountFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSampleCountFlagBits)
pub const SAMPLE_COUNT_16_BIT: SampleCountFlagBits = vks::VK_SAMPLE_COUNT_16_BIT;

/// See [`VkSampleCountFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSampleCountFlagBits)
pub const SAMPLE_COUNT_32_BIT: SampleCountFlagBits = vks::VK_SAMPLE_COUNT_32_BIT;

/// See [`VkSampleCountFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSampleCountFlagBits)
pub const SAMPLE_COUNT_64_BIT: SampleCountFlagBits = vks::VK_SAMPLE_COUNT_64_BIT;

/// See [`VkQueueFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueueFlagBits)
pub type QueueFlags = vks::VkQueueFlags;

/// See [`VkQueueFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueueFlagBits)
pub type QueueFlagBits = vks::VkQueueFlagBits;

/// See [`VkQueueFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueueFlagBits)
pub const QUEUE_GRAPHICS_BIT: QueueFlagBits = vks::VK_QUEUE_GRAPHICS_BIT;

/// See [`VkQueueFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueueFlagBits)
pub const QUEUE_COMPUTE_BIT: QueueFlagBits = vks::VK_QUEUE_COMPUTE_BIT;

/// See [`VkQueueFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueueFlagBits)
pub const QUEUE_TRANSFER_BIT: QueueFlagBits = vks::VK_QUEUE_TRANSFER_BIT;

/// See [`VkQueueFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueueFlagBits)
pub const QUEUE_SPARSE_BINDING_BIT: QueueFlagBits = vks::VK_QUEUE_SPARSE_BINDING_BIT;

/// See [`VkMemoryPropertyFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryPropertyFlagBits)
pub type MemoryPropertyFlags = vks::VkMemoryPropertyFlags;

/// See [`VkMemoryPropertyFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryPropertyFlagBits)
pub type MemoryPropertyFlagBits = vks::VkMemoryPropertyFlagBits;

/// See [`VkMemoryPropertyFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryPropertyFlagBits)
pub const MEMORY_PROPERTY_DEVICE_LOCAL_BIT: MemoryPropertyFlagBits = vks::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT;

/// See [`VkMemoryPropertyFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryPropertyFlagBits)
pub const MEMORY_PROPERTY_HOST_VISIBLE_BIT: MemoryPropertyFlagBits = vks::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT;

/// See [`VkMemoryPropertyFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryPropertyFlagBits)
pub const MEMORY_PROPERTY_HOST_COHERENT_BIT: MemoryPropertyFlagBits = vks::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT;

/// See [`VkMemoryPropertyFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryPropertyFlagBits)
pub const MEMORY_PROPERTY_HOST_CACHED_BIT: MemoryPropertyFlagBits = vks::VK_MEMORY_PROPERTY_HOST_CACHED_BIT;

/// See [`VkMemoryPropertyFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryPropertyFlagBits)
pub const MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT: MemoryPropertyFlagBits = vks::VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT;

/// See [`VkMemoryHeapFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryHeapFlagBits)
pub type MemoryHeapFlags = vks::VkMemoryHeapFlags;

/// See [`VkMemoryHeapFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryHeapFlagBits)
pub type MemoryHeapFlagBits = vks::VkMemoryHeapFlagBits;

/// See [`VkMemoryHeapFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryHeapFlagBits)
pub const MEMORY_HEAP_DEVICE_LOCAL_BIT: MemoryHeapFlagBits = vks::VK_MEMORY_HEAP_DEVICE_LOCAL_BIT;

/// See [`VkDeviceCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceCreateFlags)
pub type DeviceCreateFlags = vks::VkDeviceCreateFlags;

/// See [`VkDeviceCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceCreateFlags)
pub type DeviceCreateFlagBits = vks::VkDeviceCreateFlagBits;

/// See [`VkDeviceQueueCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceQueueCreateFlags)
pub type DeviceQueueCreateFlags = vks::VkDeviceQueueCreateFlags;

/// See [`VkDeviceQueueCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceQueueCreateFlags)
pub type DeviceQueueCreateFlagBits = vks::VkDeviceQueueCreateFlagBits;

/// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
pub type PipelineStageFlags = vks::VkPipelineStageFlags;

/// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
pub type PipelineStageFlagBits = vks::VkPipelineStageFlagBits;

/// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
pub const PIPELINE_STAGE_TOP_OF_PIPE_BIT: PipelineStageFlagBits = vks::VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT;

/// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
pub const PIPELINE_STAGE_DRAW_INDIRECT_BIT: PipelineStageFlagBits = vks::VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT;

/// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
pub const PIPELINE_STAGE_VERTEX_INPUT_BIT: PipelineStageFlagBits = vks::VK_PIPELINE_STAGE_VERTEX_INPUT_BIT;

/// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
pub const PIPELINE_STAGE_VERTEX_SHADER_BIT: PipelineStageFlagBits = vks::VK_PIPELINE_STAGE_VERTEX_SHADER_BIT;

/// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
pub const PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT: PipelineStageFlagBits = vks::VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT;

/// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
pub const PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT: PipelineStageFlagBits = vks::VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT;

/// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
pub const PIPELINE_STAGE_GEOMETRY_SHADER_BIT: PipelineStageFlagBits = vks::VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT;

/// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
pub const PIPELINE_STAGE_FRAGMENT_SHADER_BIT: PipelineStageFlagBits = vks::VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT;

/// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
pub const PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT: PipelineStageFlagBits = vks::VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT;

/// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
pub const PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT: PipelineStageFlagBits = vks::VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT;

/// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
pub const PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT: PipelineStageFlagBits = vks::VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT;

/// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
pub const PIPELINE_STAGE_COMPUTE_SHADER_BIT: PipelineStageFlagBits = vks::VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT;

/// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
pub const PIPELINE_STAGE_TRANSFER_BIT: PipelineStageFlagBits = vks::VK_PIPELINE_STAGE_TRANSFER_BIT;

/// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
pub const PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT: PipelineStageFlagBits = vks::VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT;

/// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
pub const PIPELINE_STAGE_HOST_BIT: PipelineStageFlagBits = vks::VK_PIPELINE_STAGE_HOST_BIT;

/// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
pub const PIPELINE_STAGE_ALL_GRAPHICS_BIT: PipelineStageFlagBits = vks::VK_PIPELINE_STAGE_ALL_GRAPHICS_BIT;

/// See [`VkPipelineStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineStageFlagBits)
pub const PIPELINE_STAGE_ALL_COMMANDS_BIT: PipelineStageFlagBits = vks::VK_PIPELINE_STAGE_ALL_COMMANDS_BIT;

/// See [`VkMemoryMapFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryMapFlags)
pub type MemoryMapFlags = vks::VkMemoryMapFlags;

/// See [`VkMemoryMapFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryMapFlags)
pub type MemoryMapFlagBits = vks::VkMemoryMapFlagBits;

/// See [`VkImageAspectFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageAspectFlagBits)
pub type ImageAspectFlags = vks::VkImageAspectFlags;

/// See [`VkImageAspectFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageAspectFlagBits)
pub type ImageAspectFlagBits = vks::VkImageAspectFlagBits;

/// See [`VkImageAspectFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageAspectFlagBits)
pub const IMAGE_ASPECT_COLOR_BIT: ImageAspectFlagBits = vks::VK_IMAGE_ASPECT_COLOR_BIT;

/// See [`VkImageAspectFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageAspectFlagBits)
pub const IMAGE_ASPECT_DEPTH_BIT: ImageAspectFlagBits = vks::VK_IMAGE_ASPECT_DEPTH_BIT;

/// See [`VkImageAspectFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageAspectFlagBits)
pub const IMAGE_ASPECT_STENCIL_BIT: ImageAspectFlagBits = vks::VK_IMAGE_ASPECT_STENCIL_BIT;

/// See [`VkImageAspectFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageAspectFlagBits)
pub const IMAGE_ASPECT_METADATA_BIT: ImageAspectFlagBits = vks::VK_IMAGE_ASPECT_METADATA_BIT;

/// See [`VkSparseImageFormatFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseImageFormatFlagBits)
pub type SparseImageFormatFlags = vks::VkSparseImageFormatFlags;

/// See [`VkSparseImageFormatFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseImageFormatFlagBits)
pub type SparseImageFormatFlagBits = vks::VkSparseImageFormatFlagBits;

/// See [`VkSparseImageFormatFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseImageFormatFlagBits)
pub const SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT: SparseImageFormatFlagBits = vks::VK_SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT;

/// See [`VkSparseImageFormatFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseImageFormatFlagBits)
pub const SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT: SparseImageFormatFlagBits = vks::VK_SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT;

/// See [`VkSparseImageFormatFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseImageFormatFlagBits)
pub const SPARSE_IMAGE_FORMAT_NONSTANDARD_BLOCK_SIZE_BIT: SparseImageFormatFlagBits = vks::VK_SPARSE_IMAGE_FORMAT_NONSTANDARD_BLOCK_SIZE_BIT;

/// See [`VkSparseMemoryBindFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseMemoryBindFlagBits)
pub type SparseMemoryBindFlags = vks::VkSparseMemoryBindFlags;

/// See [`VkSparseMemoryBindFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseMemoryBindFlagBits)
pub type SparseMemoryBindFlagBits = vks::VkSparseMemoryBindFlagBits;

/// See [`VkSparseMemoryBindFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSparseMemoryBindFlagBits)
pub const SPARSE_MEMORY_BIND_METADATA_BIT: SparseMemoryBindFlagBits = vks::VK_SPARSE_MEMORY_BIND_METADATA_BIT;

/// See [`VkFenceCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFenceCreateFlagBits)
pub type FenceCreateFlags = vks::VkFenceCreateFlags;

/// See [`VkFenceCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFenceCreateFlagBits)
pub type FenceCreateFlagBits = vks::VkFenceCreateFlagBits;

/// See [`VkFenceCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFenceCreateFlagBits)
pub const FENCE_CREATE_SIGNALED_BIT: FenceCreateFlagBits = vks::VK_FENCE_CREATE_SIGNALED_BIT;

/// See [`VkSemaphoreCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSemaphoreCreateFlags)
pub type SemaphoreCreateFlags = vks::VkSemaphoreCreateFlags;

/// See [`VkSemaphoreCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSemaphoreCreateFlags)
pub type SemaphoreCreateFlagBits = vks::VkSemaphoreCreateFlagBits;

/// See [`VkEventCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkEventCreateFlags)
pub type EventCreateFlags = vks::VkEventCreateFlags;

/// See [`VkEventCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkEventCreateFlags)
pub type EventCreateFlagBits = vks::VkEventCreateFlagBits;

/// See [`VkQueryPoolCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPoolCreateFlags)
pub type QueryPoolCreateFlags = vks::VkQueryPoolCreateFlags;

/// See [`VkQueryPoolCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPoolCreateFlags)
pub type QueryPoolCreateFlagBits = vks::VkQueryPoolCreateFlagBits;

/// See [`VkQueryPipelineStatisticFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlagBits)
pub type QueryPipelineStatisticFlags = vks::VkQueryPipelineStatisticFlags;

/// See [`VkQueryPipelineStatisticFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlagBits)
pub type QueryPipelineStatisticFlagBits = vks::VkQueryPipelineStatisticFlagBits;

/// See [`VkQueryPipelineStatisticFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlagBits)
pub const QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_VERTICES_BIT: QueryPipelineStatisticFlagBits = vks::VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_VERTICES_BIT;

/// See [`VkQueryPipelineStatisticFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlagBits)
pub const QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_PRIMITIVES_BIT: QueryPipelineStatisticFlagBits = vks::VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_PRIMITIVES_BIT;

/// See [`VkQueryPipelineStatisticFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlagBits)
pub const QUERY_PIPELINE_STATISTIC_VERTEX_SHADER_INVOCATIONS_BIT: QueryPipelineStatisticFlagBits = vks::VK_QUERY_PIPELINE_STATISTIC_VERTEX_SHADER_INVOCATIONS_BIT;

/// See [`VkQueryPipelineStatisticFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlagBits)
pub const QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_INVOCATIONS_BIT: QueryPipelineStatisticFlagBits = vks::VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_INVOCATIONS_BIT;

/// See [`VkQueryPipelineStatisticFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlagBits)
pub const QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_PRIMITIVES_BIT: QueryPipelineStatisticFlagBits = vks::VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_PRIMITIVES_BIT;

/// See [`VkQueryPipelineStatisticFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlagBits)
pub const QUERY_PIPELINE_STATISTIC_CLIPPING_INVOCATIONS_BIT: QueryPipelineStatisticFlagBits = vks::VK_QUERY_PIPELINE_STATISTIC_CLIPPING_INVOCATIONS_BIT;

/// See [`VkQueryPipelineStatisticFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlagBits)
pub const QUERY_PIPELINE_STATISTIC_CLIPPING_PRIMITIVES_BIT: QueryPipelineStatisticFlagBits = vks::VK_QUERY_PIPELINE_STATISTIC_CLIPPING_PRIMITIVES_BIT;

/// See [`VkQueryPipelineStatisticFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlagBits)
pub const QUERY_PIPELINE_STATISTIC_FRAGMENT_SHADER_INVOCATIONS_BIT: QueryPipelineStatisticFlagBits = vks::VK_QUERY_PIPELINE_STATISTIC_FRAGMENT_SHADER_INVOCATIONS_BIT;

/// See [`VkQueryPipelineStatisticFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlagBits)
pub const QUERY_PIPELINE_STATISTIC_TESSELLATION_CONTROL_SHADER_PATCHES_BIT: QueryPipelineStatisticFlagBits = vks::VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_CONTROL_SHADER_PATCHES_BIT;

/// See [`VkQueryPipelineStatisticFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlagBits)
pub const QUERY_PIPELINE_STATISTIC_TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT: QueryPipelineStatisticFlagBits = vks::VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT;

/// See [`VkQueryPipelineStatisticFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPipelineStatisticFlagBits)
pub const QUERY_PIPELINE_STATISTIC_COMPUTE_SHADER_INVOCATIONS_BIT: QueryPipelineStatisticFlagBits = vks::VK_QUERY_PIPELINE_STATISTIC_COMPUTE_SHADER_INVOCATIONS_BIT;

/// See [`VkQueryResultFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryResultFlagBits)
pub type QueryResultFlags = vks::VkQueryResultFlags;

/// See [`VkQueryResultFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryResultFlagBits)
pub type QueryResultFlagBits = vks::VkQueryResultFlagBits;

/// See [`VkQueryResultFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryResultFlagBits)
pub const QUERY_RESULT_64_BIT: QueryResultFlagBits = vks::VK_QUERY_RESULT_64_BIT;

/// See [`VkQueryResultFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryResultFlagBits)
pub const QUERY_RESULT_WAIT_BIT: QueryResultFlagBits = vks::VK_QUERY_RESULT_WAIT_BIT;

/// See [`VkQueryResultFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryResultFlagBits)
pub const QUERY_RESULT_WITH_AVAILABILITY_BIT: QueryResultFlagBits = vks::VK_QUERY_RESULT_WITH_AVAILABILITY_BIT;

/// See [`VkQueryResultFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryResultFlagBits)
pub const QUERY_RESULT_PARTIAL_BIT: QueryResultFlagBits = vks::VK_QUERY_RESULT_PARTIAL_BIT;

/// See [`VkBufferCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferCreateFlagBits)
pub type BufferCreateFlags = vks::VkBufferCreateFlags;

/// See [`VkBufferCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferCreateFlagBits)
pub type BufferCreateFlagBits = vks::VkBufferCreateFlagBits;

/// See [`VkBufferCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferCreateFlagBits)
pub const BUFFER_CREATE_SPARSE_BINDING_BIT: BufferCreateFlagBits = vks::VK_BUFFER_CREATE_SPARSE_BINDING_BIT;

/// See [`VkBufferCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferCreateFlagBits)
pub const BUFFER_CREATE_SPARSE_RESIDENCY_BIT: BufferCreateFlagBits = vks::VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT;

/// See [`VkBufferCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferCreateFlagBits)
pub const BUFFER_CREATE_SPARSE_ALIASED_BIT: BufferCreateFlagBits = vks::VK_BUFFER_CREATE_SPARSE_ALIASED_BIT;

/// See [`VkBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferUsageFlagBits)
pub type BufferUsageFlags = vks::VkBufferUsageFlags;

/// See [`VkBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferUsageFlagBits)
pub type BufferUsageFlagBits = vks::VkBufferUsageFlagBits;

/// See [`VkBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferUsageFlagBits)
pub const BUFFER_USAGE_TRANSFER_SRC_BIT: BufferUsageFlagBits = vks::VK_BUFFER_USAGE_TRANSFER_SRC_BIT;

/// See [`VkBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferUsageFlagBits)
pub const BUFFER_USAGE_TRANSFER_DST_BIT: BufferUsageFlagBits = vks::VK_BUFFER_USAGE_TRANSFER_DST_BIT;

/// See [`VkBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferUsageFlagBits)
pub const BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT: BufferUsageFlagBits = vks::VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT;

/// See [`VkBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferUsageFlagBits)
pub const BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT: BufferUsageFlagBits = vks::VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT;

/// See [`VkBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferUsageFlagBits)
pub const BUFFER_USAGE_UNIFORM_BUFFER_BIT: BufferUsageFlagBits = vks::VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT;

/// See [`VkBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferUsageFlagBits)
pub const BUFFER_USAGE_STORAGE_BUFFER_BIT: BufferUsageFlagBits = vks::VK_BUFFER_USAGE_STORAGE_BUFFER_BIT;

/// See [`VkBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferUsageFlagBits)
pub const BUFFER_USAGE_INDEX_BUFFER_BIT: BufferUsageFlagBits = vks::VK_BUFFER_USAGE_INDEX_BUFFER_BIT;

/// See [`VkBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferUsageFlagBits)
pub const BUFFER_USAGE_VERTEX_BUFFER_BIT: BufferUsageFlagBits = vks::VK_BUFFER_USAGE_VERTEX_BUFFER_BIT;

/// See [`VkBufferViewCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferViewCreateFlags)
pub type BufferViewCreateFlags = vks::VkBufferViewCreateFlags;

/// See [`VkBufferViewCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferViewCreateFlags)
pub type BufferViewCreateFlagBits = vks::VkBufferViewCreateFlagBits;

/// See [`VkImageViewCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageViewCreateFlags)
pub type ImageViewCreateFlags = vks::VkImageViewCreateFlags;

/// See [`VkImageViewCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageViewCreateFlags)
pub type ImageViewCreateFlagBits = vks::VkImageViewCreateFlagBits;

/// See [`VkShaderModuleCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderModuleCreateFlags)
pub type ShaderModuleCreateFlags = vks::VkShaderModuleCreateFlags;

/// See [`VkShaderModuleCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderModuleCreateFlags)
pub type ShaderModuleCreateFlagBits = vks::VkShaderModuleCreateFlagBits;

/// See [`VkPipelineCacheCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCacheCreateFlags)
pub type PipelineCacheCreateFlags = vks::VkPipelineCacheCreateFlags;

/// See [`VkPipelineCacheCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCacheCreateFlags)
pub type PipelineCacheCreateFlagBits = vks::VkPipelineCacheCreateFlagBits;

/// See [`VkPipelineCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCreateFlagBits)
pub type PipelineCreateFlags = vks::VkPipelineCreateFlags;

/// See [`VkPipelineCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCreateFlagBits)
pub type PipelineCreateFlagBits = vks::VkPipelineCreateFlagBits;

/// See [`VkPipelineCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCreateFlagBits)
pub const PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT: PipelineCreateFlagBits = vks::VK_PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT;

/// See [`VkPipelineCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCreateFlagBits)
pub const PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT: PipelineCreateFlagBits = vks::VK_PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT;

/// See [`VkPipelineCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCreateFlagBits)
pub const PIPELINE_CREATE_DERIVATIVE_BIT: PipelineCreateFlagBits = vks::VK_PIPELINE_CREATE_DERIVATIVE_BIT;

/// See [`VkPipelineShaderStageCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineShaderStageCreateFlags)
pub type PipelineShaderStageCreateFlags = vks::VkPipelineShaderStageCreateFlags;

/// See [`VkPipelineShaderStageCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineShaderStageCreateFlags)
pub type PipelineShaderStageCreateFlagBits = vks::VkPipelineShaderStageCreateFlagBits;

/// See [`VkShaderStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderStageFlagBits)
pub type ShaderStageFlags = vks::VkShaderStageFlags;

/// See [`VkShaderStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderStageFlagBits)
pub type ShaderStageFlagBits = vks::VkShaderStageFlagBits;

/// See [`VkShaderStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderStageFlagBits)
pub const SHADER_STAGE_VERTEX_BIT: ShaderStageFlagBits = vks::VK_SHADER_STAGE_VERTEX_BIT;

/// See [`VkShaderStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderStageFlagBits)
pub const SHADER_STAGE_TESSELLATION_CONTROL_BIT: ShaderStageFlagBits = vks::VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT;

/// See [`VkShaderStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderStageFlagBits)
pub const SHADER_STAGE_TESSELLATION_EVALUATION_BIT: ShaderStageFlagBits = vks::VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT;

/// See [`VkShaderStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderStageFlagBits)
pub const SHADER_STAGE_GEOMETRY_BIT: ShaderStageFlagBits = vks::VK_SHADER_STAGE_GEOMETRY_BIT;

/// See [`VkShaderStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderStageFlagBits)
pub const SHADER_STAGE_FRAGMENT_BIT: ShaderStageFlagBits = vks::VK_SHADER_STAGE_FRAGMENT_BIT;

/// See [`VkShaderStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderStageFlagBits)
pub const SHADER_STAGE_COMPUTE_BIT: ShaderStageFlagBits = vks::VK_SHADER_STAGE_COMPUTE_BIT;

/// See [`VkShaderStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderStageFlagBits)
pub const SHADER_STAGE_ALL_GRAPHICS: ShaderStageFlagBits = vks::VK_SHADER_STAGE_ALL_GRAPHICS;

/// See [`VkShaderStageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderStageFlagBits)
pub const SHADER_STAGE_ALL: ShaderStageFlagBits = vks::VK_SHADER_STAGE_ALL;

/// See [`VkPipelineVertexInputStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineVertexInputStateCreateFlags)
pub type PipelineVertexInputStateCreateFlags = vks::VkPipelineVertexInputStateCreateFlags;

/// See [`VkPipelineVertexInputStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineVertexInputStateCreateFlags)
pub type PipelineVertexInputStateCreateFlagBits = vks::VkPipelineVertexInputStateCreateFlagBits;

/// See [`VkPipelineInputAssemblyStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineInputAssemblyStateCreateFlags)
pub type PipelineInputAssemblyStateCreateFlags = vks::VkPipelineInputAssemblyStateCreateFlags;

/// See [`VkPipelineInputAssemblyStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineInputAssemblyStateCreateFlags)
pub type PipelineInputAssemblyStateCreateFlagBits = vks::VkPipelineInputAssemblyStateCreateFlagBits;

/// See [`VkPipelineTessellationStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineTessellationStateCreateFlags)
pub type PipelineTessellationStateCreateFlags = vks::VkPipelineTessellationStateCreateFlags;

/// See [`VkPipelineTessellationStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineTessellationStateCreateFlags)
pub type PipelineTessellationStateCreateFlagBits = vks::VkPipelineTessellationStateCreateFlagBits;

/// See [`VkPipelineViewportStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineViewportStateCreateFlags)
pub type PipelineViewportStateCreateFlags = vks::VkPipelineViewportStateCreateFlags;

/// See [`VkPipelineViewportStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineViewportStateCreateFlags)
pub type PipelineViewportStateCreateFlagBits = vks::VkPipelineViewportStateCreateFlagBits;

/// See [`VkPipelineRasterizationStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineRasterizationStateCreateFlags)
pub type PipelineRasterizationStateCreateFlags = vks::VkPipelineRasterizationStateCreateFlags;

/// See [`VkPipelineRasterizationStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineRasterizationStateCreateFlags)
pub type PipelineRasterizationStateCreateFlagBits = vks::VkPipelineRasterizationStateCreateFlagBits;

/// See [`VkCullModeFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCullModeFlagBits)
pub type CullModeFlags = vks::VkCullModeFlags;

/// See [`VkCullModeFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCullModeFlagBits)
pub type CullModeFlagBits = vks::VkCullModeFlagBits;

/// See [`VkCullModeFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCullModeFlagBits)
pub const CULL_MODE_NONE: CullModeFlagBits = vks::VK_CULL_MODE_NONE;

/// See [`VkCullModeFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCullModeFlagBits)
pub const CULL_MODE_FRONT_BIT: CullModeFlagBits = vks::VK_CULL_MODE_FRONT_BIT;

/// See [`VkCullModeFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCullModeFlagBits)
pub const CULL_MODE_BACK_BIT: CullModeFlagBits = vks::VK_CULL_MODE_BACK_BIT;

/// See [`VkCullModeFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCullModeFlagBits)
pub const CULL_MODE_FRONT_AND_BACK: CullModeFlagBits = vks::VK_CULL_MODE_FRONT_AND_BACK;

/// See [`VkPipelineMultisampleStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineMultisampleStateCreateFlags)
pub type PipelineMultisampleStateCreateFlags = vks::VkPipelineMultisampleStateCreateFlags;

/// See [`VkPipelineMultisampleStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineMultisampleStateCreateFlags)
pub type PipelineMultisampleStateCreateFlagBits = vks::VkPipelineMultisampleStateCreateFlagBits;

/// See [`VkPipelineDepthStencilStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineDepthStencilStateCreateFlags)
pub type PipelineDepthStencilStateCreateFlags = vks::VkPipelineDepthStencilStateCreateFlags;

/// See [`VkPipelineDepthStencilStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineDepthStencilStateCreateFlags)
pub type PipelineDepthStencilStateCreateFlagBits = vks::VkPipelineDepthStencilStateCreateFlagBits;

/// See [`VkPipelineColorBlendStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineColorBlendStateCreateFlags)
pub type PipelineColorBlendStateCreateFlags = vks::VkPipelineColorBlendStateCreateFlags;

/// See [`VkPipelineColorBlendStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineColorBlendStateCreateFlags)
pub type PipelineColorBlendStateCreateFlagBits = vks::VkPipelineColorBlendStateCreateFlagBits;

/// See [`VkPipelineColorBlendStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineColorBlendStateCreateFlags)
pub const PIPELINE_COLOR_BLEND_STATE_CREATE_DUMMY: PipelineColorBlendStateCreateFlagBits = vks::VK_PIPELINE_COLOR_BLEND_STATE_CREATE_DUMMY;

/// See [`VkColorComponentFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorComponentFlagBits)
pub type ColorComponentFlags = vks::VkColorComponentFlags;

/// See [`VkColorComponentFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorComponentFlagBits)
pub type ColorComponentFlagBits = vks::VkColorComponentFlagBits;

/// See [`VkColorComponentFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorComponentFlagBits)
pub const COLOR_COMPONENT_R_BIT: ColorComponentFlagBits = vks::VK_COLOR_COMPONENT_R_BIT;

/// See [`VkColorComponentFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorComponentFlagBits)
pub const COLOR_COMPONENT_G_BIT: ColorComponentFlagBits = vks::VK_COLOR_COMPONENT_G_BIT;

/// See [`VkColorComponentFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorComponentFlagBits)
pub const COLOR_COMPONENT_B_BIT: ColorComponentFlagBits = vks::VK_COLOR_COMPONENT_B_BIT;

/// See [`VkColorComponentFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkColorComponentFlagBits)
pub const COLOR_COMPONENT_A_BIT: ColorComponentFlagBits = vks::VK_COLOR_COMPONENT_A_BIT;

/// See [`VkPipelineDynamicStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineDynamicStateCreateFlags)
pub type PipelineDynamicStateCreateFlags = vks::VkPipelineDynamicStateCreateFlags;

/// See [`VkPipelineDynamicStateCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineDynamicStateCreateFlags)
pub type PipelineDynamicStateCreateFlagBits = vks::VkPipelineDynamicStateCreateFlagBits;

/// See [`VkPipelineLayoutCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineLayoutCreateFlags)
pub type PipelineLayoutCreateFlags = vks::VkPipelineLayoutCreateFlags;

/// See [`VkPipelineLayoutCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineLayoutCreateFlags)
pub type PipelineLayoutCreateFlagBits = vks::VkPipelineLayoutCreateFlagBits;

/// See [`VkSamplerCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSamplerCreateFlags)
pub type SamplerCreateFlags = vks::VkSamplerCreateFlags;

/// See [`VkSamplerCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSamplerCreateFlags)
pub type SamplerCreateFlagBits = vks::VkSamplerCreateFlagBits;

/// See [`VkDescriptorSetLayoutCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorSetLayoutCreateFlagBits)
pub type DescriptorSetLayoutCreateFlags = vks::VkDescriptorSetLayoutCreateFlags;

/// See [`VkDescriptorSetLayoutCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorSetLayoutCreateFlagBits)
pub type DescriptorSetLayoutCreateFlagBits = vks::VkDescriptorSetLayoutCreateFlagBits;

/// See [`VkDescriptorPoolCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorPoolCreateFlagBits)
pub type DescriptorPoolCreateFlags = vks::VkDescriptorPoolCreateFlags;

/// See [`VkDescriptorPoolCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorPoolCreateFlagBits)
pub type DescriptorPoolCreateFlagBits = vks::VkDescriptorPoolCreateFlagBits;

/// See [`VkDescriptorPoolCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorPoolCreateFlagBits)
pub const DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT: DescriptorPoolCreateFlagBits = vks::VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT;

/// See [`VkDescriptorPoolResetFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorPoolResetFlags)
pub type DescriptorPoolResetFlags = vks::VkDescriptorPoolResetFlags;

/// See [`VkDescriptorPoolResetFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorPoolResetFlags)
pub type DescriptorPoolResetFlagBits = vks::VkDescriptorPoolResetFlagBits;

/// See [`VkFramebufferCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFramebufferCreateFlags)
pub type FramebufferCreateFlags = vks::VkFramebufferCreateFlags;

/// See [`VkFramebufferCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFramebufferCreateFlags)
pub type FramebufferCreateFlagBits = vks::VkFramebufferCreateFlagBits;

/// See [`VkRenderPassCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkRenderPassCreateFlags)
pub type RenderPassCreateFlags = vks::VkRenderPassCreateFlags;

/// See [`VkRenderPassCreateFlags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkRenderPassCreateFlags)
pub type RenderPassCreateFlagBits = vks::VkRenderPassCreateFlagBits;

/// See [`VkAttachmentDescriptionFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAttachmentDescriptionFlagBits)
pub type AttachmentDescriptionFlags = vks::VkAttachmentDescriptionFlags;

/// See [`VkAttachmentDescriptionFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAttachmentDescriptionFlagBits)
pub type AttachmentDescriptionFlagBits = vks::VkAttachmentDescriptionFlagBits;

/// See [`VkAttachmentDescriptionFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAttachmentDescriptionFlagBits)
pub const ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT: AttachmentDescriptionFlagBits = vks::VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT;

/// See [`VkSubpassDescriptionFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSubpassDescriptionFlagBits)
pub type SubpassDescriptionFlags = vks::VkSubpassDescriptionFlags;

/// See [`VkSubpassDescriptionFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSubpassDescriptionFlagBits)
pub type SubpassDescriptionFlagBits = vks::VkSubpassDescriptionFlagBits;

/// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
pub type AccessFlags = vks::VkAccessFlags;

/// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
pub type AccessFlagBits = vks::VkAccessFlagBits;

/// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
pub const ACCESS_INDIRECT_COMMAND_READ_BIT: AccessFlagBits = vks::VK_ACCESS_INDIRECT_COMMAND_READ_BIT;

/// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
pub const ACCESS_INDEX_READ_BIT: AccessFlagBits = vks::VK_ACCESS_INDEX_READ_BIT;

/// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
pub const ACCESS_VERTEX_ATTRIBUTE_READ_BIT: AccessFlagBits = vks::VK_ACCESS_VERTEX_ATTRIBUTE_READ_BIT;

/// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
pub const ACCESS_UNIFORM_READ_BIT: AccessFlagBits = vks::VK_ACCESS_UNIFORM_READ_BIT;

/// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
pub const ACCESS_INPUT_ATTACHMENT_READ_BIT: AccessFlagBits = vks::VK_ACCESS_INPUT_ATTACHMENT_READ_BIT;

/// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
pub const ACCESS_SHADER_READ_BIT: AccessFlagBits = vks::VK_ACCESS_SHADER_READ_BIT;

/// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
pub const ACCESS_SHADER_WRITE_BIT: AccessFlagBits = vks::VK_ACCESS_SHADER_WRITE_BIT;

/// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
pub const ACCESS_COLOR_ATTACHMENT_READ_BIT: AccessFlagBits = vks::VK_ACCESS_COLOR_ATTACHMENT_READ_BIT;

/// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
pub const ACCESS_COLOR_ATTACHMENT_WRITE_BIT: AccessFlagBits = vks::VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT;

/// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
pub const ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT: AccessFlagBits = vks::VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT;

/// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
pub const ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT: AccessFlagBits = vks::VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT;

/// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
pub const ACCESS_TRANSFER_READ_BIT: AccessFlagBits = vks::VK_ACCESS_TRANSFER_READ_BIT;

/// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
pub const ACCESS_TRANSFER_WRITE_BIT: AccessFlagBits = vks::VK_ACCESS_TRANSFER_WRITE_BIT;

/// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
pub const ACCESS_HOST_READ_BIT: AccessFlagBits = vks::VK_ACCESS_HOST_READ_BIT;

/// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
pub const ACCESS_HOST_WRITE_BIT: AccessFlagBits = vks::VK_ACCESS_HOST_WRITE_BIT;

/// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
pub const ACCESS_MEMORY_READ_BIT: AccessFlagBits = vks::VK_ACCESS_MEMORY_READ_BIT;

/// See [`VkAccessFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAccessFlagBits)
pub const ACCESS_MEMORY_WRITE_BIT: AccessFlagBits = vks::VK_ACCESS_MEMORY_WRITE_BIT;

/// See [`VkDependencyFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDependencyFlagBits)
pub type DependencyFlags = vks::VkDependencyFlags;

/// See [`VkDependencyFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDependencyFlagBits)
pub type DependencyFlagBits = vks::VkDependencyFlagBits;

/// See [`VkDependencyFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDependencyFlagBits)
pub const DEPENDENCY_BY_REGION_BIT: DependencyFlagBits = vks::VK_DEPENDENCY_BY_REGION_BIT;

/// See [`VkCommandPoolCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandPoolCreateFlagBits)
pub type CommandPoolCreateFlags = vks::VkCommandPoolCreateFlags;

/// See [`VkCommandPoolCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandPoolCreateFlagBits)
pub type CommandPoolCreateFlagBits = vks::VkCommandPoolCreateFlagBits;

/// See [`VkCommandPoolCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandPoolCreateFlagBits)
pub const COMMAND_POOL_CREATE_TRANSIENT_BIT: CommandPoolCreateFlagBits = vks::VK_COMMAND_POOL_CREATE_TRANSIENT_BIT;

/// See [`VkCommandPoolCreateFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandPoolCreateFlagBits)
pub const COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT: CommandPoolCreateFlagBits = vks::VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT;

/// See [`VkCommandPoolResetFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandPoolResetFlagBits)
pub type CommandPoolResetFlags = vks::VkCommandPoolResetFlags;

/// See [`VkCommandPoolResetFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandPoolResetFlagBits)
pub type CommandPoolResetFlagBits = vks::VkCommandPoolResetFlagBits;

/// See [`VkCommandPoolResetFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandPoolResetFlagBits)
pub const COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT: CommandPoolResetFlagBits = vks::VK_COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT;

/// See [`VkCommandBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferUsageFlagBits)
pub type CommandBufferUsageFlags = vks::VkCommandBufferUsageFlags;

/// See [`VkCommandBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferUsageFlagBits)
pub type CommandBufferUsageFlagBits = vks::VkCommandBufferUsageFlagBits;

/// See [`VkCommandBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferUsageFlagBits)
pub const COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT: CommandBufferUsageFlagBits = vks::VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT;

/// See [`VkCommandBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferUsageFlagBits)
pub const COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT: CommandBufferUsageFlagBits = vks::VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT;

/// See [`VkCommandBufferUsageFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferUsageFlagBits)
pub const COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT: CommandBufferUsageFlagBits = vks::VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT;

/// See [`VkQueryControlFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryControlFlagBits)
pub type QueryControlFlags = vks::VkQueryControlFlags;

/// See [`VkQueryControlFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryControlFlagBits)
pub type QueryControlFlagBits = vks::VkQueryControlFlagBits;

/// See [`VkQueryControlFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryControlFlagBits)
pub const QUERY_CONTROL_PRECISE_BIT: QueryControlFlagBits = vks::VK_QUERY_CONTROL_PRECISE_BIT;

/// See [`VkCommandBufferResetFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferResetFlagBits)
pub type CommandBufferResetFlags = vks::VkCommandBufferResetFlags;

/// See [`VkCommandBufferResetFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferResetFlagBits)
pub type CommandBufferResetFlagBits = vks::VkCommandBufferResetFlagBits;

/// See [`VkCommandBufferResetFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferResetFlagBits)
pub const COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT: CommandBufferResetFlagBits = vks::VK_COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT;

/// See [`VkStencilFaceFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStencilFaceFlagBits)
pub type StencilFaceFlags = vks::VkStencilFaceFlags;

/// See [`VkStencilFaceFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStencilFaceFlagBits)
pub type StencilFaceFlagBits = vks::VkStencilFaceFlagBits;

/// See [`VkStencilFaceFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStencilFaceFlagBits)
pub const STENCIL_FACE_FRONT_BIT: StencilFaceFlagBits = vks::VK_STENCIL_FACE_FRONT_BIT;

/// See [`VkStencilFaceFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStencilFaceFlagBits)
pub const STENCIL_FACE_BACK_BIT: StencilFaceFlagBits = vks::VK_STENCIL_FACE_BACK_BIT;

/// See [`VkStencilFaceFlagBits`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkStencilFaceFlagBits)
pub const STENCIL_FRONT_AND_BACK: StencilFaceFlagBits = vks::VK_STENCIL_FRONT_AND_BACK;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum OptionalDeviceSize {
    Size(u64),
    WholeSize,
}

impl From<u64> for OptionalDeviceSize {
    fn from(size: u64) -> Self {
        if size != vks::VK_WHOLE_SIZE {
            OptionalDeviceSize::Size(size)
        }
        else {
            OptionalDeviceSize::WholeSize
        }
    }
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

impl From<u32> for OptionalMipLevels {
    fn from(mip_levels: u32) -> Self {
        if mip_levels != vks::VK_REMAINING_MIP_LEVELS {
            OptionalMipLevels::MipLevels(mip_levels)
        }
        else {
            OptionalMipLevels::Remaining
        }
    }
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

impl From<u32> for OptionalArrayLayers {
    fn from(array_layers: u32) -> Self {
        if array_layers != vks::VK_REMAINING_ARRAY_LAYERS {
            OptionalArrayLayers::ArrayLayers(array_layers)
        }
        else {
            OptionalArrayLayers::Remaining
        }
    }
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

impl From<u32> for AttachmentIndex {
    fn from(index: u32) -> Self {
        if index != vks::VK_ATTACHMENT_UNUSED {
            AttachmentIndex::Index(index)
        }
        else {
            AttachmentIndex::Unused
        }
    }
}

impl From<AttachmentIndex> for u32 {
    fn from(index: AttachmentIndex) -> Self {
        match index {
            AttachmentIndex::Index(index) => index,
            AttachmentIndex::Unused => vks::VK_ATTACHMENT_UNUSED
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum QueueFamilyIndex {
    Index(u32),
    Ignored,
}

impl From<u32> for QueueFamilyIndex {
    fn from(index: u32) -> Self {
        if index != vks::VK_QUEUE_FAMILY_IGNORED {
            QueueFamilyIndex::Index(index)
        }
        else {
            QueueFamilyIndex::Ignored
        }
    }
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

impl From<u32> for SubpassIndex {
    fn from(index: u32) -> Self {
        if index != vks::VK_SUBPASS_EXTERNAL {
            SubpassIndex::Index(index)
        }
        else {
            SubpassIndex::External
        }
    }
}

impl From<SubpassIndex> for u32 {
    fn from(index: SubpassIndex) -> Self {
        match index {
            SubpassIndex::Index(index) => index,
            SubpassIndex::External => vks::VK_SUBPASS_EXTERNAL,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum QueryResult {
    U32(u32),
    U64(u64),
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

impl From<vks::VkPipelineCacheHeaderVersion> for PipelineCacheHeaderVersion {
    fn from(version: vks::VkPipelineCacheHeaderVersion) -> Self {
        match version {
            vks::VK_PIPELINE_CACHE_HEADER_VERSION_ONE => PipelineCacheHeaderVersion::One,
            _ => PipelineCacheHeaderVersion::Unknown(version),
        }
    }
}

impl From<PipelineCacheHeaderVersion> for vks::VkPipelineCacheHeaderVersion {
    fn from(version: PipelineCacheHeaderVersion) -> Self {
        match version {
            PipelineCacheHeaderVersion::One => vks::VK_PIPELINE_CACHE_HEADER_VERSION_ONE,
            PipelineCacheHeaderVersion::Unknown(version) => version,
        }
    }
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

    #[cfg(feature = "khr_surface_25")]
    /// See extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
    SurfaceLostKHR,

    #[cfg(feature = "khr_surface_25")]
    /// See extension [`VK_KHR_surface`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_surface)
    NativeWindowInUseKHR,

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

            #[cfg(feature = "khr_surface_25")]
            Error::SurfaceLostKHR => "SurfaceLost",

            #[cfg(feature = "khr_surface_25")]
            Error::NativeWindowInUseKHR => "NativeWindowInUse",

            Error::Unknown(_) => "unknown error",
        }
    }
}

impl From<vks::VkResult> for Error {
    fn from(res: vks::VkResult) -> Self {
        debug_assert!(res.as_raw() < 0);

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

            #[cfg(feature = "khr_surface_25")]
            vks::VK_ERROR_SURFACE_LOST_KHR => Error::SurfaceLostKHR,

            #[cfg(feature = "khr_surface_25")]
            vks::VK_ERROR_NATIVE_WINDOW_IN_USE_KHR => Error::NativeWindowInUseKHR,

            _ => Error::Unknown(res),
        }
    }
}

impl From<Error> for vks::VkResult {
    fn from(err: Error) -> Self {
        match err {
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

            #[cfg(feature = "khr_surface_25")]
            Error::SurfaceLostKHR => vks::VK_ERROR_SURFACE_LOST_KHR,

            #[cfg(feature = "khr_surface_25")]
            Error::NativeWindowInUseKHR => vks::VK_ERROR_NATIVE_WINDOW_IN_USE_KHR,

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

impl From<SystemAllocationSope> for vks::VkSystemAllocationScope {
    fn from(scope: SystemAllocationSope) -> vks::VkSystemAllocationScope {
        match scope {
            SystemAllocationSope::Command => vks::VK_SYSTEM_ALLOCATION_SCOPE_COMMAND,
            SystemAllocationSope::Object => vks::VK_SYSTEM_ALLOCATION_SCOPE_OBJECT,
            SystemAllocationSope::Cache => vks::VK_SYSTEM_ALLOCATION_SCOPE_CACHE,
            SystemAllocationSope::Device => vks::VK_SYSTEM_ALLOCATION_SCOPE_DEVICE,
            SystemAllocationSope::Instance => vks::VK_SYSTEM_ALLOCATION_SCOPE_INSTANCE,
            SystemAllocationSope::Unknown(scope) => scope,
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

impl From<InternalAllocationType> for vks::VkInternalAllocationType {
    fn from(allocation_type: InternalAllocationType) -> vks::VkInternalAllocationType {
        match allocation_type {
            InternalAllocationType::Executable => vks::VK_INTERNAL_ALLOCATION_TYPE_EXECUTABLE,
            InternalAllocationType::Unknown(allocation_type) => allocation_type,
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

impl From<vks::VkImageType> for ImageType {
    fn from(image_type: vks::VkImageType) -> Self {
        match image_type {
            vks::VK_IMAGE_TYPE_1D => ImageType::Type1D,
            vks::VK_IMAGE_TYPE_2D => ImageType::Type2D,
            vks::VK_IMAGE_TYPE_3D => ImageType::Type3D,
            _ => ImageType::Unknown(image_type),
        }
    }
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

impl From<vks::VkImageTiling> for ImageTiling {
    fn from(tiling: vks::VkImageTiling) -> Self {
        match tiling {
            vks::VK_IMAGE_TILING_OPTIMAL => ImageTiling::Optimal,
            vks::VK_IMAGE_TILING_LINEAR => ImageTiling::Linear,
            _ => ImageTiling::Unknown(tiling),
        }
    }
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

impl From<vks::VkQueryType> for QueryType {
    fn from(query_type: vks::VkQueryType) -> Self {
        match query_type {
            vks::VK_QUERY_TYPE_OCCLUSION => QueryType::Occlusion,
            vks::VK_QUERY_TYPE_PIPELINE_STATISTICS => QueryType::PipelineStatistics,
            vks::VK_QUERY_TYPE_TIMESTAMP => QueryType::Timestamp,
            _ => QueryType::Unknown(query_type),
        }
    }
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

impl From<vks::VkSharingMode> for SharingMode {
    fn from(mode: vks::VkSharingMode) -> Self {
        match mode {
            vks::VK_SHARING_MODE_EXCLUSIVE => SharingMode::Exclusive,
            vks::VK_SHARING_MODE_CONCURRENT => SharingMode::Concurrent,
            _ => SharingMode::Unknown(mode),
        }
    }
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
    Unknown(vks::VkImageLayout),
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
            _ => ImageLayout::Unknown(layout),
        }
    }
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
            ImageLayout::Unknown(layout) => layout,
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

impl From<vks::VkImageViewType> for ImageViewType {
    fn from(view_type: vks::VkImageViewType) -> Self {
        match view_type {
            vks::VK_IMAGE_VIEW_TYPE_1D => ImageViewType::Type1D,
            vks::VK_IMAGE_VIEW_TYPE_2D => ImageViewType::Type2D,
            vks::VK_IMAGE_VIEW_TYPE_3D => ImageViewType::Type3D,
            vks::VK_IMAGE_VIEW_TYPE_CUBE => ImageViewType::TypeCube,
            vks::VK_IMAGE_VIEW_TYPE_1D_ARRAY => ImageViewType::Type1DArray,
            vks::VK_IMAGE_VIEW_TYPE_2D_ARRAY => ImageViewType::Type2DArray,
            vks::VK_IMAGE_VIEW_TYPE_CUBE_ARRAY => ImageViewType::TypeCubeArray,
            _ => ImageViewType::Unknown(view_type),
        }
    }
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

impl From<vks::VkComponentSwizzle> for ComponentSwizzle {
    fn from(swizzle: vks::VkComponentSwizzle) -> Self {
        match swizzle {
            vks::VK_COMPONENT_SWIZZLE_IDENTITY => ComponentSwizzle::Identity,
            vks::VK_COMPONENT_SWIZZLE_ZERO => ComponentSwizzle::Zero,
            vks::VK_COMPONENT_SWIZZLE_ONE => ComponentSwizzle::One,
            vks::VK_COMPONENT_SWIZZLE_R => ComponentSwizzle::R,
            vks::VK_COMPONENT_SWIZZLE_G => ComponentSwizzle::G,
            vks::VK_COMPONENT_SWIZZLE_B => ComponentSwizzle::B,
            vks::VK_COMPONENT_SWIZZLE_A => ComponentSwizzle::A,
            _ => ComponentSwizzle::Unknown(swizzle),
        }
    }
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

impl From<vks::VkVertexInputRate> for VertexInputRate {
    fn from(rate: vks::VkVertexInputRate) -> Self {
        match rate {
            vks::VK_VERTEX_INPUT_RATE_VERTEX => VertexInputRate::Vertex,
            vks::VK_VERTEX_INPUT_RATE_INSTANCE => VertexInputRate::Instance,
            _ => VertexInputRate::Unknown(rate),
        }
    }
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

impl From<vks::VkPrimitiveTopology> for PrimitiveTopology {
    fn from(topology: vks::VkPrimitiveTopology) -> Self {
        match topology {
            vks::VK_PRIMITIVE_TOPOLOGY_POINT_LIST => PrimitiveTopology::PointList,
            vks::VK_PRIMITIVE_TOPOLOGY_LINE_LIST => PrimitiveTopology::LineList,
            vks::VK_PRIMITIVE_TOPOLOGY_LINE_STRIP => PrimitiveTopology::LineStrip,
            vks::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST => PrimitiveTopology::TriangleList,
            vks::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP => PrimitiveTopology::TriangleStrip,
            vks::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_FAN => PrimitiveTopology::TriangleFan,
            vks::VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY => PrimitiveTopology::LineListWithAdjacency,
            vks::VK_PRIMITIVE_TOPOLOGY_LINE_STRIP_WITH_ADJACENCY => PrimitiveTopology::LineStripWithAdjacency,
            vks::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY => PrimitiveTopology::TriangleListWithAdjacency,
            vks::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_WITH_ADJACENCY => PrimitiveTopology::TriangleStripWithAdjacency,
            vks::VK_PRIMITIVE_TOPOLOGY_PATCH_LIST => PrimitiveTopology::PatchList,
            _ => PrimitiveTopology::Unknown(topology),
        }
    }
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

impl From<vks::VkPolygonMode> for PolygonMode {
    fn from(mode: vks::VkPolygonMode) -> Self {
        match mode {
            vks::VK_POLYGON_MODE_FILL => PolygonMode::Fill,
            vks::VK_POLYGON_MODE_LINE => PolygonMode::Line,
            vks::VK_POLYGON_MODE_POINT => PolygonMode::Point,
            _ => PolygonMode::Unknown(mode),
        }
    }
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

impl From<vks::VkFrontFace> for FrontFace {
    fn from(face: vks::VkFrontFace) -> Self {
        match face {
            vks::VK_FRONT_FACE_COUNTER_CLOCKWISE => FrontFace::CounterClockwise,
            vks::VK_FRONT_FACE_CLOCKWISE => FrontFace::Clockwise,
            _ => FrontFace::Unknown(face),
        }
    }
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

impl From<vks::VkCompareOp> for CompareOp {
    fn from(op: vks::VkCompareOp) -> Self {
        match op {
            vks::VK_COMPARE_OP_NEVER => CompareOp::Never,
            vks::VK_COMPARE_OP_LESS => CompareOp::Less,
            vks::VK_COMPARE_OP_EQUAL => CompareOp::Equal,
            vks::VK_COMPARE_OP_LESS_OR_EQUAL => CompareOp::LessOrEqual,
            vks::VK_COMPARE_OP_GREATER => CompareOp::Greater,
            vks::VK_COMPARE_OP_NOT_EQUAL => CompareOp::NotEqual,
            vks::VK_COMPARE_OP_GREATER_OR_EQUAL => CompareOp::GreaterOrEqual,
            vks::VK_COMPARE_OP_ALWAYS => CompareOp::Always,
            _ => CompareOp::Unknown(op),
        }
    }
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

impl From<vks::VkStencilOp> for StencilOp {
    fn from(op: vks::VkStencilOp) -> Self {
        match op {
            vks::VK_STENCIL_OP_KEEP => StencilOp::Keep,
            vks::VK_STENCIL_OP_ZERO => StencilOp::Zero,
            vks::VK_STENCIL_OP_REPLACE => StencilOp::Replace,
            vks::VK_STENCIL_OP_INCREMENT_AND_CLAMP => StencilOp::IncrementAndClamp,
            vks::VK_STENCIL_OP_DECREMENT_AND_CLAMP => StencilOp::DecrementAndClamp,
            vks::VK_STENCIL_OP_INVERT => StencilOp::Invert,
            vks::VK_STENCIL_OP_INCREMENT_AND_WRAP => StencilOp::IncrementAndWrap,
            vks::VK_STENCIL_OP_DECREMENT_AND_WRAP => StencilOp::DecrementAndWrap,
            _ => StencilOp::Unknown(op),
        }
    }
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

impl From<vks::VkLogicOp> for LogicOp {
    fn from(op: vks::VkLogicOp) -> Self {
        match op {
            vks::VK_LOGIC_OP_CLEAR => LogicOp::Clear,
            vks::VK_LOGIC_OP_AND => LogicOp::And,
            vks::VK_LOGIC_OP_AND_REVERSE => LogicOp::AndReverse,
            vks::VK_LOGIC_OP_COPY => LogicOp::Copy,
            vks::VK_LOGIC_OP_AND_INVERTED => LogicOp::AndInverted,
            vks::VK_LOGIC_OP_NO_OP => LogicOp::NoOp,
            vks::VK_LOGIC_OP_XOR => LogicOp::Xor,
            vks::VK_LOGIC_OP_OR => LogicOp::Or,
            vks::VK_LOGIC_OP_NOR => LogicOp::Nor,
            vks::VK_LOGIC_OP_EQUIVALENT => LogicOp::Equivalent,
            vks::VK_LOGIC_OP_INVERT => LogicOp::Invert,
            vks::VK_LOGIC_OP_OR_REVERSE => LogicOp::OrReverse,
            vks::VK_LOGIC_OP_COPY_INVERTED => LogicOp::CopyInverted,
            vks::VK_LOGIC_OP_OR_INVERTED => LogicOp::OrInverted,
            vks::VK_LOGIC_OP_NAND => LogicOp::Nand,
            vks::VK_LOGIC_OP_SET => LogicOp::Set,
            _ => LogicOp::Unknown(op),
        }
    }
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

impl From<vks::VkBlendFactor> for BlendFactor {
    fn from(factor: vks::VkBlendFactor) -> Self {
        match factor {
            vks::VK_BLEND_FACTOR_ZERO => BlendFactor::Zero,
            vks::VK_BLEND_FACTOR_ONE => BlendFactor::One,
            vks::VK_BLEND_FACTOR_SRC_COLOR => BlendFactor::SrcColor,
            vks::VK_BLEND_FACTOR_ONE_MINUS_SRC_COLOR => BlendFactor::OneMinusSrcColor,
            vks::VK_BLEND_FACTOR_DST_COLOR => BlendFactor::DstColor,
            vks::VK_BLEND_FACTOR_ONE_MINUS_DST_COLOR => BlendFactor::OneMinusDstColor,
            vks::VK_BLEND_FACTOR_SRC_ALPHA => BlendFactor::SrcAlpha,
            vks::VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA => BlendFactor::OneMinusSrcAlpha,
            vks::VK_BLEND_FACTOR_DST_ALPHA => BlendFactor::DstAlpha,
            vks::VK_BLEND_FACTOR_ONE_MINUS_DST_ALPHA => BlendFactor::OneMinusDstAlpha,
            vks::VK_BLEND_FACTOR_CONSTANT_COLOR => BlendFactor::ConstantColor,
            vks::VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR => BlendFactor::OneMinusConstantColor,
            vks::VK_BLEND_FACTOR_CONSTANT_ALPHA => BlendFactor::ConstantAlpha,
            vks::VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA => BlendFactor::OneMinusConstantAlpha,
            vks::VK_BLEND_FACTOR_SRC_ALPHA_SATURATE => BlendFactor::SrcAlphaSaturate,
            vks::VK_BLEND_FACTOR_SRC1_COLOR => BlendFactor::Src1Color,
            vks::VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR => BlendFactor::OneMinusSrc1Color,
            vks::VK_BLEND_FACTOR_SRC1_ALPHA => BlendFactor::Src1Alpha,
            vks::VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA => BlendFactor::OneMinusSrc1Alpha,
            _ => BlendFactor::Unknown(factor),
        }
    }
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

impl From<vks::VkBlendOp> for BlendOp {
    fn from(op: vks::VkBlendOp) -> Self {
        match op {
            vks::VK_BLEND_OP_ADD => BlendOp::Add,
            vks::VK_BLEND_OP_SUBTRACT => BlendOp::Subtract,
            vks::VK_BLEND_OP_REVERSE_SUBTRACT => BlendOp::ReverseSubtract,
            vks::VK_BLEND_OP_MIN => BlendOp::Min,
            vks::VK_BLEND_OP_MAX => BlendOp::Max,
            _ => BlendOp::Unknown(op),
        }
    }
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

impl From<vks::VkDynamicState> for DynamicState {
    fn from(state: vks::VkDynamicState) -> Self {
        match state {
            vks::VK_DYNAMIC_STATE_VIEWPORT => DynamicState::Viewport,
            vks::VK_DYNAMIC_STATE_SCISSOR => DynamicState::Scissor,
            vks::VK_DYNAMIC_STATE_LINE_WIDTH => DynamicState::LineWidth,
            vks::VK_DYNAMIC_STATE_DEPTH_BIAS => DynamicState::DepthBias,
            vks::VK_DYNAMIC_STATE_BLEND_CONSTANTS => DynamicState::BlendConstants,
            vks::VK_DYNAMIC_STATE_DEPTH_BOUNDS => DynamicState::DepthBounds,
            vks::VK_DYNAMIC_STATE_STENCIL_COMPARE_MASK => DynamicState::StencilCompareMask,
            vks::VK_DYNAMIC_STATE_STENCIL_WRITE_MASK => DynamicState::StencilWriteMask,
            vks::VK_DYNAMIC_STATE_STENCIL_REFERENCE => DynamicState::StencilReference,
            _ => DynamicState::Unknown(state),
        }
    }
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
    Unknown(vks::VkFilter),
}

impl From<vks::VkFilter> for Filter {
    fn from(filter: vks::VkFilter) -> Self {
        match filter {
            vks::VK_FILTER_NEAREST => Filter::Nearest,
            vks::VK_FILTER_LINEAR => Filter::Linear,
            _ => Filter::Unknown(filter),
        }
    }
}

impl From<Filter> for vks::VkFilter {
    fn from(filter: Filter) -> Self {
        match filter {
            Filter::Nearest => vks::VK_FILTER_NEAREST,
            Filter::Linear => vks::VK_FILTER_LINEAR,
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

impl From<vks::VkSamplerMipmapMode> for SamplerMipmapMode {
    fn from(mode: vks::VkSamplerMipmapMode) -> Self {
        match mode {
            vks::VK_SAMPLER_MIPMAP_MODE_NEAREST => SamplerMipmapMode::Nearest,
            vks::VK_SAMPLER_MIPMAP_MODE_LINEAR => SamplerMipmapMode::Linear,
            _ => SamplerMipmapMode::Unknown(mode),
        }
    }
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

impl From<vks::VkSamplerAddressMode> for SamplerAddressMode {
    fn from(mode: vks::VkSamplerAddressMode) -> Self {
        match mode {
            vks::VK_SAMPLER_ADDRESS_MODE_REPEAT => SamplerAddressMode::Repeat,
            vks::VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT => SamplerAddressMode::MirroredRepeat,
            vks::VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE => SamplerAddressMode::ClampToEdge,
            vks::VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER => SamplerAddressMode::ClampToBorder,
            vks::VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE => SamplerAddressMode::MirrorClampToEdge,
            _ => SamplerAddressMode::Unknown(mode),
        }
    }
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

impl From<vks::VkBorderColor> for BorderColor {
    fn from(color: vks::VkBorderColor) -> Self {
        match color {
            vks::VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK => BorderColor::FloatTransparentBlack,
            vks::VK_BORDER_COLOR_INT_TRANSPARENT_BLACK => BorderColor::IntTransparentBlack,
            vks::VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK => BorderColor::FloatOpaqueBlack,
            vks::VK_BORDER_COLOR_INT_OPAQUE_BLACK => BorderColor::IntOpaqueBlack,
            vks::VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE => BorderColor::FloatOpaqueWhite,
            vks::VK_BORDER_COLOR_INT_OPAQUE_WHITE => BorderColor::IntOpaqueWhite,
            _ => BorderColor::Unknown(color),
        }
    }
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

impl From<vks::VkDescriptorType> for DescriptorType {
    fn from(descriptor_type: vks::VkDescriptorType) -> Self {
        match descriptor_type {
            vks::VK_DESCRIPTOR_TYPE_SAMPLER => DescriptorType::Sampler,
            vks::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER => DescriptorType::CombinedImageSampler,
            vks::VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE => DescriptorType::SampledImage,
            vks::VK_DESCRIPTOR_TYPE_STORAGE_IMAGE => DescriptorType::StorageImage,
            vks::VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER => DescriptorType::UniformTexelBuffer,
            vks::VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER => DescriptorType::StorageTexelBuffer,
            vks::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER => DescriptorType::UniformBuffer,
            vks::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER => DescriptorType::StorageBuffer,
            vks::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC => DescriptorType::UniformBufferDynamic,
            vks::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC => DescriptorType::StorageBufferDynamic,
            vks::VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT => DescriptorType::InputAttachment,
            _ => DescriptorType::Unknown(descriptor_type),
        }
    }
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

/// See [`VkAttachmentStoreOp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAttachmentStoreOp)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AttachmentStoreOp {
    Store,
    DontCare,
    Unknown(vks::VkAttachmentStoreOp),
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

impl From<AttachmentStoreOp> for vks::VkAttachmentStoreOp {
    fn from(op: AttachmentStoreOp) -> Self {
        match op {
            AttachmentStoreOp::Store => vks::VK_ATTACHMENT_STORE_OP_STORE,
            AttachmentStoreOp::DontCare => vks::VK_ATTACHMENT_STORE_OP_DONT_CARE,
            AttachmentStoreOp::Unknown(op) => op,
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

impl From<vks::VkPipelineBindPoint> for PipelineBindPoint {
    fn from(bind_point: vks::VkPipelineBindPoint) -> Self {
        match bind_point {
            vks::VK_PIPELINE_BIND_POINT_GRAPHICS => PipelineBindPoint::Graphics,
            vks::VK_PIPELINE_BIND_POINT_COMPUTE => PipelineBindPoint::Compute,
            _ => PipelineBindPoint::Unknown(bind_point),
        }
    }
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

/// See [`VkCommandBufferLevel`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferLevel)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CommandBufferLevel {
    Primary,
    Secondary,
    Unknown(vks::VkCommandBufferLevel),
}

impl From<vks::VkCommandBufferLevel> for CommandBufferLevel {
    fn from(level: vks::VkCommandBufferLevel) -> Self {
        match level {
            vks::VK_COMMAND_BUFFER_LEVEL_PRIMARY => CommandBufferLevel::Primary,
            vks::VK_COMMAND_BUFFER_LEVEL_SECONDARY => CommandBufferLevel::Secondary,
            _ => CommandBufferLevel::Unknown(level),
        }
    }
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

impl From<vks::VkIndexType> for IndexType {
    fn from(index_type: vks::VkIndexType) -> Self {
        match index_type {
            vks::VK_INDEX_TYPE_UINT16 => IndexType::UInt16,
            vks::VK_INDEX_TYPE_UINT32 => IndexType::UInt32,
            _ => IndexType::Unknown(index_type),
        }
    }
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

impl From<vks::VkSubpassContents> for SubpassContents {
    fn from(contents: vks::VkSubpassContents) -> Self {
        match contents {
            vks::VK_SUBPASS_CONTENTS_INLINE => SubpassContents::Inline,
            vks::VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS => SubpassContents::SecondaryCommandBuffers,
            _ => SubpassContents::Unknown(contents),
        }
    }
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

/// See [`VkApplicationInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkApplicationInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum ApplicationInfoChainElement {
}

/// See [`VkApplicationInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkApplicationInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct ApplicationInfo {
    pub chain: Vec<ApplicationInfoChainElement>,
    pub application_name: Option<String>,
    pub application_version: u32,
    pub engine_name: Option<String>,
    pub engine_version: u32,
    pub api_version: Option<Version>,
}

impl<'a> From<&'a vks::VkApplicationInfo> for ApplicationInfo {
    fn from(info: &'a vks::VkApplicationInfo) -> Self {
        debug_assert_eq!(info.pNext, ptr::null());

        ApplicationInfo {
            chain: vec![],
            application_name: utils::string_from_cstr(info.pApplicationName),
            application_version: info.applicationVersion,
            engine_name: utils::string_from_cstr(info.pEngineName),
            engine_version: info.engineVersion,
            api_version: Version::from_optional_api_version(info.apiVersion),
        }
    }
}

#[derive(Debug)]
struct VkApplicationInfoWrapper {
    application_info: vks::VkApplicationInfo,
    application_name_cstr: Option<CString>,
    engine_name_cstr: Option<CString>,
}

impl Deref for VkApplicationInfoWrapper {
    type Target = vks::VkApplicationInfo;

    fn deref(&self) -> &Self::Target {
        &self.application_info
    }
}

impl AsRef<vks::VkApplicationInfo> for VkApplicationInfoWrapper {
    fn as_ref(&self) -> &vks::VkApplicationInfo {
        &self.application_info
    }
}

impl<'a> From<&'a ApplicationInfo> for VkApplicationInfoWrapper {
    fn from(info: &'a ApplicationInfo) -> Self {
        let application_name_cstr = utils::cstr_from_string(info.application_name.clone());
        let engine_name_cstr = utils::cstr_from_string(info.engine_name.clone());

        VkApplicationInfoWrapper {
            application_info: vks::VkApplicationInfo {
                sType: vks::VK_STRUCTURE_TYPE_APPLICATION_INFO,
                pNext: ptr::null(),
                pApplicationName: application_name_cstr.1,
                applicationVersion: info.application_version,
                pEngineName: engine_name_cstr.1,
                engineVersion: info.engine_version,
                apiVersion: Version::api_version_from_optional(info.api_version),
            },
            application_name_cstr: application_name_cstr.0,
            engine_name_cstr: engine_name_cstr.0,
        }
    }
}

/// See [`VkInstanceCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkInstanceCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum InstanceCreateInfoChainElement {
}

/// See [`VkInstanceCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkInstanceCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct InstanceCreateInfo {
    pub chain: Vec<InstanceCreateInfoChainElement>,
    pub flags: InstanceCreateFlags,
    pub application_info: Option<ApplicationInfo>,
    pub enabled_layers: Vec<String>,
    pub enabled_extensions: Vec<InstanceExtension>,
}

impl<'a> From<&'a vks::VkInstanceCreateInfo> for InstanceCreateInfo {
    fn from(create_info: &'a vks::VkInstanceCreateInfo) -> Self {
        debug_assert_eq!(create_info.pNext, ptr::null());

        let application_info = if !create_info.pApplicationInfo.is_null() {
            unsafe {
                Some((&*create_info.pApplicationInfo).into())
            }
        }
        else {
            None
        };

        let enabled_layers_slice = unsafe {
            slice::from_raw_parts(create_info.ppEnabledLayerNames, create_info.enabledLayerCount as usize)
        };
        let enabled_layers = enabled_layers_slice
            .iter()
            .map(|&e| unsafe { CStr::from_ptr(e).to_str().unwrap().to_owned() })
            .collect();

        let enabled_extensions_slice = unsafe {
            slice::from_raw_parts(create_info.ppEnabledExtensionNames, create_info.enabledExtensionCount as usize)
        };
        let enabled_extensions = enabled_extensions_slice
            .iter()
            .map(|&e| unsafe { CStr::from_ptr(e).to_str().unwrap() })
            .map(From::from)
            .collect();

        InstanceCreateInfo {
            chain: vec![],
            flags: create_info.flags,
            application_info: application_info,
            enabled_layers: enabled_layers,
            enabled_extensions: enabled_extensions,
        }
    }
}

#[derive(Debug)]
struct VkInstanceCreateInfoWrapper {
    create_info: vks::VkInstanceCreateInfo,
    application_info: Option<Box<VkApplicationInfoWrapper>>,
    enabled_layers: Vec<CString>,
    enabled_layers_ptrs: Vec<*const c_char>,
    enabled_extensions: Vec<CString>,
    enabled_extensions_ptrs: Vec<*const c_char>,
}

impl Deref for VkInstanceCreateInfoWrapper {
    type Target = vks::VkInstanceCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vks::VkInstanceCreateInfo> for VkInstanceCreateInfoWrapper {
    fn as_ref(&self) -> &vks::VkInstanceCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a InstanceCreateInfo> for VkInstanceCreateInfoWrapper {
    fn from(create_info: &'a InstanceCreateInfo) -> Self {
        let application_info_ptr;
        let application_info = match create_info.application_info {
            Some(ref application_info) => {
                let application_info: Box<VkApplicationInfoWrapper> = Box::new(application_info.into());
                application_info_ptr = &**application_info as *const _;
                Some(application_info)
            }

            None => {
                application_info_ptr = ptr::null();
                None
            }
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

        let enabled_extensions: Vec<_> = create_info.enabled_extensions.iter()
            .cloned()
            .map(String::from)
            .map(CString::new)
            .map(Result::unwrap)
            .collect();
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

        VkInstanceCreateInfoWrapper {
            create_info: vks::VkInstanceCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
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
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
            linear_tiling_features: properties.linearTilingFeatures,
            optimal_tiling_features: properties.optimalTilingFeatures,
            buffer_features: properties.bufferFeatures,
        }
    }
}

impl<'a> From<&'a FormatProperties> for vks::VkFormatProperties {
    fn from(properties: &'a FormatProperties) -> Self {
        vks::VkFormatProperties {
            linearTilingFeatures: properties.linear_tiling_features,
            optimalTilingFeatures: properties.optimal_tiling_features,
            bufferFeatures: properties.buffer_features,
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
            sample_counts: properties.sampleCounts,
            max_resource_size: properties.maxResourceSize,
        }
    }
}

impl<'a> From<&'a ImageFormatProperties> for vks::VkImageFormatProperties {
    fn from(properties: &'a ImageFormatProperties) -> Self {
        vks::VkImageFormatProperties {
            maxExtent: (&properties.max_extent).into(),
            maxMipLevels: properties.max_mip_levels,
            maxArrayLayers: properties.max_array_layers,
            sampleCounts: properties.sample_counts,
            maxResourceSize: properties.max_resource_size,
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
            framebuffer_color_sample_counts: limits.framebufferColorSampleCounts,
            framebuffer_depth_sample_counts: limits.framebufferDepthSampleCounts,
            framebuffer_stencil_sample_counts: limits.framebufferStencilSampleCounts,
            framebuffer_no_attachments_sample_counts: limits.framebufferNoAttachmentsSampleCounts,
            max_color_attachments: limits.maxColorAttachments,
            sampled_image_color_sample_counts: limits.sampledImageColorSampleCounts,
            sampled_image_integer_sample_counts: limits.sampledImageIntegerSampleCounts,
            sampled_image_depth_sample_counts: limits.sampledImageDepthSampleCounts,
            sampled_image_stencil_sample_counts: limits.sampledImageStencilSampleCounts,
            storage_image_sample_counts: limits.storageImageSampleCounts,
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
            framebufferColorSampleCounts: limits.framebuffer_color_sample_counts,
            framebufferDepthSampleCounts: limits.framebuffer_depth_sample_counts,
            framebufferStencilSampleCounts: limits.framebuffer_stencil_sample_counts,
            framebufferNoAttachmentsSampleCounts: limits.framebuffer_no_attachments_sample_counts,
            maxColorAttachments: limits.max_color_attachments,
            sampledImageColorSampleCounts: limits.sampled_image_color_sample_counts,
            sampledImageIntegerSampleCounts: limits.sampled_image_integer_sample_counts,
            sampledImageDepthSampleCounts: limits.sampled_image_depth_sample_counts,
            sampledImageStencilSampleCounts: limits.sampled_image_stencil_sample_counts,
            storageImageSampleCounts: limits.storage_image_sample_counts,
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
        let mut res = vks::VkPhysicalDeviceProperties {
            apiVersion: properties.api_version.as_api_version(),
            driverVersion: properties.driver_version,
            vendorID: properties.vendor_id,
            deviceID: properties.device_id,
            deviceType: properties.device_type.into(),
            deviceName: unsafe { mem::uninitialized() },
            pipelineCacheUUID: properties.pipeline_cache_uuid,
            limits: (&properties.limits).into(),
            sparseProperties: (&properties.sparse_properties).into(),
        };

        debug_assert!(properties.device_name.len() < res.deviceName.len());
        unsafe {
            ptr::copy_nonoverlapping(properties.device_name.as_ptr() as *const _, res.deviceName.as_mut_ptr(), properties.device_name.len());
        }
        res.deviceName[properties.device_name.len()] = 0;

        res
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
            queue_flags: properties.queueFlags,
            queue_count: properties.queueCount,
            timestamp_valid_bits: properties.timestampValidBits,
            min_image_transfer_granularity: (&properties.minImageTransferGranularity).into(),
        }
    }
}

impl<'a> From<&'a QueueFamilyProperties> for vks::VkQueueFamilyProperties {
    fn from(properties: &'a QueueFamilyProperties) -> Self {
        vks::VkQueueFamilyProperties {
            queueFlags: properties.queue_flags,
            queueCount: properties.queue_count,
            timestampValidBits: properties.timestamp_valid_bits,
            minImageTransferGranularity: (&properties.min_image_transfer_granularity).into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct QueueFamilyPropertiesIterator(::std::vec::IntoIter<vks::VkQueueFamilyProperties>);

impl Iterator for QueueFamilyPropertiesIterator {
    type Item = QueueFamilyProperties;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().as_ref().map(From::from)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl DoubleEndedIterator for QueueFamilyPropertiesIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().as_ref().map(From::from)
    }
}

impl ExactSizeIterator for QueueFamilyPropertiesIterator {
    fn len(&self) -> usize {
        self.0.len()
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
            property_flags: memory_type.propertyFlags,
            heap_index: memory_type.heapIndex,
        }
    }
}

impl<'a> From<&'a MemoryType> for vks::VkMemoryType {
    fn from(memory_type: &'a MemoryType) -> Self {
        vks::VkMemoryType {
            propertyFlags: memory_type.property_flags,
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
            flags: heap.flags,
        }
    }
}

impl<'a> From<&'a MemoryHeap> for vks::VkMemoryHeap {
    fn from(heap: &'a MemoryHeap) -> Self {
        vks::VkMemoryHeap {
            size: heap.size,
            flags: heap.flags,
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

        let mut res: vks::VkPhysicalDeviceMemoryProperties = unsafe { mem::uninitialized() };

        res.memoryTypeCount = properties.memory_types.len() as u32;
        for (src, dst) in properties.memory_types.iter().zip(res.memoryTypes.iter_mut()) {
            *dst = src.into();
        }

        res.memoryHeapCount = properties.memory_heaps.len() as u32;
        for (src, dst) in properties.memory_heaps.iter().zip(res.memoryHeaps.iter_mut()) {
            *dst = src.into();
        }

        res
    }
}

/// See [`VkDeviceQueueCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceQueueCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum DeviceQueueCreateInfoChainElement {
}

/// See [`VkDeviceQueueCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceQueueCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceQueueCreateInfo {
    pub chain: Vec<DeviceQueueCreateInfoChainElement>,
    pub flags: DeviceQueueCreateFlags,
    pub queue_family_index: u32,
    pub queue_priorities: Vec<f32>,
}

impl<'a> From<&'a vks::VkDeviceQueueCreateInfo> for DeviceQueueCreateInfo {
    fn from(create_info: &'a vks::VkDeviceQueueCreateInfo) -> Self {
        debug_assert_eq!(create_info.pNext, ptr::null());

        let queue_priorities_slice = unsafe {
            slice::from_raw_parts(create_info.pQueuePriorities, create_info.queueCount as usize)
        };

        DeviceQueueCreateInfo {
            chain: vec![],
            flags: create_info.flags,
            queue_family_index: create_info.queueFamilyIndex,
            queue_priorities: queue_priorities_slice.to_vec(),
        }
    }
}

#[derive(Debug)]
struct VkDeviceQueueCreateInfoWrapper {
    create_info: vks::VkDeviceQueueCreateInfo,
    queue_priorities: Vec<f32>,
}

impl Deref for VkDeviceQueueCreateInfoWrapper {
    type Target = vks::VkDeviceQueueCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vks::VkDeviceQueueCreateInfo> for VkDeviceQueueCreateInfoWrapper {
    fn as_ref(&self) -> &vks::VkDeviceQueueCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a DeviceQueueCreateInfo> for VkDeviceQueueCreateInfoWrapper {
    fn from(create_info: &'a DeviceQueueCreateInfo) -> Self {
        let queue_priorities = create_info.queue_priorities.clone();

        VkDeviceQueueCreateInfoWrapper {
            create_info: vks::VkDeviceQueueCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
                queueFamilyIndex: create_info.queue_family_index,
                queueCount: queue_priorities.len() as u32,
                pQueuePriorities: queue_priorities.as_ptr(),
            },
            queue_priorities: queue_priorities,
        }
    }
}

/// See [`VkDeviceCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum DeviceCreateInfoChainElement {
}

/// See [`VkDeviceCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDeviceCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceCreateInfo {
    pub chain: Vec<DeviceCreateInfoChainElement>,
    pub flags: DeviceCreateFlags,
    pub queue_create_infos: Vec<DeviceQueueCreateInfo>,
    pub enabled_layers: Vec<String>,
    pub enabled_extensions: Vec<DeviceExtension>,
    pub enabled_features: Option<PhysicalDeviceFeatures>,
}

impl<'a> From<&'a vks::VkDeviceCreateInfo> for DeviceCreateInfo {
    fn from(create_info: &'a vks::VkDeviceCreateInfo) -> Self {
        debug_assert_eq!(create_info.pNext, ptr::null());

        let queue_create_infos_slice = unsafe {
            slice::from_raw_parts(create_info.pQueueCreateInfos, create_info.queueCreateInfoCount as usize)
        };
        let queue_create_infos = queue_create_infos_slice
            .iter()
            .map(From::from)
            .collect();

        let enabled_layers_slice = unsafe {
            slice::from_raw_parts(create_info.ppEnabledLayerNames, create_info.enabledLayerCount as usize)
        };
        let enabled_layers = enabled_layers_slice
            .iter()
            .map(|&e| unsafe { CStr::from_ptr(e).to_str().unwrap().to_owned() })
            .collect();

        let enabled_extensions_slice = unsafe {
            slice::from_raw_parts(create_info.ppEnabledExtensionNames, create_info.enabledExtensionCount as usize)
        };
        let enabled_extensions = enabled_extensions_slice
            .iter()
            .map(|&e| unsafe { CStr::from_ptr(e).to_str().unwrap() })
            .map(From::from)
            .collect();

        let enabled_features = if !create_info.pEnabledFeatures.is_null() {
            unsafe {
                Some((&*create_info.pEnabledFeatures).into())
            }
        }
        else {
            None
        };

        DeviceCreateInfo {
            chain: vec![],
            flags: create_info.flags,
            queue_create_infos: queue_create_infos,
            enabled_layers: enabled_layers,
            enabled_extensions: enabled_extensions,
            enabled_features: enabled_features,
        }
    }
}

#[derive(Debug)]
struct VkDeviceCreateInfoWrapper {
    create_info: vks::VkDeviceCreateInfo,
    queue_create_infos_wrappers: Vec<VkDeviceQueueCreateInfoWrapper>,
    queue_create_infos: Vec<vks::VkDeviceQueueCreateInfo>,
    enabled_layers: Vec<CString>,
    enabled_layers_ptrs: Vec<*const c_char>,
    enabled_extensions: Vec<CString>,
    enabled_extensions_ptrs: Vec<*const c_char>,
    enabled_features: Option<Box<vks::VkPhysicalDeviceFeatures>>,
}

impl Deref for VkDeviceCreateInfoWrapper {
    type Target = vks::VkDeviceCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vks::VkDeviceCreateInfo> for VkDeviceCreateInfoWrapper {
    fn as_ref(&self) -> &vks::VkDeviceCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a DeviceCreateInfo> for VkDeviceCreateInfoWrapper {
    fn from(create_info: &'a DeviceCreateInfo) -> Self {
        let queue_create_infos_wrappers: Vec<VkDeviceQueueCreateInfoWrapper> = create_info.queue_create_infos
            .iter()
            .map(From::from)
            .collect();

        let queue_create_infos: Vec<vks::VkDeviceQueueCreateInfo> = queue_create_infos_wrappers
            .iter()
            .map(AsRef::as_ref)
            .cloned()
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

        let enabled_extensions: Vec<_> = create_info.enabled_extensions
            .iter()
            .cloned()
            .map(String::from)
            .map(CString::new)
            .map(Result::unwrap)
            .collect();
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

        VkDeviceCreateInfoWrapper {
            create_info: vks::VkDeviceCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
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
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InstanceExtension {
    Unknown(String),

    #[cfg(feature = "khr_surface_25")]
    KHRSurface,
}

impl From<String> for InstanceExtension {
    fn from(name: String) -> Self {
        match name.as_str() {
            #[cfg(feature = "khr_surface_25")]
            vks::VK_KHR_SURFACE_EXTENSION_NAME_STR => InstanceExtension::KHRSurface,

            _ => InstanceExtension::Unknown(name)
        }
    }
}

impl<'a> From<&'a str> for InstanceExtension {
    fn from(name: &'a str) -> Self {
        match name {
            #[cfg(feature = "khr_surface_25")]
            vks::VK_KHR_SURFACE_EXTENSION_NAME_STR => InstanceExtension::KHRSurface,

            _ => InstanceExtension::Unknown(name.to_owned())
        }
    }
}

impl From<InstanceExtension> for String {
    fn from(extension: InstanceExtension) -> Self {
        match extension {
            #[cfg(feature = "khr_surface_25")]
            InstanceExtension::KHRSurface => vks::VK_KHR_SURFACE_EXTENSION_NAME_STR.to_owned(),

            InstanceExtension::Unknown(name) => name,
        }
    }
}

impl<'a> From<&'a InstanceExtension> for &'a str {
    fn from(extension: &'a InstanceExtension) -> Self {
        match *extension {
            #[cfg(feature = "khr_surface_25")]
            InstanceExtension::KHRSurface => vks::VK_KHR_SURFACE_EXTENSION_NAME_STR,

            InstanceExtension::Unknown(ref name) => name,
        }
    }
}

/// See [`VkExtensionProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExtensionProperties)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InstanceExtensionProperties {
    pub extension: InstanceExtension,
    pub spec_version: u32,
}

impl<'a> From<&'a vks::VkExtensionProperties> for InstanceExtensionProperties {
    fn from(properties: &'a vks::VkExtensionProperties) -> Self {
        let name = unsafe { CStr::from_ptr(properties.extensionName.as_ptr()).to_str().unwrap() };

        InstanceExtensionProperties {
            extension: name.into(),
            spec_version: properties.specVersion,
        }
    }
}

impl<'a> From<&'a InstanceExtensionProperties> for vks::VkExtensionProperties {
    fn from(properties: &'a InstanceExtensionProperties) -> Self {
        unsafe {
            let name: &str = (&properties.extension).into();
            debug_assert!(name.len() < vks::VK_MAX_EXTENSION_NAME_SIZE);

            let mut res: vks::VkExtensionProperties = mem::uninitialized();
            ptr::copy_nonoverlapping(name.as_ptr() as *const _, res.extensionName.as_mut_ptr(), name.len());
            res.extensionName[name.len()] = 0;
            res.specVersion = properties.spec_version;

            res
        }
    }
}

#[derive(Debug, Clone)]
pub struct InstanceExtensionPropertiesIterator(::std::vec::IntoIter<vks::VkExtensionProperties>);

impl Iterator for InstanceExtensionPropertiesIterator {
    type Item = InstanceExtensionProperties;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().as_ref().map(From::from)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl DoubleEndedIterator for InstanceExtensionPropertiesIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().as_ref().map(From::from)
    }
}

impl ExactSizeIterator for InstanceExtensionPropertiesIterator {
    fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DeviceExtension {
    Unknown(String),
}

impl From<String> for DeviceExtension {
    fn from(name: String) -> Self {
        DeviceExtension::Unknown(name)
    }
}

impl<'a> From<&'a str> for DeviceExtension {
    fn from(name: &'a str) -> Self {
        DeviceExtension::Unknown(name.to_owned())
    }
}

impl From<DeviceExtension> for String {
    fn from(extension: DeviceExtension) -> Self {
        match extension {
            DeviceExtension::Unknown(name) => name,
        }
    }
}

impl<'a> From<&'a DeviceExtension> for &'a str {
    fn from(extension: &'a DeviceExtension) -> Self {
        match *extension {
            DeviceExtension::Unknown(ref name) => name,
        }
    }
}

/// See [`VkExtensionProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkExtensionProperties)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeviceExtensionProperties {
    pub extension: DeviceExtension,
    pub spec_version: u32,
}

impl<'a> From<&'a vks::VkExtensionProperties> for DeviceExtensionProperties {
    fn from(properties: &'a vks::VkExtensionProperties) -> Self {
        let name = unsafe { CStr::from_ptr(properties.extensionName.as_ptr()).to_str().unwrap() };

        DeviceExtensionProperties {
            extension: name.into(),
            spec_version: properties.specVersion,
        }
    }
}

impl<'a> From<&'a DeviceExtensionProperties> for vks::VkExtensionProperties {
    fn from(properties: &'a DeviceExtensionProperties) -> Self {
        unsafe {
            let name: &str = (&properties.extension).into();
            debug_assert!(name.len() < vks::VK_MAX_EXTENSION_NAME_SIZE);

            let mut res: vks::VkExtensionProperties = mem::uninitialized();
            ptr::copy_nonoverlapping(name.as_ptr() as *const _, res.extensionName.as_mut_ptr(), name.len());
            res.extensionName[name.len()] = 0;
            res.specVersion = properties.spec_version;

            res
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeviceExtensionPropertiesIterator(::std::vec::IntoIter<vks::VkExtensionProperties>);

impl Iterator for DeviceExtensionPropertiesIterator {
    type Item = DeviceExtensionProperties;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().as_ref().map(From::from)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl DoubleEndedIterator for DeviceExtensionPropertiesIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().as_ref().map(From::from)
    }
}

impl ExactSizeIterator for DeviceExtensionPropertiesIterator {
    fn len(&self) -> usize {
        self.0.len()
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

impl<'a> From<&'a LayerProperties> for vks::VkLayerProperties {
    fn from(properties: &'a LayerProperties) -> Self {
        unsafe {
            debug_assert!(properties.layer_name.len() < vks::VK_MAX_EXTENSION_NAME_SIZE);
            debug_assert!(properties.description.len() < vks::VK_MAX_DESCRIPTION_SIZE);

            let mut res: vks::VkLayerProperties = mem::uninitialized();

            ptr::copy_nonoverlapping(properties.layer_name.as_ptr() as *const _, res.layerName.as_mut_ptr(), properties.layer_name.len());
            res.layerName[properties.layer_name.len()] = 0;

            res.specVersion = properties.spec_version.as_api_version();
            res.implementationVersion = properties.implementation_version;

            ptr::copy_nonoverlapping(properties.description.as_ptr() as *const _, res.description.as_mut_ptr(), properties.description.len());
            res.description[properties.description.len()] = 0;

            res
        }
    }
}

#[derive(Debug, Clone)]
pub struct LayerPropertiesIterator(::std::vec::IntoIter<vks::VkLayerProperties>);

impl Iterator for LayerPropertiesIterator {
    type Item = LayerProperties;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().as_ref().map(From::from)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl DoubleEndedIterator for LayerPropertiesIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().as_ref().map(From::from)
    }
}

impl ExactSizeIterator for LayerPropertiesIterator {
    fn len(&self) -> usize {
        self.0.len()
    }
}

/// See [`VkSubmitInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSubmitInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum SubmitInfoChainElement {
}

/// See [`VkSubmitInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSubmitInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct SubmitInfo {
    pub chain: Vec<SubmitInfoChainElement>,
    pub wait_semaphores: Option<Vec<Semaphore>>,
    pub wait_dst_stage_mask: Option<Vec<PipelineStageFlags>>,
    pub command_buffers: Option<Vec<CommandBuffer>>,
    pub signal_semaphores: Option<Vec<Semaphore>>,
}

#[derive(Debug)]
struct VkSubmitInfoWrapper {
    info: vks::VkSubmitInfo,
    wait_semaphores: Option<Vec<Semaphore>>,
    wait_vk_semaphores: Option<Vec<vks::VkSemaphore>>,
    wait_dst_stage_mask: Option<Vec<vks::VkPipelineStageFlags>>,
    command_buffers: Option<Vec<CommandBuffer>>,
    vk_command_buffers: Option<Vec<vks::VkCommandBuffer>>,
    signal_semaphores: Option<Vec<Semaphore>>,
    signal_vk_semaphores: Option<Vec<vks::VkSemaphore>>,
}

impl Deref for VkSubmitInfoWrapper {
    type Target = vks::VkSubmitInfo;

    fn deref(&self) -> &Self::Target {
        &self.info
    }
}

impl AsRef<vks::VkSubmitInfo> for VkSubmitInfoWrapper {
    fn as_ref(&self) -> &vks::VkSubmitInfo {
        &self.info
    }
}

impl<'a> From<&'a SubmitInfo> for VkSubmitInfoWrapper {
    fn from(info: &'a SubmitInfo) -> Self {
        let (wait_semaphores_count, wait_vk_semaphores_ptr, wait_semaphores, wait_vk_semaphores) = match info.wait_semaphores {
            Some(ref wait_semaphores) => {
                let wait_semaphores = wait_semaphores.clone();
                let wait_vk_semaphores: Vec<_> = wait_semaphores.iter().map(Semaphore::handle).collect();
                (wait_semaphores.len() as u32, wait_vk_semaphores.as_ptr(), Some(wait_semaphores), Some(wait_vk_semaphores))
            }

            None => (0, ptr::null(), None, None),
        };

        let (wait_dst_stage_mask_ptr, wait_dst_stage_mask) = match info.wait_dst_stage_mask {
            Some(ref wait_dst_stage_mask) => {
                let wait_dst_stage_mask = wait_dst_stage_mask.clone();
                (wait_dst_stage_mask.as_ptr(), Some(wait_dst_stage_mask))
            }

            None => (ptr::null(), None),
        };

        let (command_buffers_count, vk_command_buffers_ptr, command_buffers, vk_command_buffers) = match info.command_buffers {
            Some(ref command_buffers) => {
                let command_buffers = command_buffers.clone();
                let vk_command_buffers: Vec<_> = command_buffers.iter().map(CommandBuffer::handle).collect();
                (command_buffers.len() as u32, vk_command_buffers.as_ptr(), Some(command_buffers), Some(vk_command_buffers))
            }

            None => (0, ptr::null(), None, None),
        };

        let (signal_semaphores_count, signal_vk_semaphores_ptr, signal_semaphores, signal_vk_semaphores) = match info.signal_semaphores {
            Some(ref signal_semaphores) => {
                let signal_semaphores = signal_semaphores.clone();
                let signal_vk_semaphores: Vec<_> = signal_semaphores.iter().map(Semaphore::handle).collect();
                (signal_semaphores.len() as u32, signal_vk_semaphores.as_ptr(), Some(signal_semaphores), Some(signal_vk_semaphores))
            }

            None => (0, ptr::null(), None, None),
        };

        VkSubmitInfoWrapper {
            info: vks::VkSubmitInfo {
                sType: vks::VK_STRUCTURE_TYPE_SUBMIT_INFO,
                pNext: ptr::null(),
                waitSemaphoreCount: wait_semaphores_count,
                pWaitSemaphores: wait_vk_semaphores_ptr,
                pWaitDstStageMask: wait_dst_stage_mask_ptr,
                commandBufferCount: command_buffers_count,
                pCommandBuffers: vk_command_buffers_ptr,
                signalSemaphoreCount: signal_semaphores_count,
                pSignalSemaphores: signal_vk_semaphores_ptr,
            },
            wait_semaphores: wait_semaphores,
            wait_vk_semaphores: wait_vk_semaphores,
            wait_dst_stage_mask: wait_dst_stage_mask,
            command_buffers: command_buffers,
            vk_command_buffers: vk_command_buffers,
            signal_semaphores: signal_semaphores,
            signal_vk_semaphores: signal_vk_semaphores,
        }
    }
}

/// See [`VkMemoryAllocateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryAllocateInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum MemoryAllocateInfoChainElement {
}

/// See [`VkMemoryAllocateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryAllocateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct MemoryAllocateInfo {
    pub chain: Vec<MemoryAllocateInfoChainElement>,
    pub allocation_size: u64,
    pub memory_type_index: u32,
}

impl<'a> From<&'a vks::VkMemoryAllocateInfo> for MemoryAllocateInfo {
    fn from(info: &'a vks::VkMemoryAllocateInfo) -> Self {
        debug_assert_eq!(info.pNext, ptr::null());

        MemoryAllocateInfo {
            chain: vec![],
            allocation_size: info.allocationSize,
            memory_type_index: info.memoryTypeIndex,
        }
    }
}

#[derive(Debug)]
struct VkMemoryAllocateInfoWrapper {
    info: vks::VkMemoryAllocateInfo,
}

impl Deref for VkMemoryAllocateInfoWrapper {
    type Target = vks::VkMemoryAllocateInfo;

    fn deref(&self) -> &Self::Target {
        &self.info
    }
}

impl AsRef<vks::VkMemoryAllocateInfo> for VkMemoryAllocateInfoWrapper {
    fn as_ref(&self) -> &vks::VkMemoryAllocateInfo {
        &self.info
    }
}

impl<'a> From<&'a MemoryAllocateInfo> for VkMemoryAllocateInfoWrapper {
    fn from(info: &'a MemoryAllocateInfo) -> Self {
        VkMemoryAllocateInfoWrapper {
            info: vks::VkMemoryAllocateInfo {
                sType: vks::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
                pNext: ptr::null(),
                allocationSize: info.allocation_size,
                memoryTypeIndex: info.memory_type_index,
            },
        }
    }
}

/// See [`VkMappedMemoryRange`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMappedMemoryRange)
#[derive(Debug, Clone, PartialEq)]
pub enum MappedMemoryRangeChainElement {
}

/// See [`VkMappedMemoryRange`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMappedMemoryRange)
#[derive(Debug, Clone, PartialEq)]
pub struct MappedMemoryRange {
    pub chain: Vec<MappedMemoryRangeChainElement>,
    pub memory: DeviceMemory,
    pub offset: u64,
    pub size: OptionalDeviceSize,
}

#[derive(Debug)]
struct VkMappedMemoryRangeWrapper {
    range: vks::VkMappedMemoryRange,
    memory: DeviceMemory,
}

impl Deref for VkMappedMemoryRangeWrapper {
    type Target = vks::VkMappedMemoryRange;

    fn deref(&self) -> &Self::Target {
        &self.range
    }
}

impl AsRef<vks::VkMappedMemoryRange> for VkMappedMemoryRangeWrapper {
    fn as_ref(&self) -> &vks::VkMappedMemoryRange {
        &self.range
    }
}

impl<'a> From<&'a MappedMemoryRange> for VkMappedMemoryRangeWrapper {
    fn from(range: &'a MappedMemoryRange) -> Self {
        VkMappedMemoryRangeWrapper {
            range: vks::VkMappedMemoryRange {
                sType: vks::VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE,
                pNext: ptr::null(),
                memory: range.memory.handle(),
                offset: range.offset,
                size: range.size.into(),
            },
            memory: range.memory.clone(),
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

impl<'a> From<&'a MemoryRequirements> for vks::VkMemoryRequirements {
    fn from(requirements: &'a MemoryRequirements) -> Self {
        vks::VkMemoryRequirements {
            size: requirements.size,
            alignment: requirements.alignment,
            memoryTypeBits: requirements.memory_type_bits,
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
            aspect_mask: properties.aspectMask,
            image_granularity: (&properties.imageGranularity).into(),
            flags: properties.flags,
        }
    }
}

impl<'a> From<&'a SparseImageFormatProperties> for vks::VkSparseImageFormatProperties {
    fn from(properties: &'a SparseImageFormatProperties) -> Self {
        vks::VkSparseImageFormatProperties {
            aspectMask: properties.aspect_mask,
            imageGranularity: (&properties.image_granularity).into(),
            flags: properties.flags,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SparseImageFormatPropertiesIterator(::std::vec::IntoIter<vks::VkSparseImageFormatProperties>);

impl Iterator for SparseImageFormatPropertiesIterator {
    type Item = SparseImageFormatProperties;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().as_ref().map(From::from)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl DoubleEndedIterator for SparseImageFormatPropertiesIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().as_ref().map(From::from)
    }
}

impl ExactSizeIterator for SparseImageFormatPropertiesIterator {
    fn len(&self) -> usize {
        self.0.len()
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

impl<'a> From<&'a SparseImageMemoryRequirements> for vks::VkSparseImageMemoryRequirements {
    fn from(requirements: &'a SparseImageMemoryRequirements) -> Self {
        vks::VkSparseImageMemoryRequirements {
            formatProperties: (&requirements.format_properties).into(),
            imageMipTailFirstLod: requirements.image_mip_tail_first_lod,
            imageMipTailSize: requirements.image_mip_tail_size,
            imageMipTailOffset: requirements.image_mip_tail_offset,
            imageMipTailStride: requirements.image_mip_tail_stride,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SparseImageMemoryRequirementsIterator(::std::vec::IntoIter<vks::VkSparseImageMemoryRequirements>);

impl Iterator for SparseImageMemoryRequirementsIterator {
    type Item = SparseImageMemoryRequirements;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().as_ref().map(From::from)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl DoubleEndedIterator for SparseImageMemoryRequirementsIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().as_ref().map(From::from)
    }
}

impl ExactSizeIterator for SparseImageMemoryRequirementsIterator {
    fn len(&self) -> usize {
        self.0.len()
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
    bind: vks::VkSparseMemoryBind,
    memory: Option<DeviceMemory>,
}

impl Deref for VkSparseMemoryBindWrapper {
    type Target = vks::VkSparseMemoryBind;

    fn deref(&self) -> &Self::Target {
        &self.bind
    }
}

impl AsRef<vks::VkSparseMemoryBind> for VkSparseMemoryBindWrapper {
    fn as_ref(&self) -> &vks::VkSparseMemoryBind {
        &self.bind
    }
}

impl<'a> From<&'a SparseMemoryBind> for VkSparseMemoryBindWrapper {
    fn from(bind: &'a SparseMemoryBind) -> Self {
        let (vk_memory, memory) = match bind.memory {
            Some(ref memory) => (memory.handle(), Some(memory.clone())),
            None => (ptr::null_mut(), None),
        };

        VkSparseMemoryBindWrapper {
            bind: vks::VkSparseMemoryBind {
                resourceOffset: bind.resource_offset,
                size: bind.size,
                memory: vk_memory,
                memoryOffset: bind.memory_offset,
                flags: bind.flags,
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
    info: vks::VkSparseBufferMemoryBindInfo,
    buffer: Buffer,
    binds: Vec<VkSparseMemoryBindWrapper>,
    binds_vk: Vec<vks::VkSparseMemoryBind>,
}

impl Deref for VkSparseBufferMemoryBindInfoWrapper {
    type Target = vks::VkSparseBufferMemoryBindInfo;

    fn deref(&self) -> &Self::Target {
        &self.info
    }
}

impl AsRef<vks::VkSparseBufferMemoryBindInfo> for VkSparseBufferMemoryBindInfoWrapper {
    fn as_ref(&self) -> &vks::VkSparseBufferMemoryBindInfo {
        &self.info
    }
}

impl<'a> From<&'a SparseBufferMemoryBindInfo> for VkSparseBufferMemoryBindInfoWrapper {
    fn from(info: &'a SparseBufferMemoryBindInfo) -> Self {
        let binds: Vec<_> = info.binds.iter().map(From::from).collect();
        let binds_vk: Vec<_> = binds.iter().map(AsRef::as_ref).cloned().collect();

        VkSparseBufferMemoryBindInfoWrapper {
            info: vks::VkSparseBufferMemoryBindInfo {
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
    info: vks::VkSparseImageOpaqueMemoryBindInfo,
    image: Image,
    binds: Vec<VkSparseMemoryBindWrapper>,
    binds_vk: Vec<vks::VkSparseMemoryBind>,
}

impl Deref for VkSparseImageOpaqueMemoryBindInfoWrapper {
    type Target = vks::VkSparseImageOpaqueMemoryBindInfo;

    fn deref(&self) -> &Self::Target {
        &self.info
    }
}

impl AsRef<vks::VkSparseImageOpaqueMemoryBindInfo> for VkSparseImageOpaqueMemoryBindInfoWrapper {
    fn as_ref(&self) -> &vks::VkSparseImageOpaqueMemoryBindInfo {
        &self.info
    }
}

impl<'a> From<&'a SparseImageOpaqueMemoryBindInfo> for VkSparseImageOpaqueMemoryBindInfoWrapper {
    fn from(info: &'a SparseImageOpaqueMemoryBindInfo) -> Self {
        let binds: Vec<_> = info.binds.iter().map(From::from).collect();
        let binds_vk: Vec<_> = binds.iter().map(AsRef::as_ref).cloned().collect();

        VkSparseImageOpaqueMemoryBindInfoWrapper {
            info: vks::VkSparseImageOpaqueMemoryBindInfo {
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

impl<'a> From<&'a vks::VkImageSubresource> for ImageSubresource {
    fn from(subresource: &'a vks::VkImageSubresource) -> Self {
        ImageSubresource {
            aspect_mask: subresource.aspectMask,
            mip_level: subresource.mipLevel,
            array_layer: subresource.arrayLayer,
        }
    }
}

impl<'a> From<&'a ImageSubresource> for vks::VkImageSubresource {
    fn from(subresource: &'a ImageSubresource) -> Self {
        vks::VkImageSubresource {
            aspectMask: subresource.aspect_mask,
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

impl<'a> From<&'a vks::VkOffset3D> for Offset3D {
    fn from(offset: &'a vks::VkOffset3D) -> Self {
        Offset3D {
            x: offset.x,
            y: offset.y,
            z: offset.z,
        }
    }
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
    bind: vks::VkSparseImageMemoryBind,
    memory: Option<DeviceMemory>,
}

impl Deref for VkSparseImageMemoryBindWrapper {
    type Target = vks::VkSparseImageMemoryBind;

    fn deref(&self) -> &Self::Target {
        &self.bind
    }
}

impl AsRef<vks::VkSparseImageMemoryBind> for VkSparseImageMemoryBindWrapper {
    fn as_ref(&self) -> &vks::VkSparseImageMemoryBind {
        &self.bind
    }
}

impl<'a> From<&'a SparseImageMemoryBind> for VkSparseImageMemoryBindWrapper {
    fn from(bind: &'a SparseImageMemoryBind) -> Self {
        let (vk_memory, memory) = match bind.memory {
            Some(ref memory) => (memory.handle(), Some(memory.clone())),
            None => (ptr::null_mut(), None),
        };

        VkSparseImageMemoryBindWrapper {
            bind: vks::VkSparseImageMemoryBind {
                subresource: (&bind.subresource).into(),
                offset: (&bind.offset).into(),
                extent: (&bind.extent).into(),
                memory: vk_memory,
                memoryOffset: bind.memory_offset,
                flags: bind.flags,
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
    info: vks::VkSparseImageMemoryBindInfo,
    image: Image,
    binds: Vec<VkSparseImageMemoryBindWrapper>,
    binds_vk: Vec<vks::VkSparseImageMemoryBind>,
}

impl Deref for VkSparseImageMemoryBindInfoWrapper {
    type Target = vks::VkSparseImageMemoryBindInfo;

    fn deref(&self) -> &Self::Target {
        &self.info
    }
}

impl AsRef<vks::VkSparseImageMemoryBindInfo> for VkSparseImageMemoryBindInfoWrapper {
    fn as_ref(&self) -> &vks::VkSparseImageMemoryBindInfo {
        &self.info
    }
}

impl<'a> From<&'a SparseImageMemoryBindInfo> for VkSparseImageMemoryBindInfoWrapper {
    fn from(info: &'a SparseImageMemoryBindInfo) -> Self {
        let binds: Vec<_> = info.binds.iter().map(From::from).collect();
        let binds_vk: Vec<_> = binds.iter().map(AsRef::as_ref).cloned().collect();

        VkSparseImageMemoryBindInfoWrapper {
            info: vks::VkSparseImageMemoryBindInfo {
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

/// See [`VkBindSparseInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBindSparseInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum BindSparseInfoChainElement {
}

/// See [`VkBindSparseInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBindSparseInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct BindSparseInfo {
    pub chain: Vec<BindSparseInfoChainElement>,
    pub wait_semaphores: Option<Vec<Semaphore>>,
    pub buffer_binds: Option<Vec<SparseBufferMemoryBindInfo>>,
    pub image_opaque_binds: Option<Vec<SparseImageOpaqueMemoryBindInfo>>,
    pub image_binds: Option<Vec<SparseImageMemoryBindInfo>>,
    pub signal_semaphores: Option<Vec<Semaphore>>,
}

#[derive(Debug)]
struct VkBindSparseInfoWrapper {
    info: vks::VkBindSparseInfo,
    wait_semaphores: Option<Vec<Semaphore>>,
    wait_vk_semaphores: Option<Vec<vks::VkSemaphore>>,
    buffer_binds: Option<Vec<VkSparseBufferMemoryBindInfoWrapper>>,
    vk_buffer_binds: Option<Vec<vks::VkSparseBufferMemoryBindInfo>>,
    image_opaque_binds: Option<Vec<VkSparseImageOpaqueMemoryBindInfoWrapper>>,
    vk_image_opaque_binds: Option<Vec<vks::VkSparseImageOpaqueMemoryBindInfo>>,
    image_binds: Option<Vec<VkSparseImageMemoryBindInfoWrapper>>,
    vk_image_binds: Option<Vec<vks::VkSparseImageMemoryBindInfo>>,
    signal_semaphores: Option<Vec<Semaphore>>,
    signal_vk_semaphores: Option<Vec<vks::VkSemaphore>>,
}

impl Deref for VkBindSparseInfoWrapper {
    type Target = vks::VkBindSparseInfo;

    fn deref(&self) -> &Self::Target {
        &self.info
    }
}

impl AsRef<vks::VkBindSparseInfo> for VkBindSparseInfoWrapper {
    fn as_ref(&self) -> &vks::VkBindSparseInfo {
        &self.info
    }
}

impl<'a> From<&'a BindSparseInfo> for VkBindSparseInfoWrapper {
    fn from(info: &'a BindSparseInfo) -> Self {
        let (wait_semaphores_count, wait_vk_semaphores_ptr, wait_semaphores, wait_vk_semaphores) = match info.wait_semaphores {
            Some(ref wait_semaphores) => {
                let wait_semaphores = wait_semaphores.clone();
                let wait_vk_semaphores: Vec<_> = wait_semaphores.iter().map(Semaphore::handle).collect();
                (wait_semaphores.len() as u32, wait_vk_semaphores.as_ptr(), Some(wait_semaphores), Some(wait_vk_semaphores))
            }

            None => (0, ptr::null(), None, None),
        };

        let (buffer_binds_count, vk_buffer_binds_ptr, buffer_binds, vk_buffer_binds) = match info.buffer_binds {
            Some(ref buffer_binds) => {
                let buffer_binds: Vec<_> = buffer_binds.iter().map(From::from).collect();
                let vk_buffer_binds: Vec<_> = buffer_binds.iter().map(AsRef::as_ref).cloned().collect();
                (buffer_binds.len() as u32, vk_buffer_binds.as_ptr(), Some(buffer_binds), Some(vk_buffer_binds))
            }

            None => (0, ptr::null(), None, None),
        };

        let (image_opaque_binds_count, vk_image_opaque_binds_ptr, image_opaque_binds, vk_image_opaque_binds) = match info.image_opaque_binds {
            Some(ref image_opaque_binds) => {
                let image_opaque_binds: Vec<_> = image_opaque_binds.iter().map(From::from).collect();
                let vk_image_opaque_binds: Vec<_> = image_opaque_binds.iter().map(AsRef::as_ref).cloned().collect();
                (image_opaque_binds.len() as u32, vk_image_opaque_binds.as_ptr(), Some(image_opaque_binds), Some(vk_image_opaque_binds))
            }

            None => (0, ptr::null(), None, None),
        };

        let (image_binds_count, vk_image_binds_ptr, image_binds, vk_image_binds) = match info.image_binds {
            Some(ref image_binds) => {
                let image_binds: Vec<_> = image_binds.iter().map(From::from).collect();
                let vk_image_binds: Vec<_> = image_binds.iter().map(AsRef::as_ref).cloned().collect();
                (image_binds.len() as u32, vk_image_binds.as_ptr(), Some(image_binds), Some(vk_image_binds))
            }

            None => (0, ptr::null(), None, None),
        };

        let (signal_semaphores_count, signal_vk_semaphores_ptr, signal_semaphores, signal_vk_semaphores) = match info.signal_semaphores {
            Some(ref signal_semaphores) => {
                let signal_semaphores = signal_semaphores.clone();
                let signal_vk_semaphores: Vec<_> = signal_semaphores.iter().map(Semaphore::handle).collect();
                (signal_semaphores.len() as u32, signal_vk_semaphores.as_ptr(), Some(signal_semaphores), Some(signal_vk_semaphores))
            }

            None => (0, ptr::null(), None, None),
        };

        VkBindSparseInfoWrapper {
            info: vks::VkBindSparseInfo {
                sType: vks::VK_STRUCTURE_TYPE_BIND_SPARSE_INFO,
                pNext: ptr::null(),
                waitSemaphoreCount: wait_semaphores_count,
                pWaitSemaphores: wait_vk_semaphores_ptr,
                bufferBindCount: buffer_binds_count,
                pBufferBinds: vk_buffer_binds_ptr,
                imageOpaqueBindCount: image_opaque_binds_count,
                pImageOpaqueBinds: vk_image_opaque_binds_ptr,
                imageBindCount: image_binds_count,
                pImageBinds: vk_image_binds_ptr,
                signalSemaphoreCount: signal_semaphores_count,
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
        }
    }
}

/// See [`VkFenceCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFenceCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum FenceCreateInfoChainElement {
}

/// See [`VkFenceCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFenceCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct FenceCreateInfo {
    pub chain: Vec<FenceCreateInfoChainElement>,
    pub flags: FenceCreateFlags,
}

impl<'a> From<&'a vks::VkFenceCreateInfo> for FenceCreateInfo {
    fn from(create_info: &'a vks::VkFenceCreateInfo) -> Self {
        debug_assert_eq!(create_info.pNext, ptr::null());

        FenceCreateInfo {
            chain: vec![],
            flags: create_info.flags,
        }
    }
}

#[derive(Debug)]
struct VkFenceCreateInfoWrapper {
    create_info: vks::VkFenceCreateInfo,
}

impl Deref for VkFenceCreateInfoWrapper {
    type Target = vks::VkFenceCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vks::VkFenceCreateInfo> for VkFenceCreateInfoWrapper {
    fn as_ref(&self) -> &vks::VkFenceCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a FenceCreateInfo> for VkFenceCreateInfoWrapper {
    fn from(create_info: &'a FenceCreateInfo) -> VkFenceCreateInfoWrapper {
        VkFenceCreateInfoWrapper {
            create_info: vks::VkFenceCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
            },
        }
    }
}

/// See [`VkSemaphoreCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSemaphoreCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum SemaphoreCreateInfoChainElement {
}

/// See [`VkSemaphoreCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSemaphoreCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct SemaphoreCreateInfo {
    pub chain: Vec<SemaphoreCreateInfoChainElement>,
    pub flags: SemaphoreCreateFlags,
}

impl<'a> From<&'a vks::VkSemaphoreCreateInfo> for SemaphoreCreateInfo {
    fn from(create_info: &'a vks::VkSemaphoreCreateInfo) -> Self {
        debug_assert_eq!(create_info.pNext, ptr::null());

        SemaphoreCreateInfo {
            chain: vec![],
            flags: create_info.flags,
        }
    }
}

#[derive(Debug)]
struct VkSemaphoreCreateInfoWrapper {
    create_info: vks::VkSemaphoreCreateInfo,
}

impl Deref for VkSemaphoreCreateInfoWrapper {
    type Target = vks::VkSemaphoreCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vks::VkSemaphoreCreateInfo> for VkSemaphoreCreateInfoWrapper {
    fn as_ref(&self) -> &vks::VkSemaphoreCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a SemaphoreCreateInfo> for VkSemaphoreCreateInfoWrapper {
    fn from(create_info: &'a SemaphoreCreateInfo) -> VkSemaphoreCreateInfoWrapper {
        VkSemaphoreCreateInfoWrapper {
            create_info: vks::VkSemaphoreCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
            },
        }
    }
}

/// See [`VkEventCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkEventCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum EventCreateInfoChainElement {
}

/// See [`VkEventCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkEventCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct EventCreateInfo {
    pub chain: Vec<EventCreateInfoChainElement>,
    pub flags: EventCreateFlags,
}

impl<'a> From<&'a vks::VkEventCreateInfo> for EventCreateInfo {
    fn from(create_info: &'a vks::VkEventCreateInfo) -> Self {
        debug_assert_eq!(create_info.pNext, ptr::null());

        EventCreateInfo {
            chain: vec![],
            flags: create_info.flags,
        }
    }
}

#[derive(Debug)]
struct VkEventCreateInfoWrapper {
    create_info: vks::VkEventCreateInfo,
}

impl Deref for VkEventCreateInfoWrapper {
    type Target = vks::VkEventCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vks::VkEventCreateInfo> for VkEventCreateInfoWrapper {
    fn as_ref(&self) -> &vks::VkEventCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a EventCreateInfo> for VkEventCreateInfoWrapper {
    fn from(create_info: &'a EventCreateInfo) -> VkEventCreateInfoWrapper {
        VkEventCreateInfoWrapper {
            create_info: vks::VkEventCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
            },
        }
    }
}

/// See [`VkQueryPoolCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPoolCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum QueryPoolCreateInfoChainElement {
}

/// See [`VkQueryPoolCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueryPoolCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct QueryPoolCreateInfo {
    pub chain: Vec<QueryPoolCreateInfoChainElement>,
    pub flags: QueryPoolCreateFlags,
    pub query_type: QueryType,
    pub query_count: u32,
    pub pipeline_statistics: QueryPipelineStatisticFlags,
}

impl<'a> From<&'a vks::VkQueryPoolCreateInfo> for QueryPoolCreateInfo {
    fn from(create_info: &'a vks::VkQueryPoolCreateInfo) -> Self {
        debug_assert_eq!(create_info.pNext, ptr::null());

        QueryPoolCreateInfo {
            chain: vec![],
            flags: create_info.flags,
            query_type: create_info.queryType.into(),
            query_count: create_info.queryCount,
            pipeline_statistics: create_info.pipelineStatistics,
        }
    }
}

#[derive(Debug)]
struct VkQueryPoolCreateInfoWrapper {
    create_info: vks::VkQueryPoolCreateInfo,
}

impl Deref for VkQueryPoolCreateInfoWrapper {
    type Target = vks::VkQueryPoolCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vks::VkQueryPoolCreateInfo> for VkQueryPoolCreateInfoWrapper {
    fn as_ref(&self) -> &vks::VkQueryPoolCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a QueryPoolCreateInfo> for VkQueryPoolCreateInfoWrapper {
    fn from(create_info: &'a QueryPoolCreateInfo) -> VkQueryPoolCreateInfoWrapper {
        VkQueryPoolCreateInfoWrapper {
            create_info: vks::VkQueryPoolCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
                queryType: create_info.query_type.into(),
                queryCount: create_info.query_count,
                pipelineStatistics: create_info.pipeline_statistics,
            },
        }
    }
}

/// See [`VkBufferCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum BufferCreateInfoChainElement {
}

/// See [`VkBufferCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct BufferCreateInfo {
    pub chain: Vec<BufferCreateInfoChainElement>,
    pub flags: BufferCreateFlags,
    pub size: u64,
    pub usage: BufferUsageFlags,
    pub sharing_mode: SharingMode,
    pub queue_family_indices: Option<Vec<u32>>,
}

impl<'a> From<&'a vks::VkBufferCreateInfo> for BufferCreateInfo {
    fn from(create_info: &'a vks::VkBufferCreateInfo) -> Self {
        debug_assert_eq!(create_info.pNext, ptr::null());

        let queue_family_indices = if !create_info.pQueueFamilyIndices.is_null() {
            unsafe {
                Some(slice::from_raw_parts(create_info.pQueueFamilyIndices, create_info.queueFamilyIndexCount as usize).to_vec())
            }
        }
        else {
            None
        };

        BufferCreateInfo {
            chain: vec![],
            flags: create_info.flags,
            size: create_info.size,
            usage: create_info.usage,
            sharing_mode: create_info.sharingMode.into(),
            queue_family_indices: queue_family_indices,
        }
    }
}

#[derive(Debug)]
struct VkBufferCreateInfoWrapper {
    create_info: vks::VkBufferCreateInfo,
    queue_family_indices: Option<Vec<u32>>,
}

impl Deref for VkBufferCreateInfoWrapper {
    type Target = vks::VkBufferCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vks::VkBufferCreateInfo> for VkBufferCreateInfoWrapper {
    fn as_ref(&self) -> &vks::VkBufferCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a BufferCreateInfo> for VkBufferCreateInfoWrapper {
    fn from(create_info: &'a BufferCreateInfo) -> Self {
        let queue_family_indices = create_info.queue_family_indices.clone();
        let (queue_family_indices_ptr, queue_family_index_count) = match queue_family_indices {
            Some(ref queue_family_indices) => (queue_family_indices.as_ptr(), queue_family_indices.len() as u32),
            None => (ptr::null(), 0)
        };

        VkBufferCreateInfoWrapper {
            create_info: vks::VkBufferCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
                size: create_info.size,
                usage: create_info.usage,
                sharingMode: create_info.sharing_mode.into(),
                queueFamilyIndexCount: queue_family_index_count,
                pQueueFamilyIndices: queue_family_indices_ptr,
            },
            queue_family_indices: queue_family_indices,
        }
    }
}

/// See [`VkBufferViewCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferViewCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum BufferViewCreateInfoChainElement {
}

/// See [`VkBufferViewCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferViewCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct BufferViewCreateInfo {
    pub chain: Vec<BufferViewCreateInfoChainElement>,
    pub flags: BufferViewCreateFlags,
    pub buffer: Buffer,
    pub format: Format,
    pub offset: u64,
    pub range: OptionalDeviceSize,
}

#[derive(Debug)]
struct VkBufferViewCreateInfoWrapper {
    create_info: vks::VkBufferViewCreateInfo,
    buffer: Buffer,
}

impl Deref for VkBufferViewCreateInfoWrapper {
    type Target = vks::VkBufferViewCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vks::VkBufferViewCreateInfo> for VkBufferViewCreateInfoWrapper {
    fn as_ref(&self) -> &vks::VkBufferViewCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a BufferViewCreateInfo> for VkBufferViewCreateInfoWrapper {
    fn from(create_info: &'a BufferViewCreateInfo) -> Self {
        VkBufferViewCreateInfoWrapper {
            create_info: vks::VkBufferViewCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
                buffer: create_info.buffer.handle(),
                format: create_info.format.into(),
                offset: create_info.offset,
                range: create_info.range.into(),
            },
            buffer: create_info.buffer.clone(),
        }
    }
}

/// See [`VkImageCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum ImageCreateInfoChainElement {
}

/// See [`VkImageCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct ImageCreateInfo {
    pub chain: Vec<ImageCreateInfoChainElement>,
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
    pub queue_family_indices: Option<Vec<u32>>,
    pub initial_layout: ImageLayout,
}

impl<'a> From<&'a vks::VkImageCreateInfo> for ImageCreateInfo {
    fn from(create_info: &'a vks::VkImageCreateInfo) -> Self {
        debug_assert_eq!(create_info.pNext, ptr::null());

        let queue_family_indices = if !create_info.pQueueFamilyIndices.is_null() {
            unsafe {
                Some(slice::from_raw_parts(create_info.pQueueFamilyIndices, create_info.queueFamilyIndexCount as usize).to_vec())
            }
        }
        else {
            None
        };

        ImageCreateInfo {
            chain: vec![],
            flags: create_info.flags,
            image_type: create_info.imageType.into(),
            format: create_info.format.into(),
            extent: (&create_info.extent).into(),
            mip_levels: create_info.mipLevels,
            array_layers: create_info.arrayLayers,
            samples: create_info.samples,
            tiling: create_info.tiling.into(),
            usage: create_info.usage,
            sharing_mode: create_info.sharingMode.into(),
            queue_family_indices: queue_family_indices,
            initial_layout: create_info.initialLayout.into(),
        }
    }
}

#[derive(Debug)]
struct VkImageCreateInfoWrapper {
    create_info: vks::VkImageCreateInfo,
    queue_family_indices: Option<Vec<u32>>,
}

impl Deref for VkImageCreateInfoWrapper {
    type Target = vks::VkImageCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vks::VkImageCreateInfo> for VkImageCreateInfoWrapper {
    fn as_ref(&self) -> &vks::VkImageCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a ImageCreateInfo> for VkImageCreateInfoWrapper {
    fn from(create_info: &'a ImageCreateInfo) -> Self {
        let queue_family_indices = create_info.queue_family_indices.clone();
        let (queue_family_indices_ptr, queue_family_index_count) = match queue_family_indices {
            Some(ref queue_family_indices) => (queue_family_indices.as_ptr(), queue_family_indices.len() as u32),
            None => (ptr::null(), 0)
        };

        VkImageCreateInfoWrapper {
            create_info: vks::VkImageCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
                imageType: create_info.image_type.into(),
                format: create_info.format.into(),
                extent: (&create_info.extent).into(),
                mipLevels: create_info.mip_levels,
                arrayLayers: create_info.array_layers,
                samples: create_info.samples,
                tiling: create_info.tiling.into(),
                usage: create_info.usage,
                sharingMode: create_info.sharing_mode.into(),
                queueFamilyIndexCount: queue_family_index_count,
                pQueueFamilyIndices: queue_family_indices_ptr,
                initialLayout: create_info.initial_layout.into(),
            },
            queue_family_indices: queue_family_indices,
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

impl<'a> From<&'a SubresourceLayout> for vks::VkSubresourceLayout {
    fn from(layout: &'a SubresourceLayout) -> Self {
        vks::VkSubresourceLayout {
            offset: layout.offset,
            size: layout.size,
            rowPitch: layout.row_pitch,
            arrayPitch: layout.array_pitch,
            depthPitch: layout.depth_pitch,
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

impl<'a> From<&'a vks::VkComponentMapping> for ComponentMapping {
    fn from(mapping: &'a vks::VkComponentMapping) -> Self {
        ComponentMapping {
            r: mapping.r.into(),
            g: mapping.g.into(),
            b: mapping.b.into(),
            a: mapping.a.into(),
        }
    }
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

/// See [`VkImageSubresourceRange`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageSubresourceRange)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ImageSubresourceRange {
    pub aspect_mask: ImageAspectFlags,
    pub base_mip_level: u32,
    pub level_count: OptionalMipLevels,
    pub base_array_layer: u32,
    pub layer_count: OptionalArrayLayers,
}

impl<'a> From<&'a vks::VkImageSubresourceRange> for ImageSubresourceRange {
    fn from(range: &'a vks::VkImageSubresourceRange) -> Self {
        ImageSubresourceRange {
            aspect_mask: range.aspectMask,
            base_mip_level: range.baseMipLevel,
            level_count: range.levelCount.into(),
            base_array_layer: range.baseArrayLayer,
            layer_count: range.layerCount.into(),
        }
    }
}

impl<'a> From<&'a ImageSubresourceRange> for vks::VkImageSubresourceRange {
    fn from(range: &'a ImageSubresourceRange) -> Self {
        vks::VkImageSubresourceRange {
            aspectMask: range.aspect_mask,
            baseMipLevel: range.base_mip_level,
            levelCount: range.level_count.into(),
            baseArrayLayer: range.base_array_layer,
            layerCount: range.layer_count.into(),
        }
    }
}

/// See [`VkImageViewCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageViewCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum ImageViewCreateInfoChainElement {
}

/// See [`VkImageViewCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageViewCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct ImageViewCreateInfo {
    pub chain: Vec<ImageViewCreateInfoChainElement>,
    pub flags: ImageViewCreateFlags,
    pub image: Image,
    pub view_type: ImageViewType,
    pub format: Format,
    pub components: ComponentMapping,
    pub subresource_range: ImageSubresourceRange,
}

#[derive(Debug)]
struct VkImageViewCreateInfoWrapper {
    create_info: vks::VkImageViewCreateInfo,
    image: Image,
}

impl Deref for VkImageViewCreateInfoWrapper {
    type Target = vks::VkImageViewCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vks::VkImageViewCreateInfo> for VkImageViewCreateInfoWrapper {
    fn as_ref(&self) -> &vks::VkImageViewCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a ImageViewCreateInfo> for VkImageViewCreateInfoWrapper {
    fn from(create_info: &'a ImageViewCreateInfo) -> Self {
        VkImageViewCreateInfoWrapper {
            create_info: vks::VkImageViewCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
                image: create_info.image.handle(),
                viewType: create_info.view_type.into(),
                format: create_info.format.into(),
                components: (&create_info.components).into(),
                subresourceRange: (&create_info.subresource_range).into(),
            },
            image: create_info.image.clone(),
        }
    }
}

/// See [`VkShaderModuleCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderModuleCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum ShaderModuleCreateInfoChainElement {
}

/// See [`VkShaderModuleCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkShaderModuleCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct ShaderModuleCreateInfo {
    pub chain: Vec<ShaderModuleCreateInfoChainElement>,
    pub flags: ShaderModuleCreateFlags,
    pub code_size: usize,
    pub code: Vec<u32>,
}

impl<'a> From<&'a vks::VkShaderModuleCreateInfo> for ShaderModuleCreateInfo {
    fn from(create_info: &'a vks::VkShaderModuleCreateInfo) -> Self {
        debug_assert_eq!(create_info.pNext, ptr::null());

        let code_size_u32 = (create_info.codeSize / 4) + 1;
        let mut code = Vec::with_capacity(code_size_u32);
        unsafe {
            code.set_len(code_size_u32);
            ptr::copy_nonoverlapping(create_info.pCode as *const u8, code.as_mut_ptr() as *mut u8, create_info.codeSize);
        }

        ShaderModuleCreateInfo {
            chain: vec![],
            flags: create_info.flags,
            code_size: create_info.codeSize,
            code: code,
        }
    }
}

#[derive(Debug)]
struct VkShaderModuleCreateInfoWrapper {
    create_info: vks::VkShaderModuleCreateInfo,
    code: Vec<u32>,
}

impl Deref for VkShaderModuleCreateInfoWrapper {
    type Target = vks::VkShaderModuleCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vks::VkShaderModuleCreateInfo> for VkShaderModuleCreateInfoWrapper {
    fn as_ref(&self) -> &vks::VkShaderModuleCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a ShaderModuleCreateInfo> for VkShaderModuleCreateInfoWrapper {
    fn from(create_info: &'a ShaderModuleCreateInfo) -> Self {
        let code = create_info.code.clone();

        VkShaderModuleCreateInfoWrapper {
            create_info: vks::VkShaderModuleCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
                codeSize: create_info.code_size,
                pCode: code.as_ptr(),
            },
            code: code,
        }
    }
}

/// See [`VkPipelineCacheCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCacheCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum PipelineCacheCreateInfoChainElement {
}

/// See [`VkPipelineCacheCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineCacheCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct PipelineCacheCreateInfo {
    pub chain: Vec<PipelineCacheCreateInfoChainElement>,
    pub flags: PipelineCacheCreateFlags,
    pub initial_data: Option<Vec<u8>>,
}

impl<'a> From<&'a vks::VkPipelineCacheCreateInfo> for PipelineCacheCreateInfo {
    fn from(create_info: &'a vks::VkPipelineCacheCreateInfo) -> Self {
        debug_assert_eq!(create_info.pNext, ptr::null());

        let initial_data = if create_info.initialDataSize > 0 {
            unsafe {
                Some(slice::from_raw_parts(create_info.pInitialData as *const u8, create_info.initialDataSize).to_vec())
            }
        }
        else {
            None
        };

        PipelineCacheCreateInfo {
            chain: vec![],
            flags: create_info.flags,
            initial_data: initial_data,
        }
    }
}

#[derive(Debug)]
struct VkPipelineCacheCreateInfoWrapper {
    create_info: vks::VkPipelineCacheCreateInfo,
    initial_data: Option<Vec<u8>>,
}

impl Deref for VkPipelineCacheCreateInfoWrapper {
    type Target = vks::VkPipelineCacheCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vks::VkPipelineCacheCreateInfo> for VkPipelineCacheCreateInfoWrapper {
    fn as_ref(&self) -> &vks::VkPipelineCacheCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a PipelineCacheCreateInfo> for VkPipelineCacheCreateInfoWrapper {
    fn from(create_info: &'a PipelineCacheCreateInfo) -> Self {
        let (initial_data, initial_data_size, initial_data_ptr) = match create_info.initial_data {
            Some(ref initial_data) => {
                let initial_data = initial_data.clone();
                let initial_data_size = initial_data.len();
                let initial_data_ptr = initial_data.as_ptr() as *const c_void;
                (Some(initial_data), initial_data_size, initial_data_ptr)
            }

            None => (None, 0, ptr::null()),
        };

        VkPipelineCacheCreateInfoWrapper {
            create_info: vks::VkPipelineCacheCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
                initialDataSize: initial_data_size,
                pInitialData: initial_data_ptr,
            },
            initial_data: initial_data,
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

impl<'a> From<&'a vks::VkSpecializationMapEntry> for SpecializationMapEntry {
    fn from(entry: &'a vks::VkSpecializationMapEntry) -> Self {
        SpecializationMapEntry {
            constant_id: entry.constantID,
            offset: entry.offset,
            size: entry.size,
        }
    }
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

impl<'a> From<&'a vks::VkSpecializationInfo> for SpecializationInfo {
    fn from(info: &'a vks::VkSpecializationInfo) -> Self {
        let map_entries = if !info.pMapEntries.is_null() {
            unsafe {
                Some(slice::from_raw_parts(info.pMapEntries, info.mapEntryCount as usize).iter().map(From::from).collect())
            }
        }
        else {
            None
        };

        let data = if !info.pData.is_null() {
            unsafe {
                Some(slice::from_raw_parts(info.pData as *const u8, info.dataSize).to_vec())
            }
        }
        else {
            None
        };

        SpecializationInfo {
            map_entries: map_entries,
            data: data,
        }
    }
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
    info: vks::VkSpecializationInfo,
    map_entries: Option<Vec<vks::VkSpecializationMapEntry>>,
    data: Option<Vec<u8>>,
}

impl Deref for VkSpecializationInfoWrapper {
    type Target = vks::VkSpecializationInfo;

    fn deref(&self) -> &Self::Target {
        &self.info
    }
}

impl AsRef<vks::VkSpecializationInfo> for VkSpecializationInfoWrapper {
    fn as_ref(&self) -> &vks::VkSpecializationInfo {
        &self.info
    }
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
            info: vks::VkSpecializationInfo {
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

/// See [`VkPipelineShaderStageCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineShaderStageCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum PipelineShaderStageCreateInfoChainElement {
}

/// See [`VkPipelineShaderStageCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineShaderStageCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct PipelineShaderStageCreateInfo {
    pub chain: Vec<PipelineShaderStageCreateInfoChainElement>,
    pub flags: PipelineShaderStageCreateFlags,
    pub stage: ShaderStageFlagBits,
    pub module: ShaderModule,
    pub name: String,
    pub specialization_info: Option<SpecializationInfo>,
}

#[derive(Debug)]
struct VkPipelineShaderStageCreateInfoWrapper {
    create_info: vks::VkPipelineShaderStageCreateInfo,
    module: ShaderModule,
    name: CString,
    specialization_info: Option<Box<VkSpecializationInfoWrapper>>,
}

impl Deref for VkPipelineShaderStageCreateInfoWrapper {
    type Target = vks::VkPipelineShaderStageCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vks::VkPipelineShaderStageCreateInfo> for VkPipelineShaderStageCreateInfoWrapper {
    fn as_ref(&self) -> &vks::VkPipelineShaderStageCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a PipelineShaderStageCreateInfo> for VkPipelineShaderStageCreateInfoWrapper {
    fn from(create_info: &'a PipelineShaderStageCreateInfo) -> Self {
        let name = CString::new(create_info.name.clone()).unwrap();

        let (specialization_info_ptr, specialization_info) = match create_info.specialization_info {
            Some(ref specialization_info) => {
                let specialization_info: Box<VkSpecializationInfoWrapper> = Box::new(specialization_info.into());
                (&**specialization_info as *const _, Some(specialization_info))
            }

            None => (ptr::null(), None),
        };

        VkPipelineShaderStageCreateInfoWrapper {
            create_info: vks::VkPipelineShaderStageCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
                stage: create_info.stage,
                module: create_info.module.handle(),
                pName: name.as_ptr(),
                pSpecializationInfo: specialization_info_ptr,
            },
            module: create_info.module.clone(),
            name: name,
            specialization_info: specialization_info,
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

impl<'a> From<&'a vks::VkVertexInputBindingDescription> for VertexInputBindingDescription {
    fn from(description: &'a vks::VkVertexInputBindingDescription) -> Self {
        VertexInputBindingDescription {
            binding: description.binding,
            stride: description.stride,
            input_rate: description.inputRate.into(),
        }
    }
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

impl<'a> From<&'a vks::VkVertexInputAttributeDescription> for VertexInputAttributeDescription {
    fn from(description: &'a vks::VkVertexInputAttributeDescription) -> Self {
        VertexInputAttributeDescription {
            location: description.location,
            binding: description.binding,
            format: description.format.into(),
            offset: description.offset,
        }
    }
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

/// See [`VkPipelineVertexInputStateCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineVertexInputStateCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum PipelineVertexInputStateCreateInfoChainElement {
}

/// See [`VkPipelineVertexInputStateCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineVertexInputStateCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct PipelineVertexInputStateCreateInfo {
    pub chain: Vec<PipelineVertexInputStateCreateInfoChainElement>,
    pub flags: PipelineVertexInputStateCreateFlags,
    pub vertex_binding_descriptions: Option<Vec<VertexInputBindingDescription>>,
    pub vertex_attribute_descriptions: Option<Vec<VertexInputAttributeDescription>>,
}

impl<'a> From<&'a vks::VkPipelineVertexInputStateCreateInfo> for PipelineVertexInputStateCreateInfo {
    fn from(create_info: &'a vks::VkPipelineVertexInputStateCreateInfo) -> Self {
        debug_assert_eq!(create_info.pNext, ptr::null());

        let vertex_binding_descriptions = if !create_info.pVertexBindingDescriptions.is_null() {
            unsafe {
                Some(slice::from_raw_parts(create_info.pVertexBindingDescriptions, create_info.vertexBindingDescriptionCount as usize)
                     .iter()
                     .map(From::from)
                     .collect())
            }
        }
        else {
            None
        };

        let vertex_attribute_descriptions = if !create_info.pVertexAttributeDescriptions.is_null() {
            unsafe {
                Some(slice::from_raw_parts(create_info.pVertexAttributeDescriptions, create_info.vertexAttributeDescriptionCount as usize)
                     .iter()
                     .map(From::from)
                     .collect())
            }
        }
        else {
            None
        };

        PipelineVertexInputStateCreateInfo {
            chain: vec![],
            flags: create_info.flags,
            vertex_binding_descriptions: vertex_binding_descriptions,
            vertex_attribute_descriptions: vertex_attribute_descriptions,
        }
    }
}

#[derive(Debug)]
struct VkPipelineVertexInputStateCreateInfoWrapper {
    create_info: vks::VkPipelineVertexInputStateCreateInfo,
    vertex_binding_descriptions: Option<Vec<vks::VkVertexInputBindingDescription>>,
    vertex_attribute_descriptions: Option<Vec<vks::VkVertexInputAttributeDescription>>,
}

impl Deref for VkPipelineVertexInputStateCreateInfoWrapper {
    type Target = vks::VkPipelineVertexInputStateCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vks::VkPipelineVertexInputStateCreateInfo> for VkPipelineVertexInputStateCreateInfoWrapper {
    fn as_ref(&self) -> &vks::VkPipelineVertexInputStateCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a PipelineVertexInputStateCreateInfo> for VkPipelineVertexInputStateCreateInfoWrapper {
    fn from(create_info: &'a PipelineVertexInputStateCreateInfo) -> Self {
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

        VkPipelineVertexInputStateCreateInfoWrapper {
            create_info: vks::VkPipelineVertexInputStateCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
                vertexBindingDescriptionCount: vertex_binding_descriptions_count,
                pVertexBindingDescriptions: vertex_binding_descriptions_ptr,
                vertexAttributeDescriptionCount: vertex_attribute_descriptions_count,
                pVertexAttributeDescriptions: vertex_attribute_descriptions_ptr,
            },
            vertex_binding_descriptions: vertex_binding_descriptions,
            vertex_attribute_descriptions: vertex_attribute_descriptions,
        }
    }
}

/// See [`VkPipelineInputAssemblyStateCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineInputAssemblyStateCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum PipelineInputAssemblyStateCreateInfoChainElement {
}

/// See [`VkPipelineInputAssemblyStateCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineInputAssemblyStateCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct PipelineInputAssemblyStateCreateInfo {
    pub chain: Vec<PipelineInputAssemblyStateCreateInfoChainElement>,
    pub flags: PipelineInputAssemblyStateCreateFlags,
    pub topology: PrimitiveTopology,
    pub primitive_restart_enable: bool,
}

impl<'a> From<&'a vks::VkPipelineInputAssemblyStateCreateInfo> for PipelineInputAssemblyStateCreateInfo {
    fn from(create_info: &'a vks::VkPipelineInputAssemblyStateCreateInfo) -> Self {
        debug_assert!(create_info.pNext.is_null());

        PipelineInputAssemblyStateCreateInfo {
            chain: vec![],
            flags: create_info.flags,
            topology: create_info.topology.into(),
            primitive_restart_enable: utils::from_vk_bool(create_info.primitiveRestartEnable),
        }
    }
}

#[derive(Debug)]
struct VkPipelineInputAssemblyStateCreateInfoWrapper {
    create_info: vks::VkPipelineInputAssemblyStateCreateInfo,
}

impl Deref for VkPipelineInputAssemblyStateCreateInfoWrapper {
    type Target = vks::VkPipelineInputAssemblyStateCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vks::VkPipelineInputAssemblyStateCreateInfo> for VkPipelineInputAssemblyStateCreateInfoWrapper {
    fn as_ref(&self) -> &vks::VkPipelineInputAssemblyStateCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a PipelineInputAssemblyStateCreateInfo> for VkPipelineInputAssemblyStateCreateInfoWrapper {
    fn from(create_info: &'a PipelineInputAssemblyStateCreateInfo) -> Self {
        VkPipelineInputAssemblyStateCreateInfoWrapper {
            create_info: vks::VkPipelineInputAssemblyStateCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
                topology: create_info.topology.into(),
                primitiveRestartEnable: utils::to_vk_bool(create_info.primitive_restart_enable),
            },
        }
    }
}

/// See [`VkPipelineTessellationStateCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineTessellationStateCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum PipelineTessellationStateCreateInfoChainElement {
}

/// See [`VkPipelineTessellationStateCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineTessellationStateCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct PipelineTessellationStateCreateInfo {
    pub chain: Vec<PipelineTessellationStateCreateInfoChainElement>,
    pub flags: PipelineTessellationStateCreateFlags,
    pub patch_control_points: u32,
}

impl<'a> From<&'a vks::VkPipelineTessellationStateCreateInfo> for PipelineTessellationStateCreateInfo {
    fn from(create_info: &'a vks::VkPipelineTessellationStateCreateInfo) -> Self {
        debug_assert!(create_info.pNext.is_null());

        PipelineTessellationStateCreateInfo {
            chain: vec![],
            flags: create_info.flags,
            patch_control_points: create_info.patchControlPoints,
        }
    }
}

#[derive(Debug)]
struct VkPipelineTessellationStateCreateInfoWrapper {
    create_info: vks::VkPipelineTessellationStateCreateInfo,
}

impl Deref for VkPipelineTessellationStateCreateInfoWrapper {
    type Target = vks::VkPipelineTessellationStateCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vks::VkPipelineTessellationStateCreateInfo> for VkPipelineTessellationStateCreateInfoWrapper {
    fn as_ref(&self) -> &vks::VkPipelineTessellationStateCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a PipelineTessellationStateCreateInfo> for VkPipelineTessellationStateCreateInfoWrapper {
    fn from(create_info: &'a PipelineTessellationStateCreateInfo) -> Self {
        VkPipelineTessellationStateCreateInfoWrapper {
            create_info: vks::VkPipelineTessellationStateCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
                patchControlPoints: create_info.patch_control_points,
            },
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

impl<'a> From<&'a vks::VkViewport> for Viewport {
    fn from(viewport: &'a vks::VkViewport) -> Self {
        Viewport {
            x: viewport.x,
            y: viewport.y,
            width: viewport.width,
            height: viewport.height,
            min_depth: viewport.minDepth,
            max_depth: viewport.maxDepth,
        }
    }
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

/// See [`VkRect2D`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkRect2D)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Rect2D {
    pub offset: Offset2D,
    pub extent: Extent2D,
}

impl<'a> From<&'a vks::VkRect2D> for Rect2D {
    fn from(rect: &'a vks::VkRect2D) -> Self {
        Rect2D {
            offset: (&rect.offset).into(),
            extent: (&rect.extent).into(),
        }
    }
}

impl<'a> From<&'a Rect2D> for vks::VkRect2D {
    fn from(rect: &'a Rect2D) -> Self {
        vks::VkRect2D {
            offset: (&rect.offset).into(),
            extent: (&rect.extent).into(),
        }
    }
}

/// See [`VkPipelineViewportStateCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineViewportStateCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum PipelineViewportStateCreateInfoChainElement {
}

/// See [`VkPipelineViewportStateCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineViewportStateCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct PipelineViewportStateCreateInfo {
    pub chain: Vec<PipelineViewportStateCreateInfoChainElement>,
    pub flags: PipelineViewportStateCreateFlags,
    pub viewports: Vec<Viewport>,
    pub scissors: Vec<Rect2D>,
}

impl<'a> From<&'a vks::VkPipelineViewportStateCreateInfo> for PipelineViewportStateCreateInfo {
    fn from(create_info: &'a vks::VkPipelineViewportStateCreateInfo) -> Self {
        debug_assert!(create_info.pNext.is_null());

        let viewports = unsafe {
            slice::from_raw_parts(create_info.pViewports, create_info.viewportCount as usize)
                .iter()
                .map(From::from)
                .collect()
        };

        let scissors = unsafe {
            slice::from_raw_parts(create_info.pScissors, create_info.scissorCount as usize)
                .iter()
                .map(From::from)
                .collect()
        };

        PipelineViewportStateCreateInfo {
            chain: vec![],
            flags: create_info.flags,
            viewports: viewports,
            scissors: scissors,
        }
    }
}

#[derive(Debug)]
struct VkPipelineViewportStateCreateInfoWrapper {
    create_info: vks::VkPipelineViewportStateCreateInfo,
    viewports: Vec<vks::VkViewport>,
    scissors: Vec<vks::VkRect2D>,
}

impl Deref for VkPipelineViewportStateCreateInfoWrapper {
    type Target = vks::VkPipelineViewportStateCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vks::VkPipelineViewportStateCreateInfo> for VkPipelineViewportStateCreateInfoWrapper {
    fn as_ref(&self) -> &vks::VkPipelineViewportStateCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a PipelineViewportStateCreateInfo> for VkPipelineViewportStateCreateInfoWrapper {
    fn from(create_info: &'a PipelineViewportStateCreateInfo) -> Self {
        let viewports: Vec<_> = create_info.viewports.iter().map(From::from).collect();
        let scissors: Vec<_> = create_info.scissors.iter().map(From::from).collect();

        VkPipelineViewportStateCreateInfoWrapper {
            create_info: vks::VkPipelineViewportStateCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
                viewportCount: viewports.len() as u32,
                pViewports: viewports.as_ptr(),
                scissorCount: scissors.len() as u32,
                pScissors: scissors.as_ptr(),
            },
            viewports: viewports,
            scissors: scissors,
        }
    }
}

/// See [`VkPipelineRasterizationStateCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineRasterizationStateCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum PipelineRasterizationStateCreateInfoChainElement {
}

/// See [`VkPipelineRasterizationStateCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineRasterizationStateCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct PipelineRasterizationStateCreateInfo {
    pub chain: Vec<PipelineRasterizationStateCreateInfoChainElement>,
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
}

impl<'a> From<&'a vks::VkPipelineRasterizationStateCreateInfo> for PipelineRasterizationStateCreateInfo {
    fn from(create_info: &'a vks::VkPipelineRasterizationStateCreateInfo) -> Self {
        assert!(create_info.pNext.is_null());

        PipelineRasterizationStateCreateInfo {
            chain: vec![],
            flags: create_info.flags,
            depth_clamp_enable: utils::from_vk_bool(create_info.depthClampEnable),
            rasterizer_discard_enable: utils::from_vk_bool(create_info.rasterizerDiscardEnable),
            polygon_mode: create_info.polygonMode.into(),
            cull_mode: create_info.cullMode,
            front_face: create_info.frontFace.into(),
            depth_bias_enable: utils::from_vk_bool(create_info.depthBiasEnable),
            depth_bias_constant_factor: create_info.depthBiasConstantFactor,
            depth_bias_clamp: create_info.depthBiasClamp,
            depth_bias_slope_factor: create_info.depthBiasSlopeFactor,
            line_width: create_info.lineWidth,
        }
    }
}

#[derive(Debug)]
struct VkPipelineRasterizationStateCreateInfoWrapper {
    create_info: vks::VkPipelineRasterizationStateCreateInfo,
}

impl Deref for VkPipelineRasterizationStateCreateInfoWrapper {
    type Target = vks::VkPipelineRasterizationStateCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vks::VkPipelineRasterizationStateCreateInfo> for VkPipelineRasterizationStateCreateInfoWrapper {
    fn as_ref(&self) -> &vks::VkPipelineRasterizationStateCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a PipelineRasterizationStateCreateInfo> for VkPipelineRasterizationStateCreateInfoWrapper {
    fn from(create_info: &'a PipelineRasterizationStateCreateInfo) -> Self {
        VkPipelineRasterizationStateCreateInfoWrapper {
            create_info: vks::VkPipelineRasterizationStateCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
                depthClampEnable: utils::to_vk_bool(create_info.depth_clamp_enable),
                rasterizerDiscardEnable: utils::to_vk_bool(create_info.rasterizer_discard_enable),
                polygonMode: create_info.polygon_mode.into(),
                cullMode: create_info.cull_mode,
                frontFace: create_info.front_face.into(),
                depthBiasEnable: utils::to_vk_bool(create_info.depth_bias_enable),
                depthBiasConstantFactor: create_info.depth_bias_constant_factor,
                depthBiasClamp: create_info.depth_bias_clamp,
                depthBiasSlopeFactor: create_info.depth_bias_slope_factor,
                lineWidth: create_info.line_width,
            },
        }
    }
}

/// See [`VkPipelineMultisampleStateCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineMultisampleStateCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum PipelineMultisampleStateCreateInfoChainElement {
}

/// See [`VkPipelineMultisampleStateCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineMultisampleStateCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct PipelineMultisampleStateCreateInfo {
    pub chain: Vec<PipelineMultisampleStateCreateInfoChainElement>,
    pub flags: PipelineMultisampleStateCreateFlags,
    pub rasterization_samples: SampleCountFlagBits,
    pub sample_shading_enable: bool,
    pub min_sample_shading: f32,
    pub sample_mask: Option<Vec<u32>>,
    pub alpha_to_coverage_enable: bool,
    pub alpha_to_one_enable: bool,
}

impl<'a> From<&'a vks::VkPipelineMultisampleStateCreateInfo> for PipelineMultisampleStateCreateInfo {
    fn from(create_info: &'a vks::VkPipelineMultisampleStateCreateInfo) -> Self {
        debug_assert!(create_info.pNext.is_null());

        let sample_mask = if !create_info.pSampleMask.is_null() {
            let sample_mask_len = (create_info.rasterizationSamples.bits() as usize + 31) / 32;
            unsafe {
                Some(slice::from_raw_parts(create_info.pSampleMask, sample_mask_len).to_vec())
            }
        }
        else {
            None
        };

        PipelineMultisampleStateCreateInfo {
            chain: vec![],
            flags: create_info.flags,
            rasterization_samples: create_info.rasterizationSamples,
            sample_shading_enable: utils::from_vk_bool(create_info.sampleShadingEnable),
            min_sample_shading: create_info.minSampleShading,
            sample_mask: sample_mask,
            alpha_to_coverage_enable: utils::from_vk_bool(create_info.alphaToCoverageEnable),
            alpha_to_one_enable: utils::from_vk_bool(create_info.alphaToOneEnable),
        }
    }
}

#[derive(Debug)]
struct VkPipelineMultisampleStateCreateInfoWrapper {
    create_info: vks::VkPipelineMultisampleStateCreateInfo,
    sample_mask: Option<Vec<u32>>,
}

impl Deref for VkPipelineMultisampleStateCreateInfoWrapper {
    type Target = vks::VkPipelineMultisampleStateCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vks::VkPipelineMultisampleStateCreateInfo> for VkPipelineMultisampleStateCreateInfoWrapper {
    fn as_ref(&self) -> &vks::VkPipelineMultisampleStateCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a PipelineMultisampleStateCreateInfo> for VkPipelineMultisampleStateCreateInfoWrapper {
    fn from(create_info: &'a PipelineMultisampleStateCreateInfo) -> Self {
        let (sample_mask_ptr, sample_mask) = match create_info.sample_mask {
            Some(ref sample_mask) => {
                let sample_mask = sample_mask.clone();
                (sample_mask.as_ptr(), Some(sample_mask))
            }

            None => (ptr::null(), None),
        };

        VkPipelineMultisampleStateCreateInfoWrapper {
            create_info: vks::VkPipelineMultisampleStateCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
                rasterizationSamples: create_info.rasterization_samples,
                sampleShadingEnable: utils::to_vk_bool(create_info.sample_shading_enable),
                minSampleShading: create_info.min_sample_shading,
                pSampleMask: sample_mask_ptr,
                alphaToCoverageEnable: utils::to_vk_bool(create_info.alpha_to_coverage_enable),
                alphaToOneEnable: utils::to_vk_bool(create_info.alpha_to_one_enable),
            },
            sample_mask: sample_mask,
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

impl<'a> From<&'a vks::VkStencilOpState> for StencilOpState {
    fn from(state: &'a vks::VkStencilOpState) -> Self {
        StencilOpState {
            fail_op: state.failOp.into(),
            pass_op: state.passOp.into(),
            depth_fail_op: state.depthFailOp.into(),
            compare_op: state.compareOp.into(),
            compare_mask: state.compareMask,
            write_mask: state.writeMask,
            reference: state.reference,
        }
    }
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

/// See [`VkPipelineDepthStencilStateCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineDepthStencilStateCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum PipelineDepthStencilStateCreateInfoChainElement {
}

/// See [`VkPipelineDepthStencilStateCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineDepthStencilStateCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct PipelineDepthStencilStateCreateInfo {
    pub chain: Vec<PipelineDepthStencilStateCreateInfoChainElement>,
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
}

impl<'a> From<&'a vks::VkPipelineDepthStencilStateCreateInfo> for PipelineDepthStencilStateCreateInfo {
    fn from(create_info: &'a vks::VkPipelineDepthStencilStateCreateInfo) -> Self {
        assert!(create_info.pNext.is_null());

        PipelineDepthStencilStateCreateInfo {
            chain: vec![],
            flags: create_info.flags,
            depth_test_enable: utils::from_vk_bool(create_info.depthTestEnable),
            depth_write_enable: utils::from_vk_bool(create_info.depthWriteEnable),
            depth_compare_op: create_info.depthCompareOp.into(),
            depth_bounds_test_enable: utils::from_vk_bool(create_info.depthBoundsTestEnable),
            stencil_test_enable: utils::from_vk_bool(create_info.stencilTestEnable),
            front: (&create_info.front).into(),
            back: (&create_info.back).into(),
            min_depth_bounds: create_info.minDepthBounds,
            max_depth_bounds: create_info.maxDepthBounds,
        }
    }
}

#[derive(Debug)]
struct VkPipelineDepthStencilStateCreateInfoWrapper {
    create_info: vks::VkPipelineDepthStencilStateCreateInfo,
}

impl Deref for VkPipelineDepthStencilStateCreateInfoWrapper {
    type Target = vks::VkPipelineDepthStencilStateCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vks::VkPipelineDepthStencilStateCreateInfo> for VkPipelineDepthStencilStateCreateInfoWrapper {
    fn as_ref(&self) -> &vks::VkPipelineDepthStencilStateCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a PipelineDepthStencilStateCreateInfo> for VkPipelineDepthStencilStateCreateInfoWrapper {
    fn from(create_info: &'a PipelineDepthStencilStateCreateInfo) -> Self {
        VkPipelineDepthStencilStateCreateInfoWrapper {
            create_info: vks::VkPipelineDepthStencilStateCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
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

impl<'a> From<&'a vks::VkPipelineColorBlendAttachmentState> for PipelineColorBlendAttachmentState {
    fn from(state: &'a vks::VkPipelineColorBlendAttachmentState) -> Self {
        PipelineColorBlendAttachmentState {
            blend_enable: utils::from_vk_bool(state.blendEnable),
            src_color_blend_factor: state.srcColorBlendFactor.into(),
            dst_color_blend_factor: state.dstColorBlendFactor.into(),
            color_blend_op: state.colorBlendOp.into(),
            src_alpha_blend_factor: state.srcAlphaBlendFactor.into(),
            dst_alpha_blend_factor: state.dstAlphaBlendFactor.into(),
            alpha_blend_op: state.alphaBlendOp.into(),
            color_write_mask: state.colorWriteMask,
        }
    }
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
            colorWriteMask: state.color_write_mask,
        }
    }
}

/// See [`VkPipelineColorBlendStateCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineColorBlendStateCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum PipelineColorBlendStateCreateInfoChainElement {
}

/// See [`VkPipelineColorBlendStateCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineColorBlendStateCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct PipelineColorBlendStateCreateInfo {
    pub chain: Vec<PipelineColorBlendStateCreateInfoChainElement>,
    pub flags: PipelineColorBlendStateCreateFlags,
    pub logic_op_enable: bool,
    pub logic_op: LogicOp,
    pub attachments: Option<Vec<PipelineColorBlendAttachmentState>>,
    pub blend_constants: [f32; 4],
}

impl<'a> From<&'a vks::VkPipelineColorBlendStateCreateInfo> for PipelineColorBlendStateCreateInfo {
    fn from(create_info: &'a vks::VkPipelineColorBlendStateCreateInfo) -> Self {
        assert!(create_info.pNext.is_null());

        let attachments = if !create_info.pAttachments.is_null() {
            unsafe {
                Some(slice::from_raw_parts(create_info.pAttachments, create_info.attachmentCount as usize)
                     .iter()
                     .map(From::from)
                     .collect())
            }
        }
        else {
            None
        };

        PipelineColorBlendStateCreateInfo {
            chain: vec![],
            flags: create_info.flags,
            logic_op_enable: utils::from_vk_bool(create_info.logicOpEnable),
            logic_op: create_info.logicOp.into(),
            attachments: attachments,
            blend_constants: create_info.blendConstants,
        }
    }
}

#[derive(Debug)]
struct VkPipelineColorBlendStateCreateInfoWrapper {
    create_info: vks::VkPipelineColorBlendStateCreateInfo,
    attachments: Option<Vec<vks::VkPipelineColorBlendAttachmentState>>,
}

impl Deref for VkPipelineColorBlendStateCreateInfoWrapper {
    type Target = vks::VkPipelineColorBlendStateCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vks::VkPipelineColorBlendStateCreateInfo> for VkPipelineColorBlendStateCreateInfoWrapper {
    fn as_ref(&self) -> &vks::VkPipelineColorBlendStateCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a PipelineColorBlendStateCreateInfo> for VkPipelineColorBlendStateCreateInfoWrapper {
    fn from(create_info: &'a PipelineColorBlendStateCreateInfo) -> Self {
        let (attachments_count, attachments_ptr, attachments) = match create_info.attachments {
            Some(ref attachments) => {
                let attachments: Vec<_> = attachments.iter().map(From::from).collect();
                (attachments.len() as u32, attachments.as_ptr(), Some(attachments))
            }

            None => (0, ptr::null(), None),
        };

        VkPipelineColorBlendStateCreateInfoWrapper {
            create_info: vks::VkPipelineColorBlendStateCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
                logicOpEnable: utils::to_vk_bool(create_info.logic_op_enable),
                logicOp: create_info.logic_op.into(),
                attachmentCount: attachments_count,
                pAttachments: attachments_ptr,
                blendConstants: create_info.blend_constants,
            },
            attachments: attachments,
        }
    }
}

/// See [`VkPipelineDynamicStateCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineDynamicStateCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum PipelineDynamicStateCreateInfoChainElement {
}

/// See [`VkPipelineDynamicStateCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineDynamicStateCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct PipelineDynamicStateCreateInfo {
    pub chain: Vec<PipelineDynamicStateCreateInfoChainElement>,
    pub flags: PipelineDynamicStateCreateFlags,
    pub dynamic_states: Vec<DynamicState>,
}

impl<'a> From<&'a vks::VkPipelineDynamicStateCreateInfo> for PipelineDynamicStateCreateInfo {
    fn from(create_info: &'a vks::VkPipelineDynamicStateCreateInfo) -> Self {
        assert!(create_info.pNext.is_null());

        let dynamic_states = unsafe {
            slice::from_raw_parts(create_info.pDynamicStates, create_info.dynamicStateCount as usize)
                .iter()
                .cloned()
                .map(From::from)
                .collect()
        };

        PipelineDynamicStateCreateInfo {
            chain: vec![],
            flags: create_info.flags,
            dynamic_states: dynamic_states,
        }
    }
}

#[derive(Debug)]
struct VkPipelineDynamicStateCreateInfoWrapper {
    create_info: vks::VkPipelineDynamicStateCreateInfo,
    dynamic_states: Vec<vks::VkDynamicState>,
}

impl Deref for VkPipelineDynamicStateCreateInfoWrapper {
    type Target = vks::VkPipelineDynamicStateCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vks::VkPipelineDynamicStateCreateInfo> for VkPipelineDynamicStateCreateInfoWrapper {
    fn as_ref(&self) -> &vks::VkPipelineDynamicStateCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a PipelineDynamicStateCreateInfo> for VkPipelineDynamicStateCreateInfoWrapper {
    fn from(create_info: &'a PipelineDynamicStateCreateInfo) -> Self {
        let dynamic_states: Vec<_> = create_info.dynamic_states
            .iter()
            .cloned()
            .map(From::from)
            .collect();

        VkPipelineDynamicStateCreateInfoWrapper {
            create_info: vks::VkPipelineDynamicStateCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
                dynamicStateCount: dynamic_states.len() as u32,
                pDynamicStates: dynamic_states.as_ptr(),
            },
            dynamic_states: dynamic_states,
        }
    }
}

/// See [`VkGraphicsPipelineCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkGraphicsPipelineCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum GraphicsPipelineCreateInfoChainElement {
}

/// See [`VkGraphicsPipelineCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkGraphicsPipelineCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct GraphicsPipelineCreateInfo {
    pub chain: Vec<GraphicsPipelineCreateInfoChainElement>,
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
}

#[derive(Debug)]
struct VkGraphicsPipelineCreateInfoWrapper {
    create_info: vks::VkGraphicsPipelineCreateInfo,
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
}

impl Deref for VkGraphicsPipelineCreateInfoWrapper {
    type Target = vks::VkGraphicsPipelineCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vks::VkGraphicsPipelineCreateInfo> for VkGraphicsPipelineCreateInfoWrapper {
    fn as_ref(&self) -> &vks::VkGraphicsPipelineCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a GraphicsPipelineCreateInfo> for VkGraphicsPipelineCreateInfoWrapper {
    fn from(create_info: &'a GraphicsPipelineCreateInfo) -> Self {
        let stages: Vec<_> = create_info.stages.iter().map(From::from).collect();
        let vk_stages: Vec<_> = stages.iter().map(AsRef::as_ref).cloned().collect();
        let vertex_input_state: Box<VkPipelineVertexInputStateCreateInfoWrapper> = Box::new((&create_info.vertex_input_state).into());
        let input_assembly_state: Box<VkPipelineInputAssemblyStateCreateInfoWrapper> = Box::new((&create_info.input_assembly_state).into());

        let (tessellation_state_ptr, tessellation_state) = match create_info.tessellation_state {
            Some(ref tessellation_state) => {
                let tessellation_state: Box<VkPipelineTessellationStateCreateInfoWrapper> = Box::new(tessellation_state.into());
                (&**tessellation_state as *const _, Some(tessellation_state))
            }

            None => (ptr::null(), None),
        };

        let (viewport_state_ptr, viewport_state) = match create_info.viewport_state {
            Some(ref viewport_state) => {
                let viewport_state: Box<VkPipelineViewportStateCreateInfoWrapper> = Box::new(viewport_state.into());
                (&**viewport_state as *const _, Some(viewport_state))
            }

            None => (ptr::null(), None),
        };

        let rasterization_state: Box<VkPipelineRasterizationStateCreateInfoWrapper> = Box::new((&create_info.rasterization_state).into());

        let (multisample_state_ptr, multisample_state) = match create_info.multisample_state {
            Some(ref multisample_state) => {
                let multisample_state: Box<VkPipelineMultisampleStateCreateInfoWrapper> = Box::new(multisample_state.into());
                (&**multisample_state as *const _, Some(multisample_state))
            }

            None => (ptr::null(), None),
        };

        let (depth_stencil_state_ptr, depth_stencil_state) = match create_info.depth_stencil_state {
            Some(ref depth_stencil_state) => {
                let depth_stencil_state: Box<VkPipelineDepthStencilStateCreateInfoWrapper> = Box::new(depth_stencil_state.into());
                (&**depth_stencil_state as *const _, Some(depth_stencil_state))
            }

            None => (ptr::null(), None),
        };

        let (color_blend_state_ptr, color_blend_state) = match create_info.color_blend_state {
            Some(ref color_blend_state) => {
                let color_blend_state: Box<VkPipelineColorBlendStateCreateInfoWrapper> = Box::new(color_blend_state.into());
                (&**color_blend_state as *const _, Some(color_blend_state))
            }

            None => (ptr::null(), None),
        };

        let (dynamic_state_ptr, dynamic_state) = match create_info.dynamic_state {
            Some(ref dynamic_state) => {
                let dynamic_state: Box<VkPipelineDynamicStateCreateInfoWrapper> = Box::new(dynamic_state.into());
                (&**dynamic_state as *const _, Some(dynamic_state))
            }

            None => (ptr::null(), None),
        };

        let (base_pipeline_handle, base_pipeline) = match create_info.base_pipeline {
            Some(ref base_pipeline) => (base_pipeline.handle(), Some(base_pipeline.clone())),
            None => (ptr::null_mut(), None),
        };

        let base_pipeline_index = match create_info.base_pipeline_index {
            Some(base_pipeline_index) => base_pipeline_index as i32,
            None => -1,
        };

        VkGraphicsPipelineCreateInfoWrapper {
            create_info: vks::VkGraphicsPipelineCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
                stageCount: stages.len() as u32,
                pStages: vk_stages.as_ptr(),
                pVertexInputState: &**vertex_input_state,
                pInputAssemblyState: &**input_assembly_state,
                pTessellationState: tessellation_state_ptr,
                pViewportState: viewport_state_ptr,
                pRasterizationState: &**rasterization_state,
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
        }
    }
}

/// See [`VkComputePipelineCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkComputePipelineCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum ComputePipelineCreateInfoChainElement {
}

/// See [`VkComputePipelineCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkComputePipelineCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct ComputePipelineCreateInfo {
    pub chain: Vec<ComputePipelineCreateInfoChainElement>,
    pub flags: PipelineCreateFlags,
    pub stage: PipelineShaderStageCreateInfo,
    pub layout: PipelineLayout,
    pub base_pipeline: Option<Pipeline>,
    pub base_pipeline_index: Option<u32>,
}

#[derive(Debug)]
struct VkComputePipelineCreateInfoWrapper {
    create_info: vks::VkComputePipelineCreateInfo,
    stage: VkPipelineShaderStageCreateInfoWrapper,
    layout: PipelineLayout,
    base_pipeline: Option<Pipeline>,
}

impl Deref for VkComputePipelineCreateInfoWrapper {
    type Target = vks::VkComputePipelineCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vks::VkComputePipelineCreateInfo> for VkComputePipelineCreateInfoWrapper {
    fn as_ref(&self) -> &vks::VkComputePipelineCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a ComputePipelineCreateInfo> for VkComputePipelineCreateInfoWrapper {
    fn from(create_info: &'a ComputePipelineCreateInfo) -> Self {
        let stage: VkPipelineShaderStageCreateInfoWrapper = (&create_info.stage).into();

        let (base_pipeline_handle, base_pipeline) = match create_info.base_pipeline {
            Some(ref base_pipeline) => (base_pipeline.handle(), Some(base_pipeline.clone())),
            None => (ptr::null_mut(), None),
        };

        let base_pipeline_index = match create_info.base_pipeline_index {
            Some(base_pipeline_index) => base_pipeline_index as i32,
            None => -1,
        };

        VkComputePipelineCreateInfoWrapper {
            create_info: vks::VkComputePipelineCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
                stage: *stage,
                layout: create_info.layout.handle(),
                basePipelineHandle: base_pipeline_handle,
                basePipelineIndex: base_pipeline_index,
            },
            stage: stage,
            layout: create_info.layout.clone(),
            base_pipeline: base_pipeline,
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

impl<'a> From<&'a vks::VkPushConstantRange> for PushConstantRange {
    fn from(range: &'a vks::VkPushConstantRange) -> Self {
        PushConstantRange {
            stage_flags: range.stageFlags,
            offset: range.offset,
            size: range.size,
        }
    }
}

impl<'a> From<&'a PushConstantRange> for vks::VkPushConstantRange {
    fn from(range: &'a PushConstantRange) -> Self {
        vks::VkPushConstantRange {
            stageFlags: range.stage_flags,
            offset: range.offset,
            size: range.size,
        }
    }
}

/// See [`VkPipelineLayoutCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineLayoutCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum PipelineLayoutCreateInfoChainElement {
}

/// See [`VkPipelineLayoutCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPipelineLayoutCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct PipelineLayoutCreateInfo {
    pub chain: Vec<PipelineLayoutCreateInfoChainElement>,
    pub flags: PipelineLayoutCreateFlags,
    pub set_layouts: Option<Vec<DescriptorSetLayout>>,
    pub push_constant_ranges: Option<Vec<PushConstantRange>>,
}

#[derive(Debug)]
struct VkPipelineLayoutCreateInfoWrapper {
    create_info: vks::VkPipelineLayoutCreateInfo,
    set_layouts: Option<Vec<DescriptorSetLayout>>,
    vk_set_layouts: Option<Vec<vks::VkDescriptorSetLayout>>,
    push_constant_ranges: Option<Vec<vks::VkPushConstantRange>>,
}

impl Deref for VkPipelineLayoutCreateInfoWrapper {
    type Target = vks::VkPipelineLayoutCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vks::VkPipelineLayoutCreateInfo> for VkPipelineLayoutCreateInfoWrapper {
    fn as_ref(&self) -> &vks::VkPipelineLayoutCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a PipelineLayoutCreateInfo> for VkPipelineLayoutCreateInfoWrapper {
    fn from(create_info: &'a PipelineLayoutCreateInfo) -> Self {
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

        VkPipelineLayoutCreateInfoWrapper {
            create_info: vks::VkPipelineLayoutCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
                setLayoutCount: set_layout_count,
                pSetLayouts: vk_set_layouts_ptr,
                pushConstantRangeCount: push_constant_ranges_count,
                pPushConstantRanges: push_constant_ranges_ptr,
            },
            set_layouts: set_layouts,
            vk_set_layouts: vk_set_layouts,
            push_constant_ranges: push_constant_ranges,
        }
    }
}

/// See [`VkSamplerCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSamplerCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum SamplerCreateInfoChainElement {
}

/// See [`VkSamplerCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkSamplerCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct SamplerCreateInfo {
    pub chain: Vec<SamplerCreateInfoChainElement>,
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
}

impl<'a> From<&'a vks::VkSamplerCreateInfo> for SamplerCreateInfo {
    fn from(create_info: &'a vks::VkSamplerCreateInfo) -> Self {
        debug_assert_eq!(create_info.pNext, ptr::null());

        SamplerCreateInfo {
            chain: vec![],
            flags: create_info.flags,
            mag_filter: create_info.magFilter.into(),
            min_filter: create_info.minFilter.into(),
            mipmap_mode: create_info.mipmapMode.into(),
            address_mode_u: create_info.addressModeU.into(),
            address_mode_v: create_info.addressModeV.into(),
            address_mode_w: create_info.addressModeW.into(),
            mip_lod_bias: create_info.mipLodBias,
            anisotropy_enable: utils::from_vk_bool(create_info.anisotropyEnable),
            max_anisotropy: create_info.maxAnisotropy,
            compare_enable: utils::from_vk_bool(create_info.compareEnable),
            compare_op: create_info.compareOp.into(),
            min_lod: create_info.minLod,
            max_lod: create_info.maxLod,
            border_color: create_info.borderColor.into(),
            unnormalized_coordinates: utils::from_vk_bool(create_info.unnormalizedCoordinates),
        }
    }
}

#[derive(Debug)]
struct VkSamplerCreateInfoWrapper {
    create_info: vks::VkSamplerCreateInfo,
}

impl Deref for VkSamplerCreateInfoWrapper {
    type Target = vks::VkSamplerCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vks::VkSamplerCreateInfo> for VkSamplerCreateInfoWrapper {
    fn as_ref(&self) -> &vks::VkSamplerCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a SamplerCreateInfo> for VkSamplerCreateInfoWrapper {
    fn from(create_info: &'a SamplerCreateInfo) -> Self {
        VkSamplerCreateInfoWrapper {
            create_info: vks::VkSamplerCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
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
    binding: vks::VkDescriptorSetLayoutBinding,
    immutable_samplers: Option<Vec<Sampler>>,
    immutable_vk_samplers: Option<Vec<vks::VkSampler>>,
}

impl Deref for VkDescriptorSetLayoutBindingWrapper {
    type Target = vks::VkDescriptorSetLayoutBinding;

    fn deref(&self) -> &Self::Target {
        &self.binding
    }
}

impl AsRef<vks::VkDescriptorSetLayoutBinding> for VkDescriptorSetLayoutBindingWrapper {
    fn as_ref(&self) -> &vks::VkDescriptorSetLayoutBinding {
        &self.binding
    }
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
            binding: vks::VkDescriptorSetLayoutBinding {
                binding: binding.binding,
                descriptorType: binding.descriptor_type.into(),
                descriptorCount: binding.descriptor_count,
                stageFlags: binding.stage_flags,
                pImmutableSamplers: immutable_vk_samplers_ptr,
            },
            immutable_samplers: immutable_samplers,
            immutable_vk_samplers: immutable_vk_samplers,
        }
    }
}

/// See [`VkDescriptorSetLayoutCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorSetLayoutCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum DescriptorSetLayoutCreateInfoChainElement {
}

/// See [`VkDescriptorSetLayoutCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorSetLayoutCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct DescriptorSetLayoutCreateInfo {
    pub chain: Vec<DescriptorSetLayoutCreateInfoChainElement>,
    pub flags: DescriptorSetLayoutCreateFlags,
    pub bindings: Option<Vec<DescriptorSetLayoutBinding>>,
}

#[derive(Debug)]
struct VkDescriptorSetLayoutCreateInfoWrapper {
    create_info: vks::VkDescriptorSetLayoutCreateInfo,
    bindings: Option<Vec<VkDescriptorSetLayoutBindingWrapper>>,
    vk_bindings: Option<Vec<vks::VkDescriptorSetLayoutBinding>>,
}

impl Deref for VkDescriptorSetLayoutCreateInfoWrapper {
    type Target = vks::VkDescriptorSetLayoutCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vks::VkDescriptorSetLayoutCreateInfo> for VkDescriptorSetLayoutCreateInfoWrapper {
    fn as_ref(&self) -> &vks::VkDescriptorSetLayoutCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a DescriptorSetLayoutCreateInfo> for VkDescriptorSetLayoutCreateInfoWrapper {
    fn from(create_info: &'a DescriptorSetLayoutCreateInfo) -> Self {
        let (vk_bindings_ptr, binding_count, bindings, vk_bindings) = match create_info.bindings {
            Some(ref bindings) => {
                let bindings: Vec<_> = bindings.iter().map(From::from).collect();
                let vk_bindings: Vec<_> = bindings.iter().map(AsRef::as_ref).cloned().collect();
                (vk_bindings.as_ptr(), bindings.len() as u32, Some(bindings), Some(vk_bindings))
            }

            None => (ptr::null(), 0, None, None),
        };

        VkDescriptorSetLayoutCreateInfoWrapper {
            create_info: vks::VkDescriptorSetLayoutCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
                bindingCount: binding_count,
                pBindings: vk_bindings_ptr,
            },
            bindings: bindings,
            vk_bindings: vk_bindings,
        }
    }
}

/// See [`VkDescriptorPoolSize`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorPoolSize)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DescriptorPoolSize {
    pub descriptor_type: DescriptorType,
    pub descriptor_count: u32,
}

impl<'a> From<&'a vks::VkDescriptorPoolSize> for DescriptorPoolSize {
    fn from(size: &'a vks::VkDescriptorPoolSize) -> Self {
        DescriptorPoolSize {
            descriptor_type: size.type_.into(),
            descriptor_count: size.descriptorCount,
        }
    }
}

impl<'a> From<&'a DescriptorPoolSize> for vks::VkDescriptorPoolSize {
    fn from(size: &'a DescriptorPoolSize) -> Self {
        vks::VkDescriptorPoolSize {
            type_: size.descriptor_type.into(),
            descriptorCount: size.descriptor_count,
        }
    }
}

/// See [`VkDescriptorPoolCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorPoolCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum DescriptorPoolCreateInfoChainElement {
}

/// See [`VkDescriptorPoolCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorPoolCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct DescriptorPoolCreateInfo {
    pub chain: Vec<DescriptorPoolCreateInfoChainElement>,
    pub flags: DescriptorPoolCreateFlags,
    pub max_sets: u32,
    pub pool_sizes: Vec<DescriptorPoolSize>,
}

impl<'a> From<&'a vks::VkDescriptorPoolCreateInfo> for DescriptorPoolCreateInfo {
    fn from(create_info: &'a vks::VkDescriptorPoolCreateInfo) -> Self {
        debug_assert_eq!(create_info.pNext, ptr::null());

        let pool_sizes = unsafe {
            slice::from_raw_parts(create_info.pPoolSizes, create_info.poolSizeCount as usize)
                .iter()
                .map(From::from)
                .collect()
        };

        DescriptorPoolCreateInfo {
            chain: vec![],
            flags: create_info.flags,
            max_sets: create_info.maxSets,
            pool_sizes: pool_sizes,
        }
    }
}

#[derive(Debug)]
struct VkDescriptorPoolCreateInfoWrapper {
    create_info: vks::VkDescriptorPoolCreateInfo,
    pool_sizes: Vec<vks::VkDescriptorPoolSize>,
}

impl Deref for VkDescriptorPoolCreateInfoWrapper {
    type Target = vks::VkDescriptorPoolCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vks::VkDescriptorPoolCreateInfo> for VkDescriptorPoolCreateInfoWrapper {
    fn as_ref(&self) -> &vks::VkDescriptorPoolCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a DescriptorPoolCreateInfo> for VkDescriptorPoolCreateInfoWrapper {
    fn from(create_info: &'a DescriptorPoolCreateInfo) -> Self {
        let pool_sizes: Vec<_> = create_info.pool_sizes
            .iter()
            .map(From::from)
            .collect();

        VkDescriptorPoolCreateInfoWrapper {
            create_info: vks::VkDescriptorPoolCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
                maxSets: create_info.max_sets,
                poolSizeCount: pool_sizes.len() as u32,
                pPoolSizes: pool_sizes.as_ptr(),
            },
            pool_sizes: pool_sizes,
        }
    }
}

/// See [`VkDescriptorSetAllocateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorSetAllocateInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum DescriptorSetAllocateInfoChainElement {
}

/// See [`VkDescriptorSetAllocateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDescriptorSetAllocateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct DescriptorSetAllocateInfo {
    pub chain: Vec<DescriptorSetAllocateInfoChainElement>,
    pub descriptor_pool: DescriptorPool,
    pub set_layouts: Vec<DescriptorSetLayout>,
}

#[derive(Debug)]
struct VkDescriptorSetAllocateInfoWrapper {
    allocate_info: vks::VkDescriptorSetAllocateInfo,
    descriptor_pool: DescriptorPool,
    set_layouts: Vec<DescriptorSetLayout>,
    vk_set_layouts: Vec<vks::VkDescriptorSetLayout>,
}

impl Deref for VkDescriptorSetAllocateInfoWrapper {
    type Target = vks::VkDescriptorSetAllocateInfo;

    fn deref(&self) -> &Self::Target {
        &self.allocate_info
    }
}

impl AsRef<vks::VkDescriptorSetAllocateInfo> for VkDescriptorSetAllocateInfoWrapper {
    fn as_ref(&self) -> &vks::VkDescriptorSetAllocateInfo {
        &self.allocate_info
    }
}

impl<'a> From<&'a DescriptorSetAllocateInfo> for VkDescriptorSetAllocateInfoWrapper {
    fn from(allocate_info: &'a DescriptorSetAllocateInfo) -> Self {
        let vk_set_layouts: Vec<_> = allocate_info.set_layouts.iter().map(DescriptorSetLayout::handle).collect();

        VkDescriptorSetAllocateInfoWrapper {
            allocate_info: vks::VkDescriptorSetAllocateInfo {
                sType: vks::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,
                pNext: ptr::null(),
                descriptorPool: allocate_info.descriptor_pool.handle(),
                descriptorSetCount: vk_set_layouts.len() as u32,
                pSetLayouts: vk_set_layouts.as_ptr(),
            },
            descriptor_pool: allocate_info.descriptor_pool.clone(),
            set_layouts: allocate_info.set_layouts.clone(),
            vk_set_layouts: vk_set_layouts,
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
    info: vks::VkDescriptorImageInfo,
    sampler: Option<Sampler>,
    image_view: Option<ImageView>,
}

impl Deref for VkDescriptorImageInfoWrapper {
    type Target = vks::VkDescriptorImageInfo;

    fn deref(&self) -> &Self::Target {
        &self.info
    }
}

impl AsRef<vks::VkDescriptorImageInfo> for VkDescriptorImageInfoWrapper {
    fn as_ref(&self) -> &vks::VkDescriptorImageInfo {
        &self.info
    }
}

impl<'a> From<&'a DescriptorImageInfo> for VkDescriptorImageInfoWrapper {
    fn from(info: &'a DescriptorImageInfo) -> Self {
        let (vk_sampler, sampler) = match info.sampler {
            Some(ref sampler) => (sampler.handle(), Some(sampler.clone())),
            None => (ptr::null_mut(), None),
        };

        let (vk_image_view, image_view) = match info.image_view {
            Some(ref image_view) => (image_view.handle(), Some(image_view.clone())),
            None => (ptr::null_mut(), None),
        };

        VkDescriptorImageInfoWrapper {
            info: vks::VkDescriptorImageInfo {
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
    info: vks::VkDescriptorBufferInfo,
    buffer: Buffer,
}

impl Deref for VkDescriptorBufferInfoWrapper {
    type Target = vks::VkDescriptorBufferInfo;

    fn deref(&self) -> &Self::Target {
        &self.info
    }
}

impl AsRef<vks::VkDescriptorBufferInfo> for VkDescriptorBufferInfoWrapper {
    fn as_ref(&self) -> &vks::VkDescriptorBufferInfo {
        &self.info
    }
}

impl<'a> From<&'a DescriptorBufferInfo> for VkDescriptorBufferInfoWrapper {
    fn from(info: &'a DescriptorBufferInfo) -> Self {
        VkDescriptorBufferInfoWrapper {
            info: vks::VkDescriptorBufferInfo {
                buffer: info.buffer.handle(),
                offset: info.offset,
                range: info.range.into(),
            },
            buffer: info.buffer.clone(),
        }
    }
}

/// See [`VkWriteDescriptorSet`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkWriteDescriptorSet)
#[derive(Debug, Clone, PartialEq)]
pub enum WriteDescriptorSetChainElement {
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
    pub chain: Vec<WriteDescriptorSetChainElement>,
    pub dst_set: DescriptorSet,
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_type: DescriptorType,
    pub elements: WriteDescriptorSetElements,
}

#[derive(Debug)]
struct VkWriteDescriptorSetWrapper {
    write: vks::VkWriteDescriptorSet,
    dst_set: DescriptorSet,
    image_info: Option<Vec<VkDescriptorImageInfoWrapper>>,
    vk_image_info: Option<Vec<vks::VkDescriptorImageInfo>>,
    buffer_info: Option<Vec<VkDescriptorBufferInfoWrapper>>,
    vk_buffer_info: Option<Vec<vks::VkDescriptorBufferInfo>>,
    texel_buffer_view: Option<Vec<BufferView>>,
    vk_texel_buffer_view: Option<Vec<vks::VkBufferView>>,
}

impl Deref for VkWriteDescriptorSetWrapper {
    type Target = vks::VkWriteDescriptorSet;

    fn deref(&self) -> &Self::Target {
        &self.write
    }
}

impl AsRef<vks::VkWriteDescriptorSet> for VkWriteDescriptorSetWrapper {
    fn as_ref(&self) -> &vks::VkWriteDescriptorSet {
        &self.write
    }
}

impl<'a> From<&'a WriteDescriptorSet> for VkWriteDescriptorSetWrapper {
    fn from(write: &'a WriteDescriptorSet) -> Self {
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
                let vk_image_info: Vec<_> = image_info.iter().map(AsRef::as_ref).cloned().collect();
                (image_info.len() as u32, vk_image_info.as_ptr(), Some(vk_image_info), Some(image_info), ptr::null(), None, None, ptr::null(), None, None)
            },

            WriteDescriptorSetElements::BufferInfo(ref buffer_info) => {
                let buffer_info: Vec<VkDescriptorBufferInfoWrapper> = buffer_info.iter().map(From::from).collect();
                let vk_buffer_info: Vec<_> = buffer_info.iter().map(AsRef::as_ref).cloned().collect();
                (buffer_info.len() as u32, ptr::null(), None, None, vk_buffer_info.as_ptr(), Some(vk_buffer_info), Some(buffer_info), ptr::null(), None, None)
            },

            WriteDescriptorSetElements::TexelBufferView(ref texel_buffer_view) => {
                let texel_buffer_view = texel_buffer_view.clone();
                let vk_texel_buffer_view: Vec<_> = texel_buffer_view.iter().map(BufferView::handle).collect();
                (texel_buffer_view.len() as u32, ptr::null(), None, None, ptr::null(), None, None, vk_texel_buffer_view.as_ptr(), Some(vk_texel_buffer_view), Some(texel_buffer_view))
            },
        };

        VkWriteDescriptorSetWrapper {
            write: vks::VkWriteDescriptorSet {
                sType: vks::VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
                pNext: ptr::null(),
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
        }
    }
}

/// See [`VkCopyDescriptorSet`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCopyDescriptorSet)
#[derive(Debug, Clone, PartialEq)]
pub enum CopyDescriptorSetChainElement {
}

/// See [`VkCopyDescriptorSet`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCopyDescriptorSet)
#[derive(Debug, Clone, PartialEq)]
pub struct CopyDescriptorSet {
    pub chain: Vec<CopyDescriptorSetChainElement>,
    pub src_set: DescriptorSet,
    pub src_binding: u32,
    pub src_array_element: u32,
    pub dst_set: DescriptorSet,
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_count: u32,
}

#[derive(Debug)]
struct VkCopyDescriptorSetWrapper {
    copy: vks::VkCopyDescriptorSet,
    src_set: DescriptorSet,
    dst_set: DescriptorSet,
}

impl Deref for VkCopyDescriptorSetWrapper {
    type Target = vks::VkCopyDescriptorSet;

    fn deref(&self) -> &Self::Target {
        &self.copy
    }
}

impl AsRef<vks::VkCopyDescriptorSet> for VkCopyDescriptorSetWrapper {
    fn as_ref(&self) -> &vks::VkCopyDescriptorSet {
        &self.copy
    }
}

impl<'a> From<&'a CopyDescriptorSet> for VkCopyDescriptorSetWrapper {
    fn from(copy: &'a CopyDescriptorSet) -> Self {
        VkCopyDescriptorSetWrapper {
            copy: vks::VkCopyDescriptorSet {
                sType: vks::VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET,
                pNext: ptr::null(),
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
        }
    }
}

/// See [`VkFramebufferCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFramebufferCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum FramebufferCreateInfoChainElement {
}

/// See [`VkFramebufferCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkFramebufferCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct FramebufferCreateInfo {
    pub chain: Vec<FramebufferCreateInfoChainElement>,
    pub flags: FramebufferCreateFlags,
    pub render_pass: RenderPass,
    pub attachments: Option<Vec<ImageView>>,
    pub width: u32,
    pub height: u32,
    pub layers: u32,
}

#[derive(Debug)]
struct VkFramebufferCreateInfoWrapper {
    create_info: vks::VkFramebufferCreateInfo,
    render_pass: RenderPass,
    attachments: Option<Vec<ImageView>>,
    vk_attachments: Option<Vec<vks::VkImageView>>,
}

impl Deref for VkFramebufferCreateInfoWrapper {
    type Target = vks::VkFramebufferCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vks::VkFramebufferCreateInfo> for VkFramebufferCreateInfoWrapper {
    fn as_ref(&self) -> &vks::VkFramebufferCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a FramebufferCreateInfo> for VkFramebufferCreateInfoWrapper {
    fn from(create_info: &'a FramebufferCreateInfo) -> Self {
        let (attachments_count, vk_attachments_ptr, attachments, vk_attachments) = match create_info.attachments {
            Some(ref attachments) => {
                let attachments = attachments.clone();
                let vk_attachments: Vec<_> = attachments.iter().map(ImageView::handle).collect();
                (attachments.len() as u32, vk_attachments.as_ptr(), Some(attachments), Some(vk_attachments))
            }

            None => (0, ptr::null(), None, None),
        };

        VkFramebufferCreateInfoWrapper {
            create_info: vks::VkFramebufferCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
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

impl<'a> From<&'a vks::VkAttachmentDescription> for AttachmentDescription {
    fn from(description: &'a vks::VkAttachmentDescription) -> Self {
        AttachmentDescription {
            flags: description.flags,
            format: description.format.into(),
            samples: description.samples,
            load_op: description.loadOp.into(),
            store_op: description.storeOp.into(),
            stencil_load_op: description.stencilLoadOp.into(),
            stencil_store_op: description.stencilStoreOp.into(),
            initial_layout: description.initialLayout.into(),
            final_layout: description.finalLayout.into(),
        }
    }
}

impl<'a> From<&'a AttachmentDescription> for vks::VkAttachmentDescription {
    fn from(description: &'a AttachmentDescription) -> Self {
        vks::VkAttachmentDescription {
            flags: description.flags,
            format: description.format.into(),
            samples: description.samples,
            loadOp: description.load_op.into(),
            storeOp: description.store_op.into(),
            stencilLoadOp: description.stencil_load_op.into(),
            stencilStoreOp: description.stencil_store_op.into(),
            initialLayout: description.initial_layout.into(),
            finalLayout: description.final_layout.into(),
        }
    }
}

/// See [`VkAttachmentReference`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkAttachmentReference)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct AttachmentReference {
    pub attachment: AttachmentIndex,
    pub layout: ImageLayout,
}

impl<'a> From<&'a vks::VkAttachmentReference> for AttachmentReference {
    fn from(reference: &'a vks::VkAttachmentReference) -> Self {
        AttachmentReference {
            attachment: reference.attachment.into(),
            layout: reference.layout.into(),
        }
    }
}

impl<'a> From<&'a AttachmentReference> for vks::VkAttachmentReference {
    fn from(reference: &'a AttachmentReference) -> Self {
        vks::VkAttachmentReference {
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
            unsafe {
                Some(slice::from_raw_parts(description.pInputAttachments, description.inputAttachmentCount as usize)
                     .iter()
                     .map(From::from)
                     .collect())
            }
        }
        else {
            None
        };

        let color_attachments = if !description.pColorAttachments.is_null() {
            unsafe {
                Some(slice::from_raw_parts(description.pColorAttachments, description.colorAttachmentCount as usize)
                     .iter()
                     .map(From::from)
                     .collect())
            }
        }
        else {
            None
        };

        let resolve_attachments = if !description.pResolveAttachments.is_null() {
            unsafe {
                Some(slice::from_raw_parts(description.pResolveAttachments, description.colorAttachmentCount as usize)
                     .iter()
                     .map(From::from)
                     .collect())
            }
        }
        else {
            None
        };

        let depth_stencil_attachment = if !description.pDepthStencilAttachment.is_null() {
            unsafe {
                Some((&*description.pDepthStencilAttachment).into())
            }
        }
        else {
            None
        };

        let preserve_attachments = if !description.pPreserveAttachments.is_null() {
            unsafe {
                Some(slice::from_raw_parts(description.pPreserveAttachments, description.preserveAttachmentCount as usize).to_vec())
            }
        }
        else {
            None
        };

        SubpassDescription {
            flags: description.flags,
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
    description: vks::VkSubpassDescription,
    input_attachments: Option<Vec<vks::VkAttachmentReference>>,
    color_attachments: Option<Vec<vks::VkAttachmentReference>>,
    resolve_attachments: Option<Vec<vks::VkAttachmentReference>>,
    depth_stencil_attachment: Option<Box<vks::VkAttachmentReference>>,
    preserve_attachments: Option<Vec<u32>>,
}

impl Deref for VkSubpassDescriptionWrapper {
    type Target = vks::VkSubpassDescription;

    fn deref(&self) -> &Self::Target {
        &self.description
    }
}

impl AsRef<vks::VkSubpassDescription> for VkSubpassDescriptionWrapper {
    fn as_ref(&self) -> &vks::VkSubpassDescription {
        &self.description
    }
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
            description: vks::VkSubpassDescription {
                flags: description.flags,
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

impl<'a> From<&'a vks::VkSubpassDependency> for SubpassDependency {
    fn from(dependency: &'a vks::VkSubpassDependency) -> Self {
        SubpassDependency {
            src_subpass: dependency.srcSubpass.into(),
            dst_subpass: dependency.dstSubpass.into(),
            src_stage_mask: dependency.srcStageMask,
            dst_stage_mask: dependency.dstStageMask,
            src_access_mask: dependency.srcAccessMask,
            dst_access_mask: dependency.dstAccessMask,
            dependency_flags: dependency.dependencyFlags,
        }
    }
}

impl<'a> From<&'a SubpassDependency> for vks::VkSubpassDependency {
    fn from(dependency: &'a SubpassDependency) -> Self {
        vks::VkSubpassDependency {
            srcSubpass: dependency.src_subpass.into(),
            dstSubpass: dependency.dst_subpass.into(),
            srcStageMask: dependency.src_stage_mask,
            dstStageMask: dependency.dst_stage_mask,
            srcAccessMask: dependency.src_access_mask,
            dstAccessMask: dependency.dst_access_mask,
            dependencyFlags: dependency.dependency_flags,
        }
    }
}

/// See [`VkRenderPassCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkRenderPassCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum RenderPassCreateInfoChainElement {
}

/// See [`VkRenderPassCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkRenderPassCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct RenderPassCreateInfo {
    pub chain: Vec<RenderPassCreateInfoChainElement>,
    pub flags: RenderPassCreateFlags,
    pub attachments: Option<Vec<AttachmentDescription>>,
    pub subpasses: Vec<SubpassDescription>,
    pub dependencies: Option<Vec<SubpassDependency>>,
}

impl<'a> From<&'a vks::VkRenderPassCreateInfo> for RenderPassCreateInfo {
    fn from(create_info: &'a vks::VkRenderPassCreateInfo) -> Self {
        assert!(create_info.pNext.is_null());

        let attachments = if !create_info.pAttachments.is_null() {
            unsafe {
                Some(slice::from_raw_parts(create_info.pAttachments, create_info.attachmentCount as usize)
                     .iter()
                     .map(From::from)
                     .collect())
            }
        }
        else {
            None
        };

        let subpasses = unsafe {
            slice::from_raw_parts(create_info.pSubpasses, create_info.subpassCount as usize)
                .iter()
                .map(From::from)
                .collect()
        };

        let dependencies = if !create_info.pDependencies.is_null() {
            unsafe {
                Some(slice::from_raw_parts(create_info.pDependencies, create_info.dependencyCount as usize)
                     .iter()
                     .map(From::from)
                     .collect())
            }
        }
        else {
            None
        };

        RenderPassCreateInfo {
            chain: vec![],
            flags: create_info.flags,
            attachments: attachments,
            subpasses: subpasses,
            dependencies: dependencies,
        }
    }
}

#[derive(Debug)]
struct VkRenderPassCreateInfoWrapper {
    create_info: vks::VkRenderPassCreateInfo,
    attachments: Option<Vec<vks::VkAttachmentDescription>>,
    subpasses: Vec<VkSubpassDescriptionWrapper>,
    vk_subpasses: Vec<vks::VkSubpassDescription>,
    dependencies: Option<Vec<vks::VkSubpassDependency>>,
}

impl Deref for VkRenderPassCreateInfoWrapper {
    type Target = vks::VkRenderPassCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vks::VkRenderPassCreateInfo> for VkRenderPassCreateInfoWrapper {
    fn as_ref(&self) -> &vks::VkRenderPassCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a RenderPassCreateInfo> for VkRenderPassCreateInfoWrapper {
    fn from(create_info: &'a RenderPassCreateInfo) -> VkRenderPassCreateInfoWrapper {
        let (attachments_count, attachments_ptr, attachments) = match create_info.attachments {
            Some(ref attachments) => {
                let attachments: Vec<_> = attachments.iter().map(From::from).collect();
                (attachments.len() as u32, attachments.as_ptr(), Some(attachments))
            }

            None => (0, ptr::null(), None),
        };

        let subpasses: Vec<VkSubpassDescriptionWrapper> = create_info.subpasses.iter().map(From::from).collect();
        let vk_subpasses: Vec<vks::VkSubpassDescription> = subpasses.iter().map(AsRef::as_ref).cloned().collect();

        let (dependencies_count, dependencies_ptr, dependencies) = match create_info.dependencies {
            Some(ref dependencies) => {
                let dependencies: Vec<_> = dependencies.iter().map(From::from).collect();
                (dependencies.len() as u32, dependencies.as_ptr(), Some(dependencies))
            }

            None => (0, ptr::null(), None),
        };

        VkRenderPassCreateInfoWrapper {
            create_info: vks::VkRenderPassCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
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
        }
    }
}

/// See [`VkCommandPoolCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandPoolCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum CommandPoolCreateInfoChainElement {
}

/// See [`VkCommandPoolCreateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandPoolCreateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct CommandPoolCreateInfo {
    pub chain: Vec<CommandPoolCreateInfoChainElement>,
    pub flags: CommandPoolCreateFlags,
    pub queue_family_index: u32,
}

impl<'a> From<&'a vks::VkCommandPoolCreateInfo> for CommandPoolCreateInfo {
    fn from(create_info: &'a vks::VkCommandPoolCreateInfo) -> Self {
        debug_assert_eq!(create_info.pNext, ptr::null());

        CommandPoolCreateInfo {
            chain: vec![],
            flags: create_info.flags,
            queue_family_index: create_info.queueFamilyIndex,
        }
    }
}

#[derive(Debug)]
struct VkCommandPoolCreateInfoWrapper {
    create_info: vks::VkCommandPoolCreateInfo,
}

impl Deref for VkCommandPoolCreateInfoWrapper {
    type Target = vks::VkCommandPoolCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vks::VkCommandPoolCreateInfo> for VkCommandPoolCreateInfoWrapper {
    fn as_ref(&self) -> &vks::VkCommandPoolCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a CommandPoolCreateInfo> for VkCommandPoolCreateInfoWrapper {
    fn from(create_info: &'a CommandPoolCreateInfo) -> Self {
        VkCommandPoolCreateInfoWrapper {
            create_info: vks::VkCommandPoolCreateInfo {
                sType: vks::VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
                queueFamilyIndex: create_info.queue_family_index,
            },
        }
    }
}

/// See [`VkCommandBufferAllocateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferAllocateInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum CommandBufferAllocateInfoChainElement {
}

/// See [`VkCommandBufferAllocateInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferAllocateInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct CommandBufferAllocateInfo {
    pub chain: Vec<CommandBufferAllocateInfoChainElement>,
    pub command_pool: CommandPool,
    pub level: CommandBufferLevel,
    pub command_buffer_count: u32,
}

#[derive(Debug)]
struct VkCommandBufferAllocateInfoWrapper {
    info: vks::VkCommandBufferAllocateInfo,
    command_pool: CommandPool,
}

impl Deref for VkCommandBufferAllocateInfoWrapper {
    type Target = vks::VkCommandBufferAllocateInfo;

    fn deref(&self) -> &Self::Target {
        &self.info
    }
}

impl AsRef<vks::VkCommandBufferAllocateInfo> for VkCommandBufferAllocateInfoWrapper {
    fn as_ref(&self) -> &vks::VkCommandBufferAllocateInfo {
        &self.info
    }
}

impl<'a> From<&'a CommandBufferAllocateInfo> for VkCommandBufferAllocateInfoWrapper {
    fn from(info: &'a CommandBufferAllocateInfo) -> Self {
        VkCommandBufferAllocateInfoWrapper {
            info: vks::VkCommandBufferAllocateInfo {
                sType: vks::VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
                pNext: ptr::null(),
                commandPool: info.command_pool.handle(),
                level: info.level.into(),
                commandBufferCount: info.command_buffer_count,
            },
            command_pool: info.command_pool.clone(),
        }
    }
}

/// See [`VkCommandBufferInheritanceInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferInheritanceInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum CommandBufferInheritanceInfoChainElement {
}

/// See [`VkCommandBufferInheritanceInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferInheritanceInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct CommandBufferInheritanceInfo {
    pub chain: Vec<CommandBufferInheritanceInfoChainElement>,
    pub render_pass: Option<RenderPass>,
    pub subpass: u32,
    pub framebuffer: Option<Framebuffer>,
    pub occlusion_query_enable: bool,
    pub query_flags: QueryControlFlags,
    pub pipeline_statistics: QueryPipelineStatisticFlags,
}

#[derive(Debug)]
struct VkCommandBufferInheritanceInfoWrapper {
    info: vks::VkCommandBufferInheritanceInfo,
    render_pass: Option<RenderPass>,
    framebuffer: Option<Framebuffer>,
}

impl Deref for VkCommandBufferInheritanceInfoWrapper {
    type Target = vks::VkCommandBufferInheritanceInfo;

    fn deref(&self) -> &Self::Target {
        &self.info
    }
}

impl AsRef<vks::VkCommandBufferInheritanceInfo> for VkCommandBufferInheritanceInfoWrapper {
    fn as_ref(&self) -> &vks::VkCommandBufferInheritanceInfo {
        &self.info
    }
}

impl<'a> From<&'a CommandBufferInheritanceInfo> for VkCommandBufferInheritanceInfoWrapper {
    fn from(info: &'a CommandBufferInheritanceInfo) -> Self {
        let (render_pass_handle, render_pass) = match info.render_pass {
            Some(ref render_pass) => (render_pass.handle(), Some(render_pass.clone())),
            None => (ptr::null_mut(), None),
        };

        let (framebuffer_handle, framebuffer) = match info.framebuffer {
            Some(ref framebuffer) => (framebuffer.handle(), Some(framebuffer.clone())),
            None => (ptr::null_mut(), None),
        };

        VkCommandBufferInheritanceInfoWrapper {
            info: vks::VkCommandBufferInheritanceInfo {
                sType: vks::VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO,
                pNext: ptr::null(),
                renderPass: render_pass_handle,
                subpass: info.subpass,
                framebuffer: framebuffer_handle,
                occlusionQueryEnable: utils::to_vk_bool(info.occlusion_query_enable),
                queryFlags: info.query_flags,
                pipelineStatistics: info.pipeline_statistics,
            },
            render_pass: render_pass,
            framebuffer: framebuffer,
        }
    }
}

/// See [`VkCommandBufferBeginInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferBeginInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum CommandBufferBeginInfoChainElement {
}

/// See [`VkCommandBufferBeginInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBufferBeginInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct CommandBufferBeginInfo {
    pub chain: Vec<CommandBufferBeginInfoChainElement>,
    pub flags: CommandBufferUsageFlags,
    pub inheritance_info: Option<CommandBufferInheritanceInfo>,
}

#[derive(Debug)]
struct VkCommandBufferBeginInfoWrapper {
    begin_info: vks::VkCommandBufferBeginInfo,
    inheritance_info: Option<Box<VkCommandBufferInheritanceInfoWrapper>>,
}

impl Deref for VkCommandBufferBeginInfoWrapper {
    type Target = vks::VkCommandBufferBeginInfo;

    fn deref(&self) -> &Self::Target {
        &self.begin_info
    }
}

impl AsRef<vks::VkCommandBufferBeginInfo> for VkCommandBufferBeginInfoWrapper {
    fn as_ref(&self) -> &vks::VkCommandBufferBeginInfo {
        &self.begin_info
    }
}

impl<'a> From<&'a CommandBufferBeginInfo> for VkCommandBufferBeginInfoWrapper {
    fn from(begin_info: &'a CommandBufferBeginInfo) -> Self {
        let (inheritance_info_ptr, inheritance_info) = match begin_info.inheritance_info {
            Some(ref inheritance_info) => {
                let inheritance_info: Box<VkCommandBufferInheritanceInfoWrapper> = Box::new(inheritance_info.into());
                (&**inheritance_info as *const _, Some(inheritance_info))
            }

            None => (ptr::null(), None),
        };

        VkCommandBufferBeginInfoWrapper {
            begin_info: vks::VkCommandBufferBeginInfo {
                sType: vks::VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
                pNext: ptr::null(),
                flags: begin_info.flags,
                pInheritanceInfo: inheritance_info_ptr,
            },
            inheritance_info: inheritance_info,
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

impl<'a> From<&'a vks::VkBufferCopy> for BufferCopy {
    fn from(copy: &'a vks::VkBufferCopy) -> Self {
        BufferCopy {
            src_offset: copy.srcOffset,
            dst_offset: copy.dstOffset,
            size: copy.size,
        }
    }
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

impl<'a> From<&'a vks::VkImageSubresourceLayers> for ImageSubresourceLayers {
    fn from(layers: &'a vks::VkImageSubresourceLayers) -> Self {
        ImageSubresourceLayers {
            aspect_mask: layers.aspectMask,
            mip_level: layers.mipLevel,
            base_array_layer: layers.baseArrayLayer,
            layer_count: layers.layerCount,
        }
    }
}

impl<'a> From<&'a ImageSubresourceLayers> for vks::VkImageSubresourceLayers {
    fn from(layers: &'a ImageSubresourceLayers) -> Self {
        vks::VkImageSubresourceLayers {
            aspectMask: layers.aspect_mask,
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

impl<'a> From<&'a vks::VkImageCopy> for ImageCopy {
    fn from(copy: &'a vks::VkImageCopy) -> Self {
        ImageCopy {
            src_subresource: (&copy.srcSubresource).into(),
            src_offset: (&copy.srcOffset).into(),
            dst_subresource: (&copy.dstSubresource).into(),
            dst_offset: (&copy.dstOffset).into(),
            extent: (&copy.extent).into(),
        }
    }
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

impl<'a> From<&'a vks::VkImageBlit> for ImageBlit {
    fn from(blit: &'a vks::VkImageBlit) -> Self {
        let src_offsets = [(&blit.srcOffsets[0]).into(), (&blit.srcOffsets[1]).into()];
        let dst_offsets = [(&blit.dstOffsets[0]).into(), (&blit.dstOffsets[1]).into()];

        ImageBlit {
            src_subresource: (&blit.srcSubresource).into(),
            src_offsets: src_offsets,
            dst_subresource: (&blit.dstSubresource).into(),
            dst_offsets: dst_offsets,
        }
    }
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

impl<'a> From<&'a vks::VkBufferImageCopy> for BufferImageCopy {
    fn from(copy: &'a vks::VkBufferImageCopy) -> Self {
        BufferImageCopy {
            buffer_offset: copy.bufferOffset,
            buffer_row_length: copy.bufferRowLength,
            buffer_image_height: copy.bufferImageHeight,
            image_subresource: (&copy.imageSubresource).into(),
            image_offset: (&copy.imageOffset).into(),
            image_extent: (&copy.imageExtent).into(),
        }
    }
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

impl<'a> From<&'a vks::VkClearDepthStencilValue> for ClearDepthStencilValue {
    fn from(value: &'a vks::VkClearDepthStencilValue) -> Self {
        ClearDepthStencilValue {
            depth: value.depth,
            stencil: value.stencil,
        }
    }
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
            aspectMask: foobar.aspect_mask,
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

impl<'a> From<&'a vks::VkClearRect> for ClearRect {
    fn from(rect: &'a vks::VkClearRect) -> Self {
        ClearRect {
            rect: (&rect.rect).into(),
            base_array_layer: rect.baseArrayLayer,
            layer_count: rect.layerCount,
        }
    }
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

impl<'a> From<&'a vks::VkImageResolve> for ImageResolve {
    fn from(resolve: &'a vks::VkImageResolve) -> Self {
        ImageResolve {
            src_subresource: (&resolve.srcSubresource).into(),
            src_offset: (&resolve.srcOffset).into(),
            dst_subresource: (&resolve.dstSubresource).into(),
            dst_offset: (&resolve.dstOffset).into(),
            extent: (&resolve.extent).into(),
        }
    }
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

/// See [`VkMemoryBarrier`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryBarrier)
#[derive(Debug, Clone, PartialEq)]
pub enum MemoryBarrierChainElement {
}

/// See [`VkMemoryBarrier`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkMemoryBarrier)
#[derive(Debug, Clone, PartialEq)]
pub struct MemoryBarrier {
    pub chain: Vec<MemoryBarrierChainElement>,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
}

impl<'a> From<&'a vks::VkMemoryBarrier> for MemoryBarrier {
    fn from(barrier: &'a vks::VkMemoryBarrier) -> Self {
        assert!(barrier.pNext.is_null());

        MemoryBarrier {
            chain: vec![],
            src_access_mask: barrier.srcAccessMask,
            dst_access_mask: barrier.dstAccessMask,
        }
    }
}

#[derive(Debug)]
struct VkMemoryBarrierWrapper {
    barrier: vks::VkMemoryBarrier,
}

impl Deref for VkMemoryBarrierWrapper {
    type Target = vks::VkMemoryBarrier;

    fn deref(&self) -> &Self::Target {
        &self.barrier
    }
}

impl AsRef<vks::VkMemoryBarrier> for VkMemoryBarrierWrapper {
    fn as_ref(&self) -> &vks::VkMemoryBarrier {
        &self.barrier
    }
}

impl<'a> From<&'a MemoryBarrier> for VkMemoryBarrierWrapper {
    fn from(barrier: &'a MemoryBarrier) -> Self {
        VkMemoryBarrierWrapper {
            barrier: vks::VkMemoryBarrier {
                sType: vks::VK_STRUCTURE_TYPE_MEMORY_BARRIER,
                pNext: ptr::null(),
                srcAccessMask: barrier.src_access_mask,
                dstAccessMask: barrier.dst_access_mask,
            },
        }
    }
}

/// See [`VkBufferMemoryBarrier`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferMemoryBarrier)
#[derive(Debug, Clone, PartialEq)]
pub enum BufferMemoryBarrierChainElement {
}

/// See [`VkBufferMemoryBarrier`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkBufferMemoryBarrier)
#[derive(Debug, Clone, PartialEq)]
pub struct BufferMemoryBarrier {
    pub chain: Vec<BufferMemoryBarrierChainElement>,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub src_queue_family_index: QueueFamilyIndex,
    pub dst_queue_family_index: QueueFamilyIndex,
    pub buffer: Buffer,
    pub offset: u64,
    pub size: OptionalDeviceSize,
}

#[derive(Debug)]
struct VkBufferMemoryBarrierWrapper {
    barrier: vks::VkBufferMemoryBarrier,
    buffer: Buffer,
}

impl Deref for VkBufferMemoryBarrierWrapper {
    type Target = vks::VkBufferMemoryBarrier;

    fn deref(&self) -> &Self::Target {
        &self.barrier
    }
}

impl AsRef<vks::VkBufferMemoryBarrier> for VkBufferMemoryBarrierWrapper {
    fn as_ref(&self) -> &vks::VkBufferMemoryBarrier {
        &self.barrier
    }
}

impl<'a> From<&'a BufferMemoryBarrier> for VkBufferMemoryBarrierWrapper {
    fn from(barrier: &'a BufferMemoryBarrier) -> Self {
        VkBufferMemoryBarrierWrapper {
            barrier: vks::VkBufferMemoryBarrier {
                sType: vks::VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER,
                pNext: ptr::null(),
                srcAccessMask: barrier.src_access_mask,
                dstAccessMask: barrier.dst_access_mask,
                srcQueueFamilyIndex: barrier.src_queue_family_index.into(),
                dstQueueFamilyIndex: barrier.dst_queue_family_index.into(),
                buffer: barrier.buffer.handle(),
                offset: barrier.offset,
                size: barrier.size.into(),
            },
            buffer: barrier.buffer.clone(),
        }
    }
}

/// See [`VkImageMemoryBarrier`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageMemoryBarrier)
#[derive(Debug, Clone, PartialEq)]
pub enum ImageMemoryBarrierChainElement {
}

/// See [`VkImageMemoryBarrier`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkImageMemoryBarrier)
#[derive(Debug, Clone, PartialEq)]
pub struct ImageMemoryBarrier {
    pub chain: Vec<ImageMemoryBarrierChainElement>,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub old_layout: ImageLayout,
    pub new_layout: ImageLayout,
    pub src_queue_family_index: QueueFamilyIndex,
    pub dst_queue_family_index: QueueFamilyIndex,
    pub image: Image,
    pub subresource_range: ImageSubresourceRange,
}

#[derive(Debug)]
struct VkImageMemoryBarrierWrapper {
    barrier: vks::VkImageMemoryBarrier,
    image: Image,
}

impl Deref for VkImageMemoryBarrierWrapper {
    type Target = vks::VkImageMemoryBarrier;

    fn deref(&self) -> &Self::Target {
        &self.barrier
    }
}

impl AsRef<vks::VkImageMemoryBarrier> for VkImageMemoryBarrierWrapper {
    fn as_ref(&self) -> &vks::VkImageMemoryBarrier {
        &self.barrier
    }
}

impl<'a> From<&'a ImageMemoryBarrier> for VkImageMemoryBarrierWrapper {
    fn from(barrier: &'a ImageMemoryBarrier) -> Self {
        VkImageMemoryBarrierWrapper {
            barrier: vks::VkImageMemoryBarrier {
                sType: vks::VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER,
                pNext: ptr::null(),
                srcAccessMask: barrier.src_access_mask,
                dstAccessMask: barrier.dst_access_mask,
                oldLayout: barrier.old_layout.into(),
                newLayout: barrier.new_layout.into(),
                srcQueueFamilyIndex: barrier.src_queue_family_index.into(),
                dstQueueFamilyIndex: barrier.dst_queue_family_index.into(),
                image: barrier.image.handle(),
                subresourceRange: (&barrier.subresource_range).into(),
            },
            image: barrier.image.clone(),
        }
    }
}

/// See [`VkRenderPassBeginInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkRenderPassBeginInfo)
#[derive(Debug, Clone, PartialEq)]
pub enum RenderPassBeginInfoChainElement {
}

/// See [`VkRenderPassBeginInfo`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkRenderPassBeginInfo)
#[derive(Debug, Clone, PartialEq)]
pub struct RenderPassBeginInfo {
    pub chain: Vec<RenderPassBeginInfoChainElement>,
    pub render_pass: RenderPass,
    pub framebuffer: Framebuffer,
    pub render_area: Rect2D,
    pub clear_values: Option<Vec<ClearValue>>,
}

#[derive(Debug)]
struct VkRenderPassBeginInfoWrapper {
    begin_info: vks::VkRenderPassBeginInfo,
    render_pass: RenderPass,
    framebuffer: Framebuffer,
    clear_values: Option<Vec<vks::VkClearValue>>,
}

impl Deref for VkRenderPassBeginInfoWrapper {
    type Target = vks::VkRenderPassBeginInfo;

    fn deref(&self) -> &Self::Target {
        &self.begin_info
    }
}

impl AsRef<vks::VkRenderPassBeginInfo> for VkRenderPassBeginInfoWrapper {
    fn as_ref(&self) -> &vks::VkRenderPassBeginInfo {
        &self.begin_info
    }
}

impl<'a> From<&'a RenderPassBeginInfo> for VkRenderPassBeginInfoWrapper {
    fn from(begin_info: &'a RenderPassBeginInfo) -> Self {
        let (clear_values_count, clear_values_ptr, clear_values) = match begin_info.clear_values {
            Some(ref clear_values) => {
                let clear_values: Vec<_> = clear_values.iter().map(From::from).collect();
                (clear_values.len() as u32, clear_values.as_ptr(), Some(clear_values))
            }

            None => (0, ptr::null(), None),
        };

        VkRenderPassBeginInfoWrapper {
            begin_info: vks::VkRenderPassBeginInfo {
                sType: vks::VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,
                pNext: ptr::null(),
                renderPass: begin_info.render_pass.handle(),
                framebuffer: begin_info.framebuffer.handle(),
                renderArea: (&begin_info.render_area).into(),
                clearValueCount: clear_values_count,
                pClearValues: clear_values_ptr,
            },
            render_pass: begin_info.render_pass.clone(),
            framebuffer: begin_info.framebuffer.clone(),
            clear_values: clear_values,
        }
    }
}

/// See [`VkDispatchIndirectCommand`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDispatchIndirectCommand)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DispatchIndirectCommand {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl<'a> From<&'a vks::VkDispatchIndirectCommand> for DispatchIndirectCommand {
    fn from(command: &'a vks::VkDispatchIndirectCommand) -> Self {
        DispatchIndirectCommand {
            x: command.x,
            y: command.y,
            z: command.z,
        }
    }
}

impl<'a> From<&'a DispatchIndirectCommand> for vks::VkDispatchIndirectCommand {
    fn from(command: &'a DispatchIndirectCommand) -> Self {
        vks::VkDispatchIndirectCommand {
            x: command.x,
            y: command.y,
            z: command.z,
        }
    }
}

/// See [`VkDrawIndexedIndirectCommand`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDrawIndexedIndirectCommand)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DrawIndexedIndirectCommand {
    pub index_count: u32,
    pub instance_count: u32,
    pub first_index: u32,
    pub vertex_offset: i32,
    pub first_instance: u32,
}

impl<'a> From<&'a vks::VkDrawIndexedIndirectCommand> for DrawIndexedIndirectCommand {
    fn from(command: &'a vks::VkDrawIndexedIndirectCommand) -> Self {
        DrawIndexedIndirectCommand {
            index_count: command.indexCount,
            instance_count: command.instanceCount,
            first_index: command.firstIndex,
            vertex_offset: command.vertexOffset,
            first_instance: command.firstInstance,
        }
    }
}

impl<'a> From<&'a DrawIndexedIndirectCommand> for vks::VkDrawIndexedIndirectCommand {
    fn from(command: &'a DrawIndexedIndirectCommand) -> Self {
        vks::VkDrawIndexedIndirectCommand {
            indexCount: command.index_count,
            instanceCount: command.instance_count,
            firstIndex: command.first_index,
            vertexOffset: command.vertex_offset,
            firstInstance: command.first_instance,
        }
    }
}

/// See [`VkDrawIndirectCommand`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDrawIndirectCommand)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DrawIndirectCommand {
    pub vertex_count: u32,
    pub instance_count: u32,
    pub first_vertex: u32,
    pub first_instance: u32,
}

impl<'a> From<&'a vks::VkDrawIndirectCommand> for DrawIndirectCommand {
    fn from(command: &'a vks::VkDrawIndirectCommand) -> Self {
        DrawIndirectCommand {
            vertex_count: command.vertexCount,
            instance_count: command.instanceCount,
            first_vertex: command.firstVertex,
            first_instance: command.firstInstance,
        }
    }
}

impl<'a> From<&'a DrawIndirectCommand> for vks::VkDrawIndirectCommand {
    fn from(command: &'a DrawIndirectCommand) -> Self {
        vks::VkDrawIndirectCommand {
            vertexCount: command.vertex_count,
            instanceCount: command.instance_count,
            firstVertex: command.first_vertex,
            firstInstance: command.first_instance,
        }
    }
}
