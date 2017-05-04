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

mod allocator_helper;
mod buffer;
mod buffer_view;
mod command_buffer;
mod command_pool;
mod descriptor_pool;
mod descriptor_set_layout;
mod device;
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

use libc::{c_char, c_void};
use std::ffi::{CStr, CString};
use std::fmt;
use std::mem;
use std::ops::Deref;
use std::ptr;
use std::slice;
use utils;
use vk_sys;

pub use self::buffer::Buffer;
pub use self::buffer_view::BufferView;
pub use self::command_buffer::CommandBuffer;
pub use self::command_pool::CommandPool;
pub use self::descriptor_pool::DescriptorPool;
pub use self::descriptor_set_layout::DescriptorSetLayout;
pub use self::device::Device;
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

#[derive(Debug, Copy, Clone)]
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
            major: vk_sys::vk_version_major(version),
            minor: vk_sys::vk_version_minor(version),
            patch: vk_sys::vk_version_patch(version),
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
        vk_sys::vk_make_version(self.major, self.minor, self.patch)
    }

    pub fn api_version_from_optional(version: Option<Version>) -> u32 {
        match version {
            Some(version) => version.as_api_version(),
            None => 0,
        }
    }
}

pub enum PipelineCacheHeaderVersion {
    One,
    Unknown(vk_sys::VkPipelineCacheHeaderVersion),
}

impl From<vk_sys::VkPipelineCacheHeaderVersion> for PipelineCacheHeaderVersion {
    fn from(version: vk_sys::VkPipelineCacheHeaderVersion) -> Self {
        match version {
            vk_sys::VK_PIPELINE_CACHE_HEADER_VERSION_ONE => PipelineCacheHeaderVersion::One,
            _ => PipelineCacheHeaderVersion::Unknown(version),
        }
    }
}

impl From<PipelineCacheHeaderVersion> for vk_sys::VkPipelineCacheHeaderVersion {
    fn from(version: PipelineCacheHeaderVersion) -> Self {
        match version {
            PipelineCacheHeaderVersion::One => vk_sys::VK_PIPELINE_CACHE_HEADER_VERSION_ONE,
            PipelineCacheHeaderVersion::Unknown(version) => version,
        }
    }
}

#[derive(Debug, Copy, Clone)]
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
    Unknown(vk_sys::VkResult),
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
            Error::Unknown(_) => "unknown error",
        }
    }
}

impl From<vk_sys::VkResult> for Error {
    fn from(res: vk_sys::VkResult) -> Self {
        debug_assert!(res.as_raw() < 0);

        match res {
            vk_sys::VK_ERROR_OUT_OF_HOST_MEMORY => Error::OutOfHostMemory,
            vk_sys::VK_ERROR_OUT_OF_DEVICE_MEMORY => Error::OutOfDeviceMemory,
            vk_sys::VK_ERROR_INITIALIZATION_FAILED => Error::InitializationFailed,
            vk_sys::VK_ERROR_DEVICE_LOST => Error::DeviceLost,
            vk_sys::VK_ERROR_MEMORY_MAP_FAILED => Error::MemoryMapFailed,
            vk_sys::VK_ERROR_LAYER_NOT_PRESENT => Error::LayerNotPresent,
            vk_sys::VK_ERROR_EXTENSION_NOT_PRESENT => Error::ExtensionNotPresent,
            vk_sys::VK_ERROR_FEATURE_NOT_PRESENT => Error::FeatureNotPresent,
            vk_sys::VK_ERROR_INCOMPATIBLE_DRIVER => Error::IncompatibleDriver,
            vk_sys::VK_ERROR_TOO_MANY_OBJECTS => Error::TooManyObjects,
            vk_sys::VK_ERROR_FORMAT_NOT_SUPPORTED => Error::FormatNotSupported,
            _ => Error::Unknown(res),
        }
    }
}

impl From<Error> for vk_sys::VkResult {
    fn from(err: Error) -> Self {
        match err {
            Error::OutOfHostMemory => vk_sys::VK_ERROR_OUT_OF_HOST_MEMORY,
            Error::OutOfDeviceMemory => vk_sys::VK_ERROR_OUT_OF_DEVICE_MEMORY,
            Error::InitializationFailed => vk_sys::VK_ERROR_INITIALIZATION_FAILED,
            Error::DeviceLost => vk_sys::VK_ERROR_DEVICE_LOST,
            Error::MemoryMapFailed => vk_sys::VK_ERROR_MEMORY_MAP_FAILED,
            Error::LayerNotPresent => vk_sys::VK_ERROR_LAYER_NOT_PRESENT,
            Error::ExtensionNotPresent => vk_sys::VK_ERROR_EXTENSION_NOT_PRESENT,
            Error::FeatureNotPresent => vk_sys::VK_ERROR_FEATURE_NOT_PRESENT,
            Error::IncompatibleDriver => vk_sys::VK_ERROR_INCOMPATIBLE_DRIVER,
            Error::TooManyObjects => vk_sys::VK_ERROR_TOO_MANY_OBJECTS,
            Error::FormatNotSupported => vk_sys::VK_ERROR_FORMAT_NOT_SUPPORTED,
            Error::Unknown(res) => res,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SystemAllocationSope {
    Command,
    Object,
    Cache,
    Device,
    Instance,
    Unknown(vk_sys::VkSystemAllocationScope),
}

impl From<vk_sys::VkSystemAllocationScope> for SystemAllocationSope {
    fn from(scope: vk_sys::VkSystemAllocationScope) -> Self {
        match scope {
            vk_sys::VK_SYSTEM_ALLOCATION_SCOPE_COMMAND => SystemAllocationSope::Command,
            vk_sys::VK_SYSTEM_ALLOCATION_SCOPE_OBJECT => SystemAllocationSope::Object,
            vk_sys::VK_SYSTEM_ALLOCATION_SCOPE_CACHE => SystemAllocationSope::Cache,
            vk_sys::VK_SYSTEM_ALLOCATION_SCOPE_DEVICE => SystemAllocationSope::Device,
            vk_sys::VK_SYSTEM_ALLOCATION_SCOPE_INSTANCE => SystemAllocationSope::Instance,
            _ => SystemAllocationSope::Unknown(scope),
        }
    }
}

impl From<SystemAllocationSope> for vk_sys::VkSystemAllocationScope {
    fn from(scope: SystemAllocationSope) -> vk_sys::VkSystemAllocationScope {
        match scope {
            SystemAllocationSope::Command => vk_sys::VK_SYSTEM_ALLOCATION_SCOPE_COMMAND,
            SystemAllocationSope::Object => vk_sys::VK_SYSTEM_ALLOCATION_SCOPE_OBJECT,
            SystemAllocationSope::Cache => vk_sys::VK_SYSTEM_ALLOCATION_SCOPE_CACHE,
            SystemAllocationSope::Device => vk_sys::VK_SYSTEM_ALLOCATION_SCOPE_DEVICE,
            SystemAllocationSope::Instance => vk_sys::VK_SYSTEM_ALLOCATION_SCOPE_INSTANCE,
            SystemAllocationSope::Unknown(scope) => scope,
        }
    }
}

pub enum InternalAllocationType {
    Executable,
    Unknown(vk_sys::VkInternalAllocationType),
}

impl From<vk_sys::VkInternalAllocationType> for InternalAllocationType {
    fn from(allocation_type: vk_sys::VkInternalAllocationType) -> Self {
        match allocation_type {
            vk_sys::VK_INTERNAL_ALLOCATION_TYPE_EXECUTABLE => InternalAllocationType::Executable,
            _ => InternalAllocationType::Unknown(allocation_type),
        }
    }
}

impl From<InternalAllocationType> for vk_sys::VkInternalAllocationType {
    fn from(allocation_type: InternalAllocationType) -> vk_sys::VkInternalAllocationType {
        match allocation_type {
            InternalAllocationType::Executable => vk_sys::VK_INTERNAL_ALLOCATION_TYPE_EXECUTABLE,
            InternalAllocationType::Unknown(allocation_type) => allocation_type,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
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
    Unknown(vk_sys::VkFormat),
}

impl From<vk_sys::VkFormat> for Format {
    fn from(format: vk_sys::VkFormat) -> Self {
        match format {
            vk_sys::VK_FORMAT_UNDEFINED => Format::Undefined,
            vk_sys::VK_FORMAT_R4G4_UNORM_PACK8 => Format::R4G4_UNorm_Pack8,
            vk_sys::VK_FORMAT_R4G4B4A4_UNORM_PACK16 => Format::R4G4B4A4_UNorm_Pack16,
            vk_sys::VK_FORMAT_B4G4R4A4_UNORM_PACK16 => Format::B4G4R4A4_UNorm_Pack16,
            vk_sys::VK_FORMAT_R5G6B5_UNORM_PACK16 => Format::R5G6B5_UNorm_Pack16,
            vk_sys::VK_FORMAT_B5G6R5_UNORM_PACK16 => Format::B5G6R5_UNorm_Pack16,
            vk_sys::VK_FORMAT_R5G5B5A1_UNORM_PACK16 => Format::R5G5B5A1_UNorm_Pack16,
            vk_sys::VK_FORMAT_B5G5R5A1_UNORM_PACK16 => Format::B5G5R5A1_UNorm_Pack16,
            vk_sys::VK_FORMAT_A1R5G5B5_UNORM_PACK16 => Format::A1R5G5B5_UNorm_Pack16,
            vk_sys::VK_FORMAT_R8_UNORM => Format::R8_UNorm,
            vk_sys::VK_FORMAT_R8_SNORM => Format::R8_SNorm,
            vk_sys::VK_FORMAT_R8_USCALED => Format::R8_UScaled,
            vk_sys::VK_FORMAT_R8_SSCALED => Format::R8_SScaled,
            vk_sys::VK_FORMAT_R8_UINT => Format::R8_UInt,
            vk_sys::VK_FORMAT_R8_SINT => Format::R8_SInt,
            vk_sys::VK_FORMAT_R8_SRGB => Format::R8_sRGB,
            vk_sys::VK_FORMAT_R8G8_UNORM => Format::R8G8_UNorm,
            vk_sys::VK_FORMAT_R8G8_SNORM => Format::R8G8_SNorm,
            vk_sys::VK_FORMAT_R8G8_USCALED => Format::R8G8_UScaled,
            vk_sys::VK_FORMAT_R8G8_SSCALED => Format::R8G8_SScaled,
            vk_sys::VK_FORMAT_R8G8_UINT => Format::R8G8_UInt,
            vk_sys::VK_FORMAT_R8G8_SINT => Format::R8G8_SInt,
            vk_sys::VK_FORMAT_R8G8_SRGB => Format::R8G8_sRGB,
            vk_sys::VK_FORMAT_R8G8B8_UNORM => Format::R8G8B8_UNorm,
            vk_sys::VK_FORMAT_R8G8B8_SNORM => Format::R8G8B8_SNorm,
            vk_sys::VK_FORMAT_R8G8B8_USCALED => Format::R8G8B8_UScaled,
            vk_sys::VK_FORMAT_R8G8B8_SSCALED => Format::R8G8B8_SScaled,
            vk_sys::VK_FORMAT_R8G8B8_UINT => Format::R8G8B8_UInt,
            vk_sys::VK_FORMAT_R8G8B8_SINT => Format::R8G8B8_SInt,
            vk_sys::VK_FORMAT_R8G8B8_SRGB => Format::R8G8B8_sRGB,
            vk_sys::VK_FORMAT_B8G8R8_UNORM => Format::B8G8R8_UNorm,
            vk_sys::VK_FORMAT_B8G8R8_SNORM => Format::B8G8R8_SNorm,
            vk_sys::VK_FORMAT_B8G8R8_USCALED => Format::B8G8R8_UScaled,
            vk_sys::VK_FORMAT_B8G8R8_SSCALED => Format::B8G8R8_SScaled,
            vk_sys::VK_FORMAT_B8G8R8_UINT => Format::B8G8R8_UInt,
            vk_sys::VK_FORMAT_B8G8R8_SINT => Format::B8G8R8_SInt,
            vk_sys::VK_FORMAT_B8G8R8_SRGB => Format::B8G8R8_sRGB,
            vk_sys::VK_FORMAT_R8G8B8A8_UNORM => Format::R8G8B8A8_UNorm,
            vk_sys::VK_FORMAT_R8G8B8A8_SNORM => Format::R8G8B8A8_SNorm,
            vk_sys::VK_FORMAT_R8G8B8A8_USCALED => Format::R8G8B8A8_UScaled,
            vk_sys::VK_FORMAT_R8G8B8A8_SSCALED => Format::R8G8B8A8_SScaled,
            vk_sys::VK_FORMAT_R8G8B8A8_UINT => Format::R8G8B8A8_UInt,
            vk_sys::VK_FORMAT_R8G8B8A8_SINT => Format::R8G8B8A8_SInt,
            vk_sys::VK_FORMAT_R8G8B8A8_SRGB => Format::R8G8B8A8_sRGB,
            vk_sys::VK_FORMAT_B8G8R8A8_UNORM => Format::B8G8R8A8_UNorm,
            vk_sys::VK_FORMAT_B8G8R8A8_SNORM => Format::B8G8R8A8_SNorm,
            vk_sys::VK_FORMAT_B8G8R8A8_USCALED => Format::B8G8R8A8_UScaled,
            vk_sys::VK_FORMAT_B8G8R8A8_SSCALED => Format::B8G8R8A8_SScaled,
            vk_sys::VK_FORMAT_B8G8R8A8_UINT => Format::B8G8R8A8_UInt,
            vk_sys::VK_FORMAT_B8G8R8A8_SINT => Format::B8G8R8A8_SInt,
            vk_sys::VK_FORMAT_B8G8R8A8_SRGB => Format::B8G8R8A8_sRGB,
            vk_sys::VK_FORMAT_A8B8G8R8_UNORM_PACK32 => Format::A8B8G8R8_UNorm_Pack32,
            vk_sys::VK_FORMAT_A8B8G8R8_SNORM_PACK32 => Format::A8B8G8R8_SNorm_Pack32,
            vk_sys::VK_FORMAT_A8B8G8R8_USCALED_PACK32 => Format::A8B8G8R8_UScaled_Pack32,
            vk_sys::VK_FORMAT_A8B8G8R8_SSCALED_PACK32 => Format::A8B8G8R8_SScaled_Pack32,
            vk_sys::VK_FORMAT_A8B8G8R8_UINT_PACK32 => Format::A8B8G8R8_UInt_Pack32,
            vk_sys::VK_FORMAT_A8B8G8R8_SINT_PACK32 => Format::A8B8G8R8_SInt_Pack32,
            vk_sys::VK_FORMAT_A8B8G8R8_SRGB_PACK32 => Format::A8B8G8R8_sRGB_Pack32,
            vk_sys::VK_FORMAT_A2R10G10B10_UNORM_PACK32 => Format::A2R10G10B10_UNorm_Pack32,
            vk_sys::VK_FORMAT_A2R10G10B10_SNORM_PACK32 => Format::A2R10G10B10_SNorm_Pack32,
            vk_sys::VK_FORMAT_A2R10G10B10_USCALED_PACK32 => Format::A2R10G10B10_UScaled_Pack32,
            vk_sys::VK_FORMAT_A2R10G10B10_SSCALED_PACK32 => Format::A2R10G10B10_SScaled_Pack32,
            vk_sys::VK_FORMAT_A2R10G10B10_UINT_PACK32 => Format::A2R10G10B10_UInt_Pack32,
            vk_sys::VK_FORMAT_A2R10G10B10_SINT_PACK32 => Format::A2R10G10B10_SInt_Pack32,
            vk_sys::VK_FORMAT_A2B10G10R10_UNORM_PACK32 => Format::A2B10G10R10_UNorm_Pack32,
            vk_sys::VK_FORMAT_A2B10G10R10_SNORM_PACK32 => Format::A2B10G10R10_SNorm_Pack32,
            vk_sys::VK_FORMAT_A2B10G10R10_USCALED_PACK32 => Format::A2B10G10R10_UScaled_Pack32,
            vk_sys::VK_FORMAT_A2B10G10R10_SSCALED_PACK32 => Format::A2B10G10R10_SScaled_Pack32,
            vk_sys::VK_FORMAT_A2B10G10R10_UINT_PACK32 => Format::A2B10G10R10_UInt_Pack32,
            vk_sys::VK_FORMAT_A2B10G10R10_SINT_PACK32 => Format::A2B10G10R10_SInt_Pack32,
            vk_sys::VK_FORMAT_R16_UNORM => Format::R16_UNorm,
            vk_sys::VK_FORMAT_R16_SNORM => Format::R16_SNorm,
            vk_sys::VK_FORMAT_R16_USCALED => Format::R16_UScaled,
            vk_sys::VK_FORMAT_R16_SSCALED => Format::R16_SScaled,
            vk_sys::VK_FORMAT_R16_UINT => Format::R16_UInt,
            vk_sys::VK_FORMAT_R16_SINT => Format::R16_SInt,
            vk_sys::VK_FORMAT_R16_SFLOAT => Format::R16_SFloat,
            vk_sys::VK_FORMAT_R16G16_UNORM => Format::R16G16_UNorm,
            vk_sys::VK_FORMAT_R16G16_SNORM => Format::R16G16_SNorm,
            vk_sys::VK_FORMAT_R16G16_USCALED => Format::R16G16_UScaled,
            vk_sys::VK_FORMAT_R16G16_SSCALED => Format::R16G16_SScaled,
            vk_sys::VK_FORMAT_R16G16_UINT => Format::R16G16_UInt,
            vk_sys::VK_FORMAT_R16G16_SINT => Format::R16G16_SInt,
            vk_sys::VK_FORMAT_R16G16_SFLOAT => Format::R16G16_SFloat,
            vk_sys::VK_FORMAT_R16G16B16_UNORM => Format::R16G16B16_UNorm,
            vk_sys::VK_FORMAT_R16G16B16_SNORM => Format::R16G16B16_SNorm,
            vk_sys::VK_FORMAT_R16G16B16_USCALED => Format::R16G16B16_UScaled,
            vk_sys::VK_FORMAT_R16G16B16_SSCALED => Format::R16G16B16_SScaled,
            vk_sys::VK_FORMAT_R16G16B16_UINT => Format::R16G16B16_UInt,
            vk_sys::VK_FORMAT_R16G16B16_SINT => Format::R16G16B16_SInt,
            vk_sys::VK_FORMAT_R16G16B16_SFLOAT => Format::R16G16B16_SFloat,
            vk_sys::VK_FORMAT_R16G16B16A16_UNORM => Format::R16G16B16A16_UNorm,
            vk_sys::VK_FORMAT_R16G16B16A16_SNORM => Format::R16G16B16A16_SNorm,
            vk_sys::VK_FORMAT_R16G16B16A16_USCALED => Format::R16G16B16A16_UScaled,
            vk_sys::VK_FORMAT_R16G16B16A16_SSCALED => Format::R16G16B16A16_SScaled,
            vk_sys::VK_FORMAT_R16G16B16A16_UINT => Format::R16G16B16A16_UInt,
            vk_sys::VK_FORMAT_R16G16B16A16_SINT => Format::R16G16B16A16_SInt,
            vk_sys::VK_FORMAT_R16G16B16A16_SFLOAT => Format::R16G16B16A16_SFloat,
            vk_sys::VK_FORMAT_R32_UINT => Format::R32_UInt,
            vk_sys::VK_FORMAT_R32_SINT => Format::R32_SInt,
            vk_sys::VK_FORMAT_R32_SFLOAT => Format::R32_SFloat,
            vk_sys::VK_FORMAT_R32G32_UINT => Format::R32G32_UInt,
            vk_sys::VK_FORMAT_R32G32_SINT => Format::R32G32_SInt,
            vk_sys::VK_FORMAT_R32G32_SFLOAT => Format::R32G32_SFloat,
            vk_sys::VK_FORMAT_R32G32B32_UINT => Format::R32G32B32_UInt,
            vk_sys::VK_FORMAT_R32G32B32_SINT => Format::R32G32B32_SInt,
            vk_sys::VK_FORMAT_R32G32B32_SFLOAT => Format::R32G32B32_SFloat,
            vk_sys::VK_FORMAT_R32G32B32A32_UINT => Format::R32G32B32A32_UInt,
            vk_sys::VK_FORMAT_R32G32B32A32_SINT => Format::R32G32B32A32_SInt,
            vk_sys::VK_FORMAT_R32G32B32A32_SFLOAT => Format::R32G32B32A32_SFloat,
            vk_sys::VK_FORMAT_R64_UINT => Format::R64_UInt,
            vk_sys::VK_FORMAT_R64_SINT => Format::R64_SInt,
            vk_sys::VK_FORMAT_R64_SFLOAT => Format::R64_SFloat,
            vk_sys::VK_FORMAT_R64G64_UINT => Format::R64G64_UInt,
            vk_sys::VK_FORMAT_R64G64_SINT => Format::R64G64_SInt,
            vk_sys::VK_FORMAT_R64G64_SFLOAT => Format::R64G64_SFloat,
            vk_sys::VK_FORMAT_R64G64B64_UINT => Format::R64G64B64_UInt,
            vk_sys::VK_FORMAT_R64G64B64_SINT => Format::R64G64B64_SInt,
            vk_sys::VK_FORMAT_R64G64B64_SFLOAT => Format::R64G64B64_SFloat,
            vk_sys::VK_FORMAT_R64G64B64A64_UINT => Format::R64G64B64A64_UInt,
            vk_sys::VK_FORMAT_R64G64B64A64_SINT => Format::R64G64B64A64_SInt,
            vk_sys::VK_FORMAT_R64G64B64A64_SFLOAT => Format::R64G64B64A64_SFloat,
            vk_sys::VK_FORMAT_B10G11R11_UFLOAT_PACK32 => Format::B10G11R11_UFloat_Pack32,
            vk_sys::VK_FORMAT_E5B9G9R9_UFLOAT_PACK32 => Format::E5B9G9R9_UFloat_Pack32,
            vk_sys::VK_FORMAT_D16_UNORM => Format::D16_UNorm,
            vk_sys::VK_FORMAT_X8_D24_UNORM_PACK32 => Format::X8_D24_UNorm_Pack32,
            vk_sys::VK_FORMAT_D32_SFLOAT => Format::D32_SFloat,
            vk_sys::VK_FORMAT_S8_UINT => Format::S8_UInt,
            vk_sys::VK_FORMAT_D16_UNORM_S8_UINT => Format::D16_UNorm_S8_UInt,
            vk_sys::VK_FORMAT_D24_UNORM_S8_UINT => Format::D24_UNorm_S8_UInt,
            vk_sys::VK_FORMAT_D32_SFLOAT_S8_UINT => Format::D32_SFloat_S8_UInt,
            vk_sys::VK_FORMAT_BC1_RGB_UNORM_BLOCK => Format::BC1_RGB_UNorm_Block,
            vk_sys::VK_FORMAT_BC1_RGB_SRGB_BLOCK => Format::BC1_RGB_sRGB_Block,
            vk_sys::VK_FORMAT_BC1_RGBA_UNORM_BLOCK => Format::BC1_RGBA_UNorm_Block,
            vk_sys::VK_FORMAT_BC1_RGBA_SRGB_BLOCK => Format::BC1_RGBA_sRGB_Block,
            vk_sys::VK_FORMAT_BC2_UNORM_BLOCK => Format::BC2_UNorm_Block,
            vk_sys::VK_FORMAT_BC2_SRGB_BLOCK => Format::BC2_sRGB_Block,
            vk_sys::VK_FORMAT_BC3_UNORM_BLOCK => Format::BC3_UNorm_Block,
            vk_sys::VK_FORMAT_BC3_SRGB_BLOCK => Format::BC3_sRGB_Block,
            vk_sys::VK_FORMAT_BC4_UNORM_BLOCK => Format::BC4_UNorm_Block,
            vk_sys::VK_FORMAT_BC4_SNORM_BLOCK => Format::BC4_SNorm_Block,
            vk_sys::VK_FORMAT_BC5_UNORM_BLOCK => Format::BC5_UNorm_Block,
            vk_sys::VK_FORMAT_BC5_SNORM_BLOCK => Format::BC5_SNorm_Block,
            vk_sys::VK_FORMAT_BC6H_UFLOAT_BLOCK => Format::BC6H_UFloat_Block,
            vk_sys::VK_FORMAT_BC6H_SFLOAT_BLOCK => Format::BC6H_SFloat_Block,
            vk_sys::VK_FORMAT_BC7_UNORM_BLOCK => Format::BC7_UNorm_Block,
            vk_sys::VK_FORMAT_BC7_SRGB_BLOCK => Format::BC7_sRGB_Block,
            vk_sys::VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK => Format::ETC2_R8G8B8_UNorm_Block,
            vk_sys::VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK => Format::ETC2_R8G8B8_sRGB_Block,
            vk_sys::VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK => Format::ETC2_R8G8B8A1_UNorm_Block,
            vk_sys::VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK => Format::ETC2_R8G8B8A1_sRGB_Block,
            vk_sys::VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK => Format::ETC2_R8G8B8A8_UNorm_Block,
            vk_sys::VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK => Format::ETC2_R8G8B8A8_sRGB_Block,
            vk_sys::VK_FORMAT_EAC_R11_UNORM_BLOCK => Format::EAC_R11_UNorm_Block,
            vk_sys::VK_FORMAT_EAC_R11_SNORM_BLOCK => Format::EAC_R11_SNorm_Block,
            vk_sys::VK_FORMAT_EAC_R11G11_UNORM_BLOCK => Format::EAC_R11G11_UNorm_Block,
            vk_sys::VK_FORMAT_EAC_R11G11_SNORM_BLOCK => Format::EAC_R11G11_SNorm_Block,
            vk_sys::VK_FORMAT_ASTC_4x4_UNORM_BLOCK => Format::ASTC_4x4_UNorm_Block,
            vk_sys::VK_FORMAT_ASTC_4x4_SRGB_BLOCK => Format::ASTC_4x4_sRGB_Block,
            vk_sys::VK_FORMAT_ASTC_5x4_UNORM_BLOCK => Format::ASTC_5x4_UNorm_Block,
            vk_sys::VK_FORMAT_ASTC_5x4_SRGB_BLOCK => Format::ASTC_5x4_sRGB_Block,
            vk_sys::VK_FORMAT_ASTC_5x5_UNORM_BLOCK => Format::ASTC_5x5_UNorm_Block,
            vk_sys::VK_FORMAT_ASTC_5x5_SRGB_BLOCK => Format::ASTC_5x5_sRGB_Block,
            vk_sys::VK_FORMAT_ASTC_6x5_UNORM_BLOCK => Format::ASTC_6x5_UNorm_Block,
            vk_sys::VK_FORMAT_ASTC_6x5_SRGB_BLOCK => Format::ASTC_6x5_sRGB_Block,
            vk_sys::VK_FORMAT_ASTC_6x6_UNORM_BLOCK => Format::ASTC_6x6_UNorm_Block,
            vk_sys::VK_FORMAT_ASTC_6x6_SRGB_BLOCK => Format::ASTC_6x6_sRGB_Block,
            vk_sys::VK_FORMAT_ASTC_8x5_UNORM_BLOCK => Format::ASTC_8x5_UNorm_Block,
            vk_sys::VK_FORMAT_ASTC_8x5_SRGB_BLOCK => Format::ASTC_8x5_sRGB_Block,
            vk_sys::VK_FORMAT_ASTC_8x6_UNORM_BLOCK => Format::ASTC_8x6_UNorm_Block,
            vk_sys::VK_FORMAT_ASTC_8x6_SRGB_BLOCK => Format::ASTC_8x6_sRGB_Block,
            vk_sys::VK_FORMAT_ASTC_8x8_UNORM_BLOCK => Format::ASTC_8x8_UNorm_Block,
            vk_sys::VK_FORMAT_ASTC_8x8_SRGB_BLOCK => Format::ASTC_8x8_sRGB_Block,
            vk_sys::VK_FORMAT_ASTC_10x5_UNORM_BLOCK => Format::ASTC_10x5_UNorm_Block,
            vk_sys::VK_FORMAT_ASTC_10x5_SRGB_BLOCK => Format::ASTC_10x5_sRGB_Block,
            vk_sys::VK_FORMAT_ASTC_10x6_UNORM_BLOCK => Format::ASTC_10x6_UNorm_Block,
            vk_sys::VK_FORMAT_ASTC_10x6_SRGB_BLOCK => Format::ASTC_10x6_sRGB_Block,
            vk_sys::VK_FORMAT_ASTC_10x8_UNORM_BLOCK => Format::ASTC_10x8_UNorm_Block,
            vk_sys::VK_FORMAT_ASTC_10x8_SRGB_BLOCK => Format::ASTC_10x8_sRGB_Block,
            vk_sys::VK_FORMAT_ASTC_10x10_UNORM_BLOCK => Format::ASTC_10x10_UNorm_Block,
            vk_sys::VK_FORMAT_ASTC_10x10_SRGB_BLOCK => Format::ASTC_10x10_sRGB_Block,
            vk_sys::VK_FORMAT_ASTC_12x10_UNORM_BLOCK => Format::ASTC_12x10_UNorm_Block,
            vk_sys::VK_FORMAT_ASTC_12x10_SRGB_BLOCK => Format::ASTC_12x10_sRGB_Block,
            vk_sys::VK_FORMAT_ASTC_12x12_UNORM_BLOCK => Format::ASTC_12x12_UNorm_Block,
            vk_sys::VK_FORMAT_ASTC_12x12_SRGB_BLOCK => Format::ASTC_12x12_sRGB_Block,
            _ => Format::Unknown(format),
        }
    }
}

impl From<Format> for vk_sys::VkFormat {
    fn from(format: Format) -> Self {
        match format {
            Format::Undefined => vk_sys::VK_FORMAT_UNDEFINED,
            Format::R4G4_UNorm_Pack8 => vk_sys::VK_FORMAT_R4G4_UNORM_PACK8,
            Format::R4G4B4A4_UNorm_Pack16 => vk_sys::VK_FORMAT_R4G4B4A4_UNORM_PACK16,
            Format::B4G4R4A4_UNorm_Pack16 => vk_sys::VK_FORMAT_B4G4R4A4_UNORM_PACK16,
            Format::R5G6B5_UNorm_Pack16 => vk_sys::VK_FORMAT_R5G6B5_UNORM_PACK16,
            Format::B5G6R5_UNorm_Pack16 => vk_sys::VK_FORMAT_B5G6R5_UNORM_PACK16,
            Format::R5G5B5A1_UNorm_Pack16 => vk_sys::VK_FORMAT_R5G5B5A1_UNORM_PACK16,
            Format::B5G5R5A1_UNorm_Pack16 => vk_sys::VK_FORMAT_B5G5R5A1_UNORM_PACK16,
            Format::A1R5G5B5_UNorm_Pack16 => vk_sys::VK_FORMAT_A1R5G5B5_UNORM_PACK16,
            Format::R8_UNorm => vk_sys::VK_FORMAT_R8_UNORM,
            Format::R8_SNorm => vk_sys::VK_FORMAT_R8_SNORM,
            Format::R8_UScaled => vk_sys::VK_FORMAT_R8_USCALED,
            Format::R8_SScaled => vk_sys::VK_FORMAT_R8_SSCALED,
            Format::R8_UInt => vk_sys::VK_FORMAT_R8_UINT,
            Format::R8_SInt => vk_sys::VK_FORMAT_R8_SINT,
            Format::R8_sRGB => vk_sys::VK_FORMAT_R8_SRGB,
            Format::R8G8_UNorm => vk_sys::VK_FORMAT_R8G8_UNORM,
            Format::R8G8_SNorm => vk_sys::VK_FORMAT_R8G8_SNORM,
            Format::R8G8_UScaled => vk_sys::VK_FORMAT_R8G8_USCALED,
            Format::R8G8_SScaled => vk_sys::VK_FORMAT_R8G8_SSCALED,
            Format::R8G8_UInt => vk_sys::VK_FORMAT_R8G8_UINT,
            Format::R8G8_SInt => vk_sys::VK_FORMAT_R8G8_SINT,
            Format::R8G8_sRGB => vk_sys::VK_FORMAT_R8G8_SRGB,
            Format::R8G8B8_UNorm => vk_sys::VK_FORMAT_R8G8B8_UNORM,
            Format::R8G8B8_SNorm => vk_sys::VK_FORMAT_R8G8B8_SNORM,
            Format::R8G8B8_UScaled => vk_sys::VK_FORMAT_R8G8B8_USCALED,
            Format::R8G8B8_SScaled => vk_sys::VK_FORMAT_R8G8B8_SSCALED,
            Format::R8G8B8_UInt => vk_sys::VK_FORMAT_R8G8B8_UINT,
            Format::R8G8B8_SInt => vk_sys::VK_FORMAT_R8G8B8_SINT,
            Format::R8G8B8_sRGB => vk_sys::VK_FORMAT_R8G8B8_SRGB,
            Format::B8G8R8_UNorm => vk_sys::VK_FORMAT_B8G8R8_UNORM,
            Format::B8G8R8_SNorm => vk_sys::VK_FORMAT_B8G8R8_SNORM,
            Format::B8G8R8_UScaled => vk_sys::VK_FORMAT_B8G8R8_USCALED,
            Format::B8G8R8_SScaled => vk_sys::VK_FORMAT_B8G8R8_SSCALED,
            Format::B8G8R8_UInt => vk_sys::VK_FORMAT_B8G8R8_UINT,
            Format::B8G8R8_SInt => vk_sys::VK_FORMAT_B8G8R8_SINT,
            Format::B8G8R8_sRGB => vk_sys::VK_FORMAT_B8G8R8_SRGB,
            Format::R8G8B8A8_UNorm => vk_sys::VK_FORMAT_R8G8B8A8_UNORM,
            Format::R8G8B8A8_SNorm => vk_sys::VK_FORMAT_R8G8B8A8_SNORM,
            Format::R8G8B8A8_UScaled => vk_sys::VK_FORMAT_R8G8B8A8_USCALED,
            Format::R8G8B8A8_SScaled => vk_sys::VK_FORMAT_R8G8B8A8_SSCALED,
            Format::R8G8B8A8_UInt => vk_sys::VK_FORMAT_R8G8B8A8_UINT,
            Format::R8G8B8A8_SInt => vk_sys::VK_FORMAT_R8G8B8A8_SINT,
            Format::R8G8B8A8_sRGB => vk_sys::VK_FORMAT_R8G8B8A8_SRGB,
            Format::B8G8R8A8_UNorm => vk_sys::VK_FORMAT_B8G8R8A8_UNORM,
            Format::B8G8R8A8_SNorm => vk_sys::VK_FORMAT_B8G8R8A8_SNORM,
            Format::B8G8R8A8_UScaled => vk_sys::VK_FORMAT_B8G8R8A8_USCALED,
            Format::B8G8R8A8_SScaled => vk_sys::VK_FORMAT_B8G8R8A8_SSCALED,
            Format::B8G8R8A8_UInt => vk_sys::VK_FORMAT_B8G8R8A8_UINT,
            Format::B8G8R8A8_SInt => vk_sys::VK_FORMAT_B8G8R8A8_SINT,
            Format::B8G8R8A8_sRGB => vk_sys::VK_FORMAT_B8G8R8A8_SRGB,
            Format::A8B8G8R8_UNorm_Pack32 => vk_sys::VK_FORMAT_A8B8G8R8_UNORM_PACK32,
            Format::A8B8G8R8_SNorm_Pack32 => vk_sys::VK_FORMAT_A8B8G8R8_SNORM_PACK32,
            Format::A8B8G8R8_UScaled_Pack32 => vk_sys::VK_FORMAT_A8B8G8R8_USCALED_PACK32,
            Format::A8B8G8R8_SScaled_Pack32 => vk_sys::VK_FORMAT_A8B8G8R8_SSCALED_PACK32,
            Format::A8B8G8R8_UInt_Pack32 => vk_sys::VK_FORMAT_A8B8G8R8_UINT_PACK32,
            Format::A8B8G8R8_SInt_Pack32 => vk_sys::VK_FORMAT_A8B8G8R8_SINT_PACK32,
            Format::A8B8G8R8_sRGB_Pack32 => vk_sys::VK_FORMAT_A8B8G8R8_SRGB_PACK32,
            Format::A2R10G10B10_UNorm_Pack32 => vk_sys::VK_FORMAT_A2R10G10B10_UNORM_PACK32,
            Format::A2R10G10B10_SNorm_Pack32 => vk_sys::VK_FORMAT_A2R10G10B10_SNORM_PACK32,
            Format::A2R10G10B10_UScaled_Pack32 => vk_sys::VK_FORMAT_A2R10G10B10_USCALED_PACK32,
            Format::A2R10G10B10_SScaled_Pack32 => vk_sys::VK_FORMAT_A2R10G10B10_SSCALED_PACK32,
            Format::A2R10G10B10_UInt_Pack32 => vk_sys::VK_FORMAT_A2R10G10B10_UINT_PACK32,
            Format::A2R10G10B10_SInt_Pack32 => vk_sys::VK_FORMAT_A2R10G10B10_SINT_PACK32,
            Format::A2B10G10R10_UNorm_Pack32 => vk_sys::VK_FORMAT_A2B10G10R10_UNORM_PACK32,
            Format::A2B10G10R10_SNorm_Pack32 => vk_sys::VK_FORMAT_A2B10G10R10_SNORM_PACK32,
            Format::A2B10G10R10_UScaled_Pack32 => vk_sys::VK_FORMAT_A2B10G10R10_USCALED_PACK32,
            Format::A2B10G10R10_SScaled_Pack32 => vk_sys::VK_FORMAT_A2B10G10R10_SSCALED_PACK32,
            Format::A2B10G10R10_UInt_Pack32 => vk_sys::VK_FORMAT_A2B10G10R10_UINT_PACK32,
            Format::A2B10G10R10_SInt_Pack32 => vk_sys::VK_FORMAT_A2B10G10R10_SINT_PACK32,
            Format::R16_UNorm => vk_sys::VK_FORMAT_R16_UNORM,
            Format::R16_SNorm => vk_sys::VK_FORMAT_R16_SNORM,
            Format::R16_UScaled => vk_sys::VK_FORMAT_R16_USCALED,
            Format::R16_SScaled => vk_sys::VK_FORMAT_R16_SSCALED,
            Format::R16_UInt => vk_sys::VK_FORMAT_R16_UINT,
            Format::R16_SInt => vk_sys::VK_FORMAT_R16_SINT,
            Format::R16_SFloat => vk_sys::VK_FORMAT_R16_SFLOAT,
            Format::R16G16_UNorm => vk_sys::VK_FORMAT_R16G16_UNORM,
            Format::R16G16_SNorm => vk_sys::VK_FORMAT_R16G16_SNORM,
            Format::R16G16_UScaled => vk_sys::VK_FORMAT_R16G16_USCALED,
            Format::R16G16_SScaled => vk_sys::VK_FORMAT_R16G16_SSCALED,
            Format::R16G16_UInt => vk_sys::VK_FORMAT_R16G16_UINT,
            Format::R16G16_SInt => vk_sys::VK_FORMAT_R16G16_SINT,
            Format::R16G16_SFloat => vk_sys::VK_FORMAT_R16G16_SFLOAT,
            Format::R16G16B16_UNorm => vk_sys::VK_FORMAT_R16G16B16_UNORM,
            Format::R16G16B16_SNorm => vk_sys::VK_FORMAT_R16G16B16_SNORM,
            Format::R16G16B16_UScaled => vk_sys::VK_FORMAT_R16G16B16_USCALED,
            Format::R16G16B16_SScaled => vk_sys::VK_FORMAT_R16G16B16_SSCALED,
            Format::R16G16B16_UInt => vk_sys::VK_FORMAT_R16G16B16_UINT,
            Format::R16G16B16_SInt => vk_sys::VK_FORMAT_R16G16B16_SINT,
            Format::R16G16B16_SFloat => vk_sys::VK_FORMAT_R16G16B16_SFLOAT,
            Format::R16G16B16A16_UNorm => vk_sys::VK_FORMAT_R16G16B16A16_UNORM,
            Format::R16G16B16A16_SNorm => vk_sys::VK_FORMAT_R16G16B16A16_SNORM,
            Format::R16G16B16A16_UScaled => vk_sys::VK_FORMAT_R16G16B16A16_USCALED,
            Format::R16G16B16A16_SScaled => vk_sys::VK_FORMAT_R16G16B16A16_SSCALED,
            Format::R16G16B16A16_UInt => vk_sys::VK_FORMAT_R16G16B16A16_UINT,
            Format::R16G16B16A16_SInt => vk_sys::VK_FORMAT_R16G16B16A16_SINT,
            Format::R16G16B16A16_SFloat => vk_sys::VK_FORMAT_R16G16B16A16_SFLOAT,
            Format::R32_UInt => vk_sys::VK_FORMAT_R32_UINT,
            Format::R32_SInt => vk_sys::VK_FORMAT_R32_SINT,
            Format::R32_SFloat => vk_sys::VK_FORMAT_R32_SFLOAT,
            Format::R32G32_UInt => vk_sys::VK_FORMAT_R32G32_UINT,
            Format::R32G32_SInt => vk_sys::VK_FORMAT_R32G32_SINT,
            Format::R32G32_SFloat => vk_sys::VK_FORMAT_R32G32_SFLOAT,
            Format::R32G32B32_UInt => vk_sys::VK_FORMAT_R32G32B32_UINT,
            Format::R32G32B32_SInt => vk_sys::VK_FORMAT_R32G32B32_SINT,
            Format::R32G32B32_SFloat => vk_sys::VK_FORMAT_R32G32B32_SFLOAT,
            Format::R32G32B32A32_UInt => vk_sys::VK_FORMAT_R32G32B32A32_UINT,
            Format::R32G32B32A32_SInt => vk_sys::VK_FORMAT_R32G32B32A32_SINT,
            Format::R32G32B32A32_SFloat => vk_sys::VK_FORMAT_R32G32B32A32_SFLOAT,
            Format::R64_UInt => vk_sys::VK_FORMAT_R64_UINT,
            Format::R64_SInt => vk_sys::VK_FORMAT_R64_SINT,
            Format::R64_SFloat => vk_sys::VK_FORMAT_R64_SFLOAT,
            Format::R64G64_UInt => vk_sys::VK_FORMAT_R64G64_UINT,
            Format::R64G64_SInt => vk_sys::VK_FORMAT_R64G64_SINT,
            Format::R64G64_SFloat => vk_sys::VK_FORMAT_R64G64_SFLOAT,
            Format::R64G64B64_UInt => vk_sys::VK_FORMAT_R64G64B64_UINT,
            Format::R64G64B64_SInt => vk_sys::VK_FORMAT_R64G64B64_SINT,
            Format::R64G64B64_SFloat => vk_sys::VK_FORMAT_R64G64B64_SFLOAT,
            Format::R64G64B64A64_UInt => vk_sys::VK_FORMAT_R64G64B64A64_UINT,
            Format::R64G64B64A64_SInt => vk_sys::VK_FORMAT_R64G64B64A64_SINT,
            Format::R64G64B64A64_SFloat => vk_sys::VK_FORMAT_R64G64B64A64_SFLOAT,
            Format::B10G11R11_UFloat_Pack32 => vk_sys::VK_FORMAT_B10G11R11_UFLOAT_PACK32,
            Format::E5B9G9R9_UFloat_Pack32 => vk_sys::VK_FORMAT_E5B9G9R9_UFLOAT_PACK32,
            Format::D16_UNorm => vk_sys::VK_FORMAT_D16_UNORM,
            Format::X8_D24_UNorm_Pack32 => vk_sys::VK_FORMAT_X8_D24_UNORM_PACK32,
            Format::D32_SFloat => vk_sys::VK_FORMAT_D32_SFLOAT,
            Format::S8_UInt => vk_sys::VK_FORMAT_S8_UINT,
            Format::D16_UNorm_S8_UInt => vk_sys::VK_FORMAT_D16_UNORM_S8_UINT,
            Format::D24_UNorm_S8_UInt => vk_sys::VK_FORMAT_D24_UNORM_S8_UINT,
            Format::D32_SFloat_S8_UInt => vk_sys::VK_FORMAT_D32_SFLOAT_S8_UINT,
            Format::BC1_RGB_UNorm_Block => vk_sys::VK_FORMAT_BC1_RGB_UNORM_BLOCK,
            Format::BC1_RGB_sRGB_Block => vk_sys::VK_FORMAT_BC1_RGB_SRGB_BLOCK,
            Format::BC1_RGBA_UNorm_Block => vk_sys::VK_FORMAT_BC1_RGBA_UNORM_BLOCK,
            Format::BC1_RGBA_sRGB_Block => vk_sys::VK_FORMAT_BC1_RGBA_SRGB_BLOCK,
            Format::BC2_UNorm_Block => vk_sys::VK_FORMAT_BC2_UNORM_BLOCK,
            Format::BC2_sRGB_Block => vk_sys::VK_FORMAT_BC2_SRGB_BLOCK,
            Format::BC3_UNorm_Block => vk_sys::VK_FORMAT_BC3_UNORM_BLOCK,
            Format::BC3_sRGB_Block => vk_sys::VK_FORMAT_BC3_SRGB_BLOCK,
            Format::BC4_UNorm_Block => vk_sys::VK_FORMAT_BC4_UNORM_BLOCK,
            Format::BC4_SNorm_Block => vk_sys::VK_FORMAT_BC4_SNORM_BLOCK,
            Format::BC5_UNorm_Block => vk_sys::VK_FORMAT_BC5_UNORM_BLOCK,
            Format::BC5_SNorm_Block => vk_sys::VK_FORMAT_BC5_SNORM_BLOCK,
            Format::BC6H_UFloat_Block => vk_sys::VK_FORMAT_BC6H_UFLOAT_BLOCK,
            Format::BC6H_SFloat_Block => vk_sys::VK_FORMAT_BC6H_SFLOAT_BLOCK,
            Format::BC7_UNorm_Block => vk_sys::VK_FORMAT_BC7_UNORM_BLOCK,
            Format::BC7_sRGB_Block => vk_sys::VK_FORMAT_BC7_SRGB_BLOCK,
            Format::ETC2_R8G8B8_UNorm_Block => vk_sys::VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK,
            Format::ETC2_R8G8B8_sRGB_Block => vk_sys::VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK,
            Format::ETC2_R8G8B8A1_UNorm_Block => vk_sys::VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK,
            Format::ETC2_R8G8B8A1_sRGB_Block => vk_sys::VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK,
            Format::ETC2_R8G8B8A8_UNorm_Block => vk_sys::VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK,
            Format::ETC2_R8G8B8A8_sRGB_Block => vk_sys::VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK,
            Format::EAC_R11_UNorm_Block => vk_sys::VK_FORMAT_EAC_R11_UNORM_BLOCK,
            Format::EAC_R11_SNorm_Block => vk_sys::VK_FORMAT_EAC_R11_SNORM_BLOCK,
            Format::EAC_R11G11_UNorm_Block => vk_sys::VK_FORMAT_EAC_R11G11_UNORM_BLOCK,
            Format::EAC_R11G11_SNorm_Block => vk_sys::VK_FORMAT_EAC_R11G11_SNORM_BLOCK,
            Format::ASTC_4x4_UNorm_Block => vk_sys::VK_FORMAT_ASTC_4x4_UNORM_BLOCK,
            Format::ASTC_4x4_sRGB_Block => vk_sys::VK_FORMAT_ASTC_4x4_SRGB_BLOCK,
            Format::ASTC_5x4_UNorm_Block => vk_sys::VK_FORMAT_ASTC_5x4_UNORM_BLOCK,
            Format::ASTC_5x4_sRGB_Block => vk_sys::VK_FORMAT_ASTC_5x4_SRGB_BLOCK,
            Format::ASTC_5x5_UNorm_Block => vk_sys::VK_FORMAT_ASTC_5x5_UNORM_BLOCK,
            Format::ASTC_5x5_sRGB_Block => vk_sys::VK_FORMAT_ASTC_5x5_SRGB_BLOCK,
            Format::ASTC_6x5_UNorm_Block => vk_sys::VK_FORMAT_ASTC_6x5_UNORM_BLOCK,
            Format::ASTC_6x5_sRGB_Block => vk_sys::VK_FORMAT_ASTC_6x5_SRGB_BLOCK,
            Format::ASTC_6x6_UNorm_Block => vk_sys::VK_FORMAT_ASTC_6x6_UNORM_BLOCK,
            Format::ASTC_6x6_sRGB_Block => vk_sys::VK_FORMAT_ASTC_6x6_SRGB_BLOCK,
            Format::ASTC_8x5_UNorm_Block => vk_sys::VK_FORMAT_ASTC_8x5_UNORM_BLOCK,
            Format::ASTC_8x5_sRGB_Block => vk_sys::VK_FORMAT_ASTC_8x5_SRGB_BLOCK,
            Format::ASTC_8x6_UNorm_Block => vk_sys::VK_FORMAT_ASTC_8x6_UNORM_BLOCK,
            Format::ASTC_8x6_sRGB_Block => vk_sys::VK_FORMAT_ASTC_8x6_SRGB_BLOCK,
            Format::ASTC_8x8_UNorm_Block => vk_sys::VK_FORMAT_ASTC_8x8_UNORM_BLOCK,
            Format::ASTC_8x8_sRGB_Block => vk_sys::VK_FORMAT_ASTC_8x8_SRGB_BLOCK,
            Format::ASTC_10x5_UNorm_Block => vk_sys::VK_FORMAT_ASTC_10x5_UNORM_BLOCK,
            Format::ASTC_10x5_sRGB_Block => vk_sys::VK_FORMAT_ASTC_10x5_SRGB_BLOCK,
            Format::ASTC_10x6_UNorm_Block => vk_sys::VK_FORMAT_ASTC_10x6_UNORM_BLOCK,
            Format::ASTC_10x6_sRGB_Block => vk_sys::VK_FORMAT_ASTC_10x6_SRGB_BLOCK,
            Format::ASTC_10x8_UNorm_Block => vk_sys::VK_FORMAT_ASTC_10x8_UNORM_BLOCK,
            Format::ASTC_10x8_sRGB_Block => vk_sys::VK_FORMAT_ASTC_10x8_SRGB_BLOCK,
            Format::ASTC_10x10_UNorm_Block => vk_sys::VK_FORMAT_ASTC_10x10_UNORM_BLOCK,
            Format::ASTC_10x10_sRGB_Block => vk_sys::VK_FORMAT_ASTC_10x10_SRGB_BLOCK,
            Format::ASTC_12x10_UNorm_Block => vk_sys::VK_FORMAT_ASTC_12x10_UNORM_BLOCK,
            Format::ASTC_12x10_sRGB_Block => vk_sys::VK_FORMAT_ASTC_12x10_SRGB_BLOCK,
            Format::ASTC_12x12_UNorm_Block => vk_sys::VK_FORMAT_ASTC_12x12_UNORM_BLOCK,
            Format::ASTC_12x12_sRGB_Block => vk_sys::VK_FORMAT_ASTC_12x12_SRGB_BLOCK,
            Format::Unknown(format) => format,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ImageType {
    Type1D,
    Type2D,
    Type3D,
    Unknown(vk_sys::VkImageType),
}

impl From<vk_sys::VkImageType> for ImageType {
    fn from(image_type: vk_sys::VkImageType) -> Self {
        match image_type {
            vk_sys::VK_IMAGE_TYPE_1D => ImageType::Type1D,
            vk_sys::VK_IMAGE_TYPE_2D => ImageType::Type2D,
            vk_sys::VK_IMAGE_TYPE_3D => ImageType::Type3D,
            _ => ImageType::Unknown(image_type),
        }
    }
}

impl From<ImageType> for vk_sys::VkImageType {
    fn from(image_type: ImageType) -> Self {
        match image_type {
            ImageType::Type1D => vk_sys::VK_IMAGE_TYPE_1D,
            ImageType::Type2D => vk_sys::VK_IMAGE_TYPE_2D,
            ImageType::Type3D => vk_sys::VK_IMAGE_TYPE_3D,
            ImageType::Unknown(image_type) => image_type,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ImageTiling {
    Optimal,
    Linear,
    Unknown(vk_sys::VkImageTiling),
}

impl From<vk_sys::VkImageTiling> for ImageTiling {
    fn from(tiling: vk_sys::VkImageTiling) -> Self {
        match tiling {
            vk_sys::VK_IMAGE_TILING_OPTIMAL => ImageTiling::Optimal,
            vk_sys::VK_IMAGE_TILING_LINEAR => ImageTiling::Linear,
            _ => ImageTiling::Unknown(tiling),
        }
    }
}

impl From<ImageTiling> for vk_sys::VkImageTiling {
    fn from(tiling: ImageTiling) -> Self {
        match tiling {
            ImageTiling::Optimal => vk_sys::VK_IMAGE_TILING_OPTIMAL,
            ImageTiling::Linear => vk_sys::VK_IMAGE_TILING_LINEAR,
            ImageTiling::Unknown(tiling) => tiling,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum PhysicalDeviceType {
    Other,
    IntegratedGpu,
    DiscreteGpu,
    VirtualGpu,
    Cpu,
    Unknown(vk_sys::VkPhysicalDeviceType),
}

impl From<vk_sys::VkPhysicalDeviceType> for PhysicalDeviceType {
    fn from(physical_device_type: vk_sys::VkPhysicalDeviceType) -> Self {
        match physical_device_type {
            vk_sys::VK_PHYSICAL_DEVICE_TYPE_OTHER => PhysicalDeviceType::Other,
            vk_sys::VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU => PhysicalDeviceType::IntegratedGpu,
            vk_sys::VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU => PhysicalDeviceType::DiscreteGpu,
            vk_sys::VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU => PhysicalDeviceType::VirtualGpu,
            vk_sys::VK_PHYSICAL_DEVICE_TYPE_CPU => PhysicalDeviceType::Cpu,
            _ => PhysicalDeviceType::Unknown(physical_device_type),
        }
    }
}

impl From<PhysicalDeviceType> for vk_sys::VkPhysicalDeviceType {
    fn from(physical_device_type: PhysicalDeviceType) -> Self {
        match physical_device_type {
            PhysicalDeviceType::Other => vk_sys::VK_PHYSICAL_DEVICE_TYPE_OTHER,
            PhysicalDeviceType::IntegratedGpu => vk_sys::VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU,
            PhysicalDeviceType::DiscreteGpu => vk_sys::VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU,
            PhysicalDeviceType::VirtualGpu => vk_sys::VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU,
            PhysicalDeviceType::Cpu => vk_sys::VK_PHYSICAL_DEVICE_TYPE_CPU,
            PhysicalDeviceType::Unknown(physical_device_type) => physical_device_type,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum QueryType {
    Occlusion,
    PipelineStatistics,
    Timestamp,
    Unknown(vk_sys::VkQueryType),
}

impl From<vk_sys::VkQueryType> for QueryType {
    fn from(query_type: vk_sys::VkQueryType) -> Self {
        match query_type {
            vk_sys::VK_QUERY_TYPE_OCCLUSION => QueryType::Occlusion,
            vk_sys::VK_QUERY_TYPE_PIPELINE_STATISTICS => QueryType::PipelineStatistics,
            vk_sys::VK_QUERY_TYPE_TIMESTAMP => QueryType::Timestamp,
            _ => QueryType::Unknown(query_type),
        }
    }
}

impl From<QueryType> for vk_sys::VkQueryType {
    fn from(query_type: QueryType) -> Self {
        match query_type {
            QueryType::Occlusion => vk_sys::VK_QUERY_TYPE_OCCLUSION,
            QueryType::PipelineStatistics => vk_sys::VK_QUERY_TYPE_PIPELINE_STATISTICS,
            QueryType::Timestamp => vk_sys::VK_QUERY_TYPE_TIMESTAMP,
            QueryType::Unknown(query_type) => query_type,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SharingMode {
    Exclusive,
    Concurrent,
    Unknown(vk_sys::VkSharingMode),
}

impl From<vk_sys::VkSharingMode> for SharingMode {
    fn from(mode: vk_sys::VkSharingMode) -> Self {
        match mode {
            vk_sys::VK_SHARING_MODE_EXCLUSIVE => SharingMode::Exclusive,
            vk_sys::VK_SHARING_MODE_CONCURRENT => SharingMode::Concurrent,
            _ => SharingMode::Unknown(mode),
        }
    }
}

impl From<SharingMode> for vk_sys::VkSharingMode {
    fn from(mode: SharingMode) -> Self {
        match mode {
            SharingMode::Exclusive => vk_sys::VK_SHARING_MODE_EXCLUSIVE,
            SharingMode::Concurrent => vk_sys::VK_SHARING_MODE_CONCURRENT,
            SharingMode::Unknown(mode) => mode,
        }
    }
}

#[derive(Debug, Copy, Clone)]
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
    Unknown(vk_sys::VkImageLayout),
}

impl From<vk_sys::VkImageLayout> for ImageLayout {
    fn from(layout: vk_sys::VkImageLayout) -> Self {
        match layout {
            vk_sys::VK_IMAGE_LAYOUT_UNDEFINED => ImageLayout::Undefined,
            vk_sys::VK_IMAGE_LAYOUT_GENERAL => ImageLayout::General,
            vk_sys::VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL => ImageLayout::ColorAttachmentOptimal,
            vk_sys::VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL => ImageLayout::DepthStencilAttachmentOptimal,
            vk_sys::VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL => ImageLayout::DepthStencilReadOnlyOptimal,
            vk_sys::VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL => ImageLayout::ShaderReadOnlyOptimal,
            vk_sys::VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL => ImageLayout::TransferSrcOptimal,
            vk_sys::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL => ImageLayout::TransferDstOptimal,
            vk_sys::VK_IMAGE_LAYOUT_PREINITIALIZED => ImageLayout::Preinitialized,
            _ => ImageLayout::Unknown(layout),
        }
    }
}

impl From<ImageLayout> for vk_sys::VkImageLayout {
    fn from(layout: ImageLayout) -> Self {
        match layout {
            ImageLayout::Undefined => vk_sys::VK_IMAGE_LAYOUT_UNDEFINED,
            ImageLayout::General => vk_sys::VK_IMAGE_LAYOUT_GENERAL,
            ImageLayout::ColorAttachmentOptimal => vk_sys::VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
            ImageLayout::DepthStencilAttachmentOptimal => vk_sys::VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
            ImageLayout::DepthStencilReadOnlyOptimal => vk_sys::VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL,
            ImageLayout::ShaderReadOnlyOptimal => vk_sys::VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL,
            ImageLayout::TransferSrcOptimal => vk_sys::VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
            ImageLayout::TransferDstOptimal => vk_sys::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
            ImageLayout::Preinitialized => vk_sys::VK_IMAGE_LAYOUT_PREINITIALIZED,
            ImageLayout::Unknown(layout) => layout,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ImageViewType {
    Type1D,
    Type2D,
    Type3D,
    TypeCube,
    Type1DArray,
    Type2DArray,
    TypeCubeArray,
    Unknown(vk_sys::VkImageViewType),
}

impl From<vk_sys::VkImageViewType> for ImageViewType {
    fn from(view_type: vk_sys::VkImageViewType) -> Self {
        match view_type {
            vk_sys::VK_IMAGE_VIEW_TYPE_1D => ImageViewType::Type1D,
            vk_sys::VK_IMAGE_VIEW_TYPE_2D => ImageViewType::Type2D,
            vk_sys::VK_IMAGE_VIEW_TYPE_3D => ImageViewType::Type3D,
            vk_sys::VK_IMAGE_VIEW_TYPE_CUBE => ImageViewType::TypeCube,
            vk_sys::VK_IMAGE_VIEW_TYPE_1D_ARRAY => ImageViewType::Type1DArray,
            vk_sys::VK_IMAGE_VIEW_TYPE_2D_ARRAY => ImageViewType::Type2DArray,
            vk_sys::VK_IMAGE_VIEW_TYPE_CUBE_ARRAY => ImageViewType::TypeCubeArray,
            _ => ImageViewType::Unknown(view_type),
        }
    }
}

impl From<ImageViewType> for vk_sys::VkImageViewType {
    fn from(view_type: ImageViewType) -> Self {
        match view_type {
            ImageViewType::Type1D => vk_sys::VK_IMAGE_VIEW_TYPE_1D,
            ImageViewType::Type2D => vk_sys::VK_IMAGE_VIEW_TYPE_2D,
            ImageViewType::Type3D => vk_sys::VK_IMAGE_VIEW_TYPE_3D,
            ImageViewType::TypeCube => vk_sys::VK_IMAGE_VIEW_TYPE_CUBE,
            ImageViewType::Type1DArray => vk_sys::VK_IMAGE_VIEW_TYPE_1D_ARRAY,
            ImageViewType::Type2DArray => vk_sys::VK_IMAGE_VIEW_TYPE_2D_ARRAY,
            ImageViewType::TypeCubeArray => vk_sys::VK_IMAGE_VIEW_TYPE_CUBE_ARRAY,
            ImageViewType::Unknown(view_type) => view_type,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ComponentSwizzle {
    Identity,
    Zero,
    One,
    R,
    G,
    B,
    A,
    Unknown(vk_sys::VkComponentSwizzle),
}

impl From<vk_sys::VkComponentSwizzle> for ComponentSwizzle {
    fn from(swizzle: vk_sys::VkComponentSwizzle) -> Self {
        match swizzle {
            vk_sys::VK_COMPONENT_SWIZZLE_IDENTITY => ComponentSwizzle::Identity,
            vk_sys::VK_COMPONENT_SWIZZLE_ZERO => ComponentSwizzle::Zero,
            vk_sys::VK_COMPONENT_SWIZZLE_ONE => ComponentSwizzle::One,
            vk_sys::VK_COMPONENT_SWIZZLE_R => ComponentSwizzle::R,
            vk_sys::VK_COMPONENT_SWIZZLE_G => ComponentSwizzle::G,
            vk_sys::VK_COMPONENT_SWIZZLE_B => ComponentSwizzle::B,
            vk_sys::VK_COMPONENT_SWIZZLE_A => ComponentSwizzle::A,
            _ => ComponentSwizzle::Unknown(swizzle),
        }
    }
}

impl From<ComponentSwizzle> for vk_sys::VkComponentSwizzle {
    fn from(swizzle: ComponentSwizzle) -> Self {
        match swizzle {
            ComponentSwizzle::Identity => vk_sys::VK_COMPONENT_SWIZZLE_IDENTITY,
            ComponentSwizzle::Zero => vk_sys::VK_COMPONENT_SWIZZLE_ZERO,
            ComponentSwizzle::One => vk_sys::VK_COMPONENT_SWIZZLE_ONE,
            ComponentSwizzle::R => vk_sys::VK_COMPONENT_SWIZZLE_R,
            ComponentSwizzle::G => vk_sys::VK_COMPONENT_SWIZZLE_G,
            ComponentSwizzle::B => vk_sys::VK_COMPONENT_SWIZZLE_B,
            ComponentSwizzle::A => vk_sys::VK_COMPONENT_SWIZZLE_A,
            ComponentSwizzle::Unknown(swizzle) => swizzle,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum VertexInputRate {
    Vertex,
    Instance,
    Unknown(vk_sys::VkVertexInputRate),
}

impl From<vk_sys::VkVertexInputRate> for VertexInputRate {
    fn from(rate: vk_sys::VkVertexInputRate) -> Self {
        match rate {
            vk_sys::VK_VERTEX_INPUT_RATE_VERTEX => VertexInputRate::Vertex,
            vk_sys::VK_VERTEX_INPUT_RATE_INSTANCE => VertexInputRate::Instance,
            _ => VertexInputRate::Unknown(rate),
        }
    }
}

impl From<VertexInputRate> for vk_sys::VkVertexInputRate {
    fn from(rate: VertexInputRate) -> Self {
        match rate {
            VertexInputRate::Vertex => vk_sys::VK_VERTEX_INPUT_RATE_VERTEX,
            VertexInputRate::Instance => vk_sys::VK_VERTEX_INPUT_RATE_INSTANCE,
            VertexInputRate::Unknown(rate) => rate,
        }
    }
}

#[derive(Debug, Copy, Clone)]
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
    Unknown(vk_sys::VkPrimitiveTopology)
}

impl From<vk_sys::VkPrimitiveTopology> for PrimitiveTopology {
    fn from(topology: vk_sys::VkPrimitiveTopology) -> Self {
        match topology {
            vk_sys::VK_PRIMITIVE_TOPOLOGY_POINT_LIST => PrimitiveTopology::PointList,
            vk_sys::VK_PRIMITIVE_TOPOLOGY_LINE_LIST => PrimitiveTopology::LineList,
            vk_sys::VK_PRIMITIVE_TOPOLOGY_LINE_STRIP => PrimitiveTopology::LineStrip,
            vk_sys::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST => PrimitiveTopology::TriangleList,
            vk_sys::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP => PrimitiveTopology::TriangleStrip,
            vk_sys::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_FAN => PrimitiveTopology::TriangleFan,
            vk_sys::VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY => PrimitiveTopology::LineListWithAdjacency,
            vk_sys::VK_PRIMITIVE_TOPOLOGY_LINE_STRIP_WITH_ADJACENCY => PrimitiveTopology::LineStripWithAdjacency,
            vk_sys::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY => PrimitiveTopology::TriangleListWithAdjacency,
            vk_sys::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_WITH_ADJACENCY => PrimitiveTopology::TriangleStripWithAdjacency,
            vk_sys::VK_PRIMITIVE_TOPOLOGY_PATCH_LIST => PrimitiveTopology::PatchList,
            _ => PrimitiveTopology::Unknown(topology),
        }
    }
}

impl From<PrimitiveTopology> for vk_sys::VkPrimitiveTopology {
    fn from(topology: PrimitiveTopology) -> Self {
        match topology {
            PrimitiveTopology::PointList => vk_sys::VK_PRIMITIVE_TOPOLOGY_POINT_LIST,
            PrimitiveTopology::LineList => vk_sys::VK_PRIMITIVE_TOPOLOGY_LINE_LIST,
            PrimitiveTopology::LineStrip => vk_sys::VK_PRIMITIVE_TOPOLOGY_LINE_STRIP,
            PrimitiveTopology::TriangleList => vk_sys::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
            PrimitiveTopology::TriangleStrip => vk_sys::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP,
            PrimitiveTopology::TriangleFan => vk_sys::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_FAN,
            PrimitiveTopology::LineListWithAdjacency => vk_sys::VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY,
            PrimitiveTopology::LineStripWithAdjacency => vk_sys::VK_PRIMITIVE_TOPOLOGY_LINE_STRIP_WITH_ADJACENCY,
            PrimitiveTopology::TriangleListWithAdjacency => vk_sys::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY,
            PrimitiveTopology::TriangleStripWithAdjacency => vk_sys::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_WITH_ADJACENCY,
            PrimitiveTopology::PatchList => vk_sys::VK_PRIMITIVE_TOPOLOGY_PATCH_LIST,
            PrimitiveTopology::Unknown(topology) => topology,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum PolygonMode {
    Fill,
    Line,
    Point,
    Unknown(vk_sys::VkPolygonMode),
}

impl From<vk_sys::VkPolygonMode> for PolygonMode {
    fn from(mode: vk_sys::VkPolygonMode) -> Self {
        match mode {
            vk_sys::VK_POLYGON_MODE_FILL => PolygonMode::Fill,
            vk_sys::VK_POLYGON_MODE_LINE => PolygonMode::Line,
            vk_sys::VK_POLYGON_MODE_POINT => PolygonMode::Point,
            _ => PolygonMode::Unknown(mode),
        }
    }
}

impl From<PolygonMode> for vk_sys::VkPolygonMode {
    fn from(mode: PolygonMode) -> Self {
        match mode {
            PolygonMode::Fill => vk_sys::VK_POLYGON_MODE_FILL,
            PolygonMode::Line => vk_sys::VK_POLYGON_MODE_LINE,
            PolygonMode::Point => vk_sys::VK_POLYGON_MODE_POINT,
            PolygonMode::Unknown(mode) => mode,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum FrontFace {
    CounterClockwise,
    Clockwise,
    Unknown(vk_sys::VkFrontFace),
}

impl From<vk_sys::VkFrontFace> for FrontFace {
    fn from(face: vk_sys::VkFrontFace) -> Self {
        match face {
            vk_sys::VK_FRONT_FACE_COUNTER_CLOCKWISE => FrontFace::CounterClockwise,
            vk_sys::VK_FRONT_FACE_CLOCKWISE => FrontFace::Clockwise,
            _ => FrontFace::Unknown(face),
        }
    }
}

impl From<FrontFace> for vk_sys::VkFrontFace {
    fn from(face: FrontFace) -> Self {
        match face {
            FrontFace::CounterClockwise => vk_sys::VK_FRONT_FACE_COUNTER_CLOCKWISE,
            FrontFace::Clockwise => vk_sys::VK_FRONT_FACE_CLOCKWISE,
            FrontFace::Unknown(face) => face,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CompareOp {
    Never,
    Less,
    Equal,
    LessOrEqual,
    Greater,
    NotEqual,
    GreaterOrEqual,
    Always,
    Unknown(vk_sys::VkCompareOp),
}

impl From<vk_sys::VkCompareOp> for CompareOp {
    fn from(op: vk_sys::VkCompareOp) -> Self {
        match op {
            vk_sys::VK_COMPARE_OP_NEVER => CompareOp::Never,
            vk_sys::VK_COMPARE_OP_LESS => CompareOp::Less,
            vk_sys::VK_COMPARE_OP_EQUAL => CompareOp::Equal,
            vk_sys::VK_COMPARE_OP_LESS_OR_EQUAL => CompareOp::LessOrEqual,
            vk_sys::VK_COMPARE_OP_GREATER => CompareOp::Greater,
            vk_sys::VK_COMPARE_OP_NOT_EQUAL => CompareOp::NotEqual,
            vk_sys::VK_COMPARE_OP_GREATER_OR_EQUAL => CompareOp::GreaterOrEqual,
            vk_sys::VK_COMPARE_OP_ALWAYS => CompareOp::Always,
            _ => CompareOp::Unknown(op),
        }
    }
}

impl From<CompareOp> for vk_sys::VkCompareOp {
    fn from(op: CompareOp) -> Self {
        match op {
            CompareOp::Never => vk_sys::VK_COMPARE_OP_NEVER,
            CompareOp::Less => vk_sys::VK_COMPARE_OP_LESS,
            CompareOp::Equal => vk_sys::VK_COMPARE_OP_EQUAL,
            CompareOp::LessOrEqual => vk_sys::VK_COMPARE_OP_LESS_OR_EQUAL,
            CompareOp::Greater => vk_sys::VK_COMPARE_OP_GREATER,
            CompareOp::NotEqual => vk_sys::VK_COMPARE_OP_NOT_EQUAL,
            CompareOp::GreaterOrEqual => vk_sys::VK_COMPARE_OP_GREATER_OR_EQUAL,
            CompareOp::Always => vk_sys::VK_COMPARE_OP_ALWAYS,
            CompareOp::Unknown(op) => op,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum StencilOp {
    Keep,
    Zero,
    Replace,
    IncrementAndClamp,
    DecrementAndClamp,
    Invert,
    IncrementAndWrap,
    DecrementAndWrap,
    Unknown(vk_sys::VkStencilOp),
}

impl From<vk_sys::VkStencilOp> for StencilOp {
    fn from(op: vk_sys::VkStencilOp) -> Self {
        match op {
            vk_sys::VK_STENCIL_OP_KEEP => StencilOp::Keep,
            vk_sys::VK_STENCIL_OP_ZERO => StencilOp::Zero,
            vk_sys::VK_STENCIL_OP_REPLACE => StencilOp::Replace,
            vk_sys::VK_STENCIL_OP_INCREMENT_AND_CLAMP => StencilOp::IncrementAndClamp,
            vk_sys::VK_STENCIL_OP_DECREMENT_AND_CLAMP => StencilOp::DecrementAndClamp,
            vk_sys::VK_STENCIL_OP_INVERT => StencilOp::Invert,
            vk_sys::VK_STENCIL_OP_INCREMENT_AND_WRAP => StencilOp::IncrementAndWrap,
            vk_sys::VK_STENCIL_OP_DECREMENT_AND_WRAP => StencilOp::DecrementAndWrap,
            _ => StencilOp::Unknown(op),
        }
    }
}

impl From<StencilOp> for vk_sys::VkStencilOp {
    fn from(op: StencilOp) -> Self {
        match op {
            StencilOp::Keep => vk_sys::VK_STENCIL_OP_KEEP,
            StencilOp::Zero => vk_sys::VK_STENCIL_OP_ZERO,
            StencilOp::Replace => vk_sys::VK_STENCIL_OP_REPLACE,
            StencilOp::IncrementAndClamp => vk_sys::VK_STENCIL_OP_INCREMENT_AND_CLAMP,
            StencilOp::DecrementAndClamp => vk_sys::VK_STENCIL_OP_DECREMENT_AND_CLAMP,
            StencilOp::Invert => vk_sys::VK_STENCIL_OP_INVERT,
            StencilOp::IncrementAndWrap => vk_sys::VK_STENCIL_OP_INCREMENT_AND_WRAP,
            StencilOp::DecrementAndWrap => vk_sys::VK_STENCIL_OP_DECREMENT_AND_WRAP,
            StencilOp::Unknown(op) => op,
        }
    }
}

#[derive(Debug, Copy, Clone)]
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
    Unknown(vk_sys::VkLogicOp),
}

impl From<vk_sys::VkLogicOp> for LogicOp {
    fn from(op: vk_sys::VkLogicOp) -> Self {
        match op {
            vk_sys::VK_LOGIC_OP_CLEAR => LogicOp::Clear,
            vk_sys::VK_LOGIC_OP_AND => LogicOp::And,
            vk_sys::VK_LOGIC_OP_AND_REVERSE => LogicOp::AndReverse,
            vk_sys::VK_LOGIC_OP_COPY => LogicOp::Copy,
            vk_sys::VK_LOGIC_OP_AND_INVERTED => LogicOp::AndInverted,
            vk_sys::VK_LOGIC_OP_NO_OP => LogicOp::NoOp,
            vk_sys::VK_LOGIC_OP_XOR => LogicOp::Xor,
            vk_sys::VK_LOGIC_OP_OR => LogicOp::Or,
            vk_sys::VK_LOGIC_OP_NOR => LogicOp::Nor,
            vk_sys::VK_LOGIC_OP_EQUIVALENT => LogicOp::Equivalent,
            vk_sys::VK_LOGIC_OP_INVERT => LogicOp::Invert,
            vk_sys::VK_LOGIC_OP_OR_REVERSE => LogicOp::OrReverse,
            vk_sys::VK_LOGIC_OP_COPY_INVERTED => LogicOp::CopyInverted,
            vk_sys::VK_LOGIC_OP_OR_INVERTED => LogicOp::OrInverted,
            vk_sys::VK_LOGIC_OP_NAND => LogicOp::Nand,
            vk_sys::VK_LOGIC_OP_SET => LogicOp::Set,
            _ => LogicOp::Unknown(op),
        }
    }
}

impl From<LogicOp> for vk_sys::VkLogicOp {
    fn from(op: LogicOp) -> Self {
        match op {
            LogicOp::Clear => vk_sys::VK_LOGIC_OP_CLEAR,
            LogicOp::And => vk_sys::VK_LOGIC_OP_AND,
            LogicOp::AndReverse => vk_sys::VK_LOGIC_OP_AND_REVERSE,
            LogicOp::Copy => vk_sys::VK_LOGIC_OP_COPY,
            LogicOp::AndInverted => vk_sys::VK_LOGIC_OP_AND_INVERTED,
            LogicOp::NoOp => vk_sys::VK_LOGIC_OP_NO_OP,
            LogicOp::Xor => vk_sys::VK_LOGIC_OP_XOR,
            LogicOp::Or => vk_sys::VK_LOGIC_OP_OR,
            LogicOp::Nor => vk_sys::VK_LOGIC_OP_NOR,
            LogicOp::Equivalent => vk_sys::VK_LOGIC_OP_EQUIVALENT,
            LogicOp::Invert => vk_sys::VK_LOGIC_OP_INVERT,
            LogicOp::OrReverse => vk_sys::VK_LOGIC_OP_OR_REVERSE,
            LogicOp::CopyInverted => vk_sys::VK_LOGIC_OP_COPY_INVERTED,
            LogicOp::OrInverted => vk_sys::VK_LOGIC_OP_OR_INVERTED,
            LogicOp::Nand => vk_sys::VK_LOGIC_OP_NAND,
            LogicOp::Set => vk_sys::VK_LOGIC_OP_SET,
            LogicOp::Unknown(op) => op,
        }
    }
}

#[derive(Debug, Copy, Clone)]
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
    Unknown(vk_sys::VkBlendFactor),
}

impl From<vk_sys::VkBlendFactor> for BlendFactor {
    fn from(factor: vk_sys::VkBlendFactor) -> Self {
        match factor {
            vk_sys::VK_BLEND_FACTOR_ZERO => BlendFactor::Zero,
            vk_sys::VK_BLEND_FACTOR_ONE => BlendFactor::One,
            vk_sys::VK_BLEND_FACTOR_SRC_COLOR => BlendFactor::SrcColor,
            vk_sys::VK_BLEND_FACTOR_ONE_MINUS_SRC_COLOR => BlendFactor::OneMinusSrcColor,
            vk_sys::VK_BLEND_FACTOR_DST_COLOR => BlendFactor::DstColor,
            vk_sys::VK_BLEND_FACTOR_ONE_MINUS_DST_COLOR => BlendFactor::OneMinusDstColor,
            vk_sys::VK_BLEND_FACTOR_SRC_ALPHA => BlendFactor::SrcAlpha,
            vk_sys::VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA => BlendFactor::OneMinusSrcAlpha,
            vk_sys::VK_BLEND_FACTOR_DST_ALPHA => BlendFactor::DstAlpha,
            vk_sys::VK_BLEND_FACTOR_ONE_MINUS_DST_ALPHA => BlendFactor::OneMinusDstAlpha,
            vk_sys::VK_BLEND_FACTOR_CONSTANT_COLOR => BlendFactor::ConstantColor,
            vk_sys::VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR => BlendFactor::OneMinusConstantColor,
            vk_sys::VK_BLEND_FACTOR_CONSTANT_ALPHA => BlendFactor::ConstantAlpha,
            vk_sys::VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA => BlendFactor::OneMinusConstantAlpha,
            vk_sys::VK_BLEND_FACTOR_SRC_ALPHA_SATURATE => BlendFactor::SrcAlphaSaturate,
            vk_sys::VK_BLEND_FACTOR_SRC1_COLOR => BlendFactor::Src1Color,
            vk_sys::VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR => BlendFactor::OneMinusSrc1Color,
            vk_sys::VK_BLEND_FACTOR_SRC1_ALPHA => BlendFactor::Src1Alpha,
            vk_sys::VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA => BlendFactor::OneMinusSrc1Alpha,
            _ => BlendFactor::Unknown(factor),
        }
    }
}

impl From<BlendFactor> for vk_sys::VkBlendFactor {
    fn from(factor: BlendFactor) -> Self {
        match factor {
            BlendFactor::Zero => vk_sys::VK_BLEND_FACTOR_ZERO,
            BlendFactor::One => vk_sys::VK_BLEND_FACTOR_ONE,
            BlendFactor::SrcColor => vk_sys::VK_BLEND_FACTOR_SRC_COLOR,
            BlendFactor::OneMinusSrcColor => vk_sys::VK_BLEND_FACTOR_ONE_MINUS_SRC_COLOR,
            BlendFactor::DstColor => vk_sys::VK_BLEND_FACTOR_DST_COLOR,
            BlendFactor::OneMinusDstColor => vk_sys::VK_BLEND_FACTOR_ONE_MINUS_DST_COLOR,
            BlendFactor::SrcAlpha => vk_sys::VK_BLEND_FACTOR_SRC_ALPHA,
            BlendFactor::OneMinusSrcAlpha => vk_sys::VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA,
            BlendFactor::DstAlpha => vk_sys::VK_BLEND_FACTOR_DST_ALPHA,
            BlendFactor::OneMinusDstAlpha => vk_sys::VK_BLEND_FACTOR_ONE_MINUS_DST_ALPHA,
            BlendFactor::ConstantColor => vk_sys::VK_BLEND_FACTOR_CONSTANT_COLOR,
            BlendFactor::OneMinusConstantColor => vk_sys::VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR,
            BlendFactor::ConstantAlpha => vk_sys::VK_BLEND_FACTOR_CONSTANT_ALPHA,
            BlendFactor::OneMinusConstantAlpha => vk_sys::VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA,
            BlendFactor::SrcAlphaSaturate => vk_sys::VK_BLEND_FACTOR_SRC_ALPHA_SATURATE,
            BlendFactor::Src1Color => vk_sys::VK_BLEND_FACTOR_SRC1_COLOR,
            BlendFactor::OneMinusSrc1Color => vk_sys::VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR,
            BlendFactor::Src1Alpha => vk_sys::VK_BLEND_FACTOR_SRC1_ALPHA,
            BlendFactor::OneMinusSrc1Alpha => vk_sys::VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA,
            BlendFactor::Unknown(factor) => factor,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum BlendOp {
    Add,
    Subtract,
    ReverseSubtract,
    Min,
    Max,
    Unknown(vk_sys::VkBlendOp),
}

impl From<vk_sys::VkBlendOp> for BlendOp {
    fn from(op: vk_sys::VkBlendOp) -> Self {
        match op {
            vk_sys::VK_BLEND_OP_ADD => BlendOp::Add,
            vk_sys::VK_BLEND_OP_SUBTRACT => BlendOp::Subtract,
            vk_sys::VK_BLEND_OP_REVERSE_SUBTRACT => BlendOp::ReverseSubtract,
            vk_sys::VK_BLEND_OP_MIN => BlendOp::Min,
            vk_sys::VK_BLEND_OP_MAX => BlendOp::Max,
            _ => BlendOp::Unknown(op),
        }
    }
}

impl From<BlendOp> for vk_sys::VkBlendOp {
    fn from(op: BlendOp) -> Self {
        match op {
            BlendOp::Add => vk_sys::VK_BLEND_OP_ADD,
            BlendOp::Subtract => vk_sys::VK_BLEND_OP_SUBTRACT,
            BlendOp::ReverseSubtract => vk_sys::VK_BLEND_OP_REVERSE_SUBTRACT,
            BlendOp::Min => vk_sys::VK_BLEND_OP_MIN,
            BlendOp::Max => vk_sys::VK_BLEND_OP_MAX,
            BlendOp::Unknown(op) => op,
        }
    }
}

#[derive(Debug, Copy, Clone)]
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
    Unknown(vk_sys::VkDynamicState),
}

impl From<vk_sys::VkDynamicState> for DynamicState {
    fn from(state: vk_sys::VkDynamicState) -> Self {
        match state {
            vk_sys::VK_DYNAMIC_STATE_VIEWPORT => DynamicState::Viewport,
            vk_sys::VK_DYNAMIC_STATE_SCISSOR => DynamicState::Scissor,
            vk_sys::VK_DYNAMIC_STATE_LINE_WIDTH => DynamicState::LineWidth,
            vk_sys::VK_DYNAMIC_STATE_DEPTH_BIAS => DynamicState::DepthBias,
            vk_sys::VK_DYNAMIC_STATE_BLEND_CONSTANTS => DynamicState::BlendConstants,
            vk_sys::VK_DYNAMIC_STATE_DEPTH_BOUNDS => DynamicState::DepthBounds,
            vk_sys::VK_DYNAMIC_STATE_STENCIL_COMPARE_MASK => DynamicState::StencilCompareMask,
            vk_sys::VK_DYNAMIC_STATE_STENCIL_WRITE_MASK => DynamicState::StencilWriteMask,
            vk_sys::VK_DYNAMIC_STATE_STENCIL_REFERENCE => DynamicState::StencilReference,
            _ => DynamicState::Unknown(state),
        }
    }
}

impl From<DynamicState> for vk_sys::VkDynamicState {
    fn from(state: DynamicState) -> Self {
        match state {
            DynamicState::Viewport => vk_sys::VK_DYNAMIC_STATE_VIEWPORT,
            DynamicState::Scissor => vk_sys::VK_DYNAMIC_STATE_SCISSOR,
            DynamicState::LineWidth => vk_sys::VK_DYNAMIC_STATE_LINE_WIDTH,
            DynamicState::DepthBias => vk_sys::VK_DYNAMIC_STATE_DEPTH_BIAS,
            DynamicState::BlendConstants => vk_sys::VK_DYNAMIC_STATE_BLEND_CONSTANTS,
            DynamicState::DepthBounds => vk_sys::VK_DYNAMIC_STATE_DEPTH_BOUNDS,
            DynamicState::StencilCompareMask => vk_sys::VK_DYNAMIC_STATE_STENCIL_COMPARE_MASK,
            DynamicState::StencilWriteMask => vk_sys::VK_DYNAMIC_STATE_STENCIL_WRITE_MASK,
            DynamicState::StencilReference => vk_sys::VK_DYNAMIC_STATE_STENCIL_REFERENCE,
            DynamicState::Unknown(state) => state,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Filter {
    Nearest,
    Linear,
    Unknown(vk_sys::VkFilter),
}

impl From<vk_sys::VkFilter> for Filter {
    fn from(filter: vk_sys::VkFilter) -> Self {
        match filter {
            vk_sys::VK_FILTER_NEAREST => Filter::Nearest,
            vk_sys::VK_FILTER_LINEAR => Filter::Linear,
            _ => Filter::Unknown(filter),
        }
    }
}

impl From<Filter> for vk_sys::VkFilter {
    fn from(filter: Filter) -> Self {
        match filter {
            Filter::Nearest => vk_sys::VK_FILTER_NEAREST,
            Filter::Linear => vk_sys::VK_FILTER_LINEAR,
            Filter::Unknown(filter) => filter,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SamplerMipmapMode {
    Nearest,
    Linear,
    Unknown(vk_sys::VkSamplerMipmapMode),
}

impl From<vk_sys::VkSamplerMipmapMode> for SamplerMipmapMode {
    fn from(mode: vk_sys::VkSamplerMipmapMode) -> Self {
        match mode {
            vk_sys::VK_SAMPLER_MIPMAP_MODE_NEAREST => SamplerMipmapMode::Nearest,
            vk_sys::VK_SAMPLER_MIPMAP_MODE_LINEAR => SamplerMipmapMode::Linear,
            _ => SamplerMipmapMode::Unknown(mode),
        }
    }
}

impl From<SamplerMipmapMode> for vk_sys::VkSamplerMipmapMode {
    fn from(mode: SamplerMipmapMode) -> Self {
        match mode {
            SamplerMipmapMode::Nearest => vk_sys::VK_SAMPLER_MIPMAP_MODE_NEAREST,
            SamplerMipmapMode::Linear => vk_sys::VK_SAMPLER_MIPMAP_MODE_LINEAR,
            SamplerMipmapMode::Unknown(mode) => mode,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SamplerAddressMode {
    Repeat,
    MirroredRepeat,
    ClampToEdge,
    ClampToBorder,
    MirrorClampToEdge,
    Unknown(vk_sys::VkSamplerAddressMode),
}

impl From<vk_sys::VkSamplerAddressMode> for SamplerAddressMode {
    fn from(mode: vk_sys::VkSamplerAddressMode) -> Self {
        match mode {
            vk_sys::VK_SAMPLER_ADDRESS_MODE_REPEAT => SamplerAddressMode::Repeat,
            vk_sys::VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT => SamplerAddressMode::MirroredRepeat,
            vk_sys::VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE => SamplerAddressMode::ClampToEdge,
            vk_sys::VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER => SamplerAddressMode::ClampToBorder,
            vk_sys::VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE => SamplerAddressMode::MirrorClampToEdge,
            _ => SamplerAddressMode::Unknown(mode),
        }
    }
}

impl From<SamplerAddressMode> for vk_sys::VkSamplerAddressMode {
    fn from(mode: SamplerAddressMode) -> Self {
        match mode {
            SamplerAddressMode::Repeat => vk_sys::VK_SAMPLER_ADDRESS_MODE_REPEAT,
            SamplerAddressMode::MirroredRepeat => vk_sys::VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT,
            SamplerAddressMode::ClampToEdge => vk_sys::VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE,
            SamplerAddressMode::ClampToBorder => vk_sys::VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER,
            SamplerAddressMode::MirrorClampToEdge => vk_sys::VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE,
            SamplerAddressMode::Unknown(mode) => mode,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum BorderColor {
    FloatTransparentBlack,
    IntTransparentBlack,
    FloatOpaqueBlack,
    IntOpaqueBlack,
    FloatOpaqueWhite,
    IntOpaqueWhite,
    Unknown(vk_sys::VkBorderColor),
}

impl From<vk_sys::VkBorderColor> for BorderColor {
    fn from(color: vk_sys::VkBorderColor) -> Self {
        match color {
            vk_sys::VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK => BorderColor::FloatTransparentBlack,
            vk_sys::VK_BORDER_COLOR_INT_TRANSPARENT_BLACK => BorderColor::IntTransparentBlack,
            vk_sys::VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK => BorderColor::FloatOpaqueBlack,
            vk_sys::VK_BORDER_COLOR_INT_OPAQUE_BLACK => BorderColor::IntOpaqueBlack,
            vk_sys::VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE => BorderColor::FloatOpaqueWhite,
            vk_sys::VK_BORDER_COLOR_INT_OPAQUE_WHITE => BorderColor::IntOpaqueWhite,
            _ => BorderColor::Unknown(color),
        }
    }
}

impl From<BorderColor> for vk_sys::VkBorderColor {
    fn from(color: BorderColor) -> Self {
        match color {
            BorderColor::FloatTransparentBlack => vk_sys::VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK,
            BorderColor::IntTransparentBlack => vk_sys::VK_BORDER_COLOR_INT_TRANSPARENT_BLACK,
            BorderColor::FloatOpaqueBlack => vk_sys::VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK,
            BorderColor::IntOpaqueBlack => vk_sys::VK_BORDER_COLOR_INT_OPAQUE_BLACK,
            BorderColor::FloatOpaqueWhite => vk_sys::VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE,
            BorderColor::IntOpaqueWhite => vk_sys::VK_BORDER_COLOR_INT_OPAQUE_WHITE,
            BorderColor::Unknown(color) => color,
        }
    }
}

#[derive(Debug, Copy, Clone)]
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
    Unknown(vk_sys::VkDescriptorType),
}

impl From<vk_sys::VkDescriptorType> for DescriptorType {
    fn from(descriptor_type: vk_sys::VkDescriptorType) -> Self {
        match descriptor_type {
            vk_sys::VK_DESCRIPTOR_TYPE_SAMPLER => DescriptorType::Sampler,
            vk_sys::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER => DescriptorType::CombinedImageSampler,
            vk_sys::VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE => DescriptorType::SampledImage,
            vk_sys::VK_DESCRIPTOR_TYPE_STORAGE_IMAGE => DescriptorType::StorageImage,
            vk_sys::VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER => DescriptorType::UniformTexelBuffer,
            vk_sys::VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER => DescriptorType::StorageTexelBuffer,
            vk_sys::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER => DescriptorType::UniformBuffer,
            vk_sys::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER => DescriptorType::StorageBuffer,
            vk_sys::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC => DescriptorType::UniformBufferDynamic,
            vk_sys::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC => DescriptorType::StorageBufferDynamic,
            vk_sys::VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT => DescriptorType::InputAttachment,
            _ => DescriptorType::Unknown(descriptor_type),
        }
    }
}

impl From<DescriptorType> for vk_sys::VkDescriptorType {
    fn from(descriptor_type: DescriptorType) -> Self {
        match descriptor_type {
            DescriptorType::Sampler => vk_sys::VK_DESCRIPTOR_TYPE_SAMPLER,
            DescriptorType::CombinedImageSampler => vk_sys::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
            DescriptorType::SampledImage => vk_sys::VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE,
            DescriptorType::StorageImage => vk_sys::VK_DESCRIPTOR_TYPE_STORAGE_IMAGE,
            DescriptorType::UniformTexelBuffer => vk_sys::VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER,
            DescriptorType::StorageTexelBuffer => vk_sys::VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER,
            DescriptorType::UniformBuffer => vk_sys::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,
            DescriptorType::StorageBuffer => vk_sys::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
            DescriptorType::UniformBufferDynamic => vk_sys::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC,
            DescriptorType::StorageBufferDynamic => vk_sys::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC,
            DescriptorType::InputAttachment => vk_sys::VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT,
            DescriptorType::Unknown(descriptor_type) => descriptor_type,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum AttachmentLoadOp {
    Load,
    Clear,
    DontCare,
    Unknown(vk_sys::VkAttachmentLoadOp),
}

impl From<vk_sys::VkAttachmentLoadOp> for AttachmentLoadOp {
    fn from(op: vk_sys::VkAttachmentLoadOp) -> Self {
        match op {
            vk_sys::VK_ATTACHMENT_LOAD_OP_LOAD => AttachmentLoadOp::Load,
            vk_sys::VK_ATTACHMENT_LOAD_OP_CLEAR => AttachmentLoadOp::Clear,
            vk_sys::VK_ATTACHMENT_LOAD_OP_DONT_CARE => AttachmentLoadOp::DontCare,
            _ => AttachmentLoadOp::Unknown(op),
        }
    }
}

impl From<AttachmentLoadOp> for vk_sys::VkAttachmentLoadOp {
    fn from(op: AttachmentLoadOp) -> Self {
        match op {
            AttachmentLoadOp::Load => vk_sys::VK_ATTACHMENT_LOAD_OP_LOAD,
            AttachmentLoadOp::Clear => vk_sys::VK_ATTACHMENT_LOAD_OP_CLEAR,
            AttachmentLoadOp::DontCare => vk_sys::VK_ATTACHMENT_LOAD_OP_DONT_CARE,
            AttachmentLoadOp::Unknown(op) => op,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum AttachmentStoreOp {
    Store,
    DontCare,
    Unknown(vk_sys::VkAttachmentStoreOp),
}

impl From<vk_sys::VkAttachmentStoreOp> for AttachmentStoreOp {
    fn from(op: vk_sys::VkAttachmentStoreOp) -> Self {
        match op {
            vk_sys::VK_ATTACHMENT_STORE_OP_STORE => AttachmentStoreOp::Store,
            vk_sys::VK_ATTACHMENT_STORE_OP_DONT_CARE => AttachmentStoreOp::DontCare,
            _ => AttachmentStoreOp::Unknown(op),
        }
    }
}

impl From<AttachmentStoreOp> for vk_sys::VkAttachmentStoreOp {
    fn from(op: AttachmentStoreOp) -> Self {
        match op {
            AttachmentStoreOp::Store => vk_sys::VK_ATTACHMENT_STORE_OP_STORE,
            AttachmentStoreOp::DontCare => vk_sys::VK_ATTACHMENT_STORE_OP_DONT_CARE,
            AttachmentStoreOp::Unknown(op) => op,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum PipelineBindPoint {
    Graphics,
    Compute,
    Unknown(vk_sys::VkPipelineBindPoint),
}

impl From<vk_sys::VkPipelineBindPoint> for PipelineBindPoint {
    fn from(bind_point: vk_sys::VkPipelineBindPoint) -> Self {
        match bind_point {
            vk_sys::VK_PIPELINE_BIND_POINT_GRAPHICS => PipelineBindPoint::Graphics,
            vk_sys::VK_PIPELINE_BIND_POINT_COMPUTE => PipelineBindPoint::Compute,
            _ => PipelineBindPoint::Unknown(bind_point),
        }
    }
}

impl From<PipelineBindPoint> for vk_sys::VkPipelineBindPoint {
    fn from(bind_point: PipelineBindPoint) -> Self {
        match bind_point {
            PipelineBindPoint::Graphics => vk_sys::VK_PIPELINE_BIND_POINT_GRAPHICS,
            PipelineBindPoint::Compute => vk_sys::VK_PIPELINE_BIND_POINT_COMPUTE,
            PipelineBindPoint::Unknown(bind_point) => bind_point,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CommandBufferLevel {
    Primary,
    Secondary,
    Unknown(vk_sys::VkCommandBufferLevel),
}

impl From<vk_sys::VkCommandBufferLevel> for CommandBufferLevel {
    fn from(level: vk_sys::VkCommandBufferLevel) -> Self {
        match level {
            vk_sys::VK_COMMAND_BUFFER_LEVEL_PRIMARY => CommandBufferLevel::Primary,
            vk_sys::VK_COMMAND_BUFFER_LEVEL_SECONDARY => CommandBufferLevel::Secondary,
            _ => CommandBufferLevel::Unknown(level),
        }
    }
}

impl From<CommandBufferLevel> for vk_sys::VkCommandBufferLevel {
    fn from(level: CommandBufferLevel) -> Self {
        match level {
            CommandBufferLevel::Primary => vk_sys::VK_COMMAND_BUFFER_LEVEL_PRIMARY,
            CommandBufferLevel::Secondary => vk_sys::VK_COMMAND_BUFFER_LEVEL_SECONDARY,
            CommandBufferLevel::Unknown(level) => level,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum IndexType {
    UInt16,
    UInt32,
    Unknown(vk_sys::VkIndexType),
}

impl From<vk_sys::VkIndexType> for IndexType {
    fn from(index_type: vk_sys::VkIndexType) -> Self {
        match index_type {
            vk_sys::VK_INDEX_TYPE_UINT16 => IndexType::UInt16,
            vk_sys::VK_INDEX_TYPE_UINT32 => IndexType::UInt32,
            _ => IndexType::Unknown(index_type),
        }
    }
}

impl From<IndexType> for vk_sys::VkIndexType {
    fn from(index_type: IndexType) -> Self {
        match index_type {
            IndexType::UInt16 => vk_sys::VK_INDEX_TYPE_UINT16,
            IndexType::UInt32 => vk_sys::VK_INDEX_TYPE_UINT32,
            IndexType::Unknown(index_type) => index_type,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SubpassContents {
    Inline,
    SecondaryCommandBuffers,
    Unknown(vk_sys::VkSubpassContents),
}

impl From<vk_sys::VkSubpassContents> for SubpassContents {
    fn from(contents: vk_sys::VkSubpassContents) -> Self {
        match contents {
            vk_sys::VK_SUBPASS_CONTENTS_INLINE => SubpassContents::Inline,
            vk_sys::VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS => SubpassContents::SecondaryCommandBuffers,
            _ => SubpassContents::Unknown(contents),
        }
    }
}

impl From<SubpassContents> for vk_sys::VkSubpassContents {
    fn from(contents: SubpassContents) -> Self {
        match contents {
            SubpassContents::Inline => vk_sys::VK_SUBPASS_CONTENTS_INLINE,
            SubpassContents::SecondaryCommandBuffers => vk_sys::VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS,
            SubpassContents::Unknown(contents) => contents,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ApplicationInfoChainElement {
}

#[derive(Debug, Clone)]
pub struct ApplicationInfo {
    pub chain: Vec<ApplicationInfoChainElement>,
    pub application_name: Option<String>,
    pub application_version: u32,
    pub engine_name: Option<String>,
    pub engine_version: u32,
    pub api_version: Option<Version>,
}

impl<'a> From<&'a vk_sys::VkApplicationInfo> for ApplicationInfo {
    fn from(info: &'a vk_sys::VkApplicationInfo) -> Self {
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
    application_info: vk_sys::VkApplicationInfo,
    application_name_cstr: Option<CString>,
    engine_name_cstr: Option<CString>,
}

impl Deref for VkApplicationInfoWrapper {
    type Target = vk_sys::VkApplicationInfo;

    fn deref(&self) -> &Self::Target {
        &self.application_info
    }
}

impl AsRef<vk_sys::VkApplicationInfo> for VkApplicationInfoWrapper {
    fn as_ref(&self) -> &vk_sys::VkApplicationInfo {
        &self.application_info
    }
}

impl<'a> From<&'a ApplicationInfo> for VkApplicationInfoWrapper {
    fn from(info: &'a ApplicationInfo) -> Self {
        let application_name_cstr = utils::cstr_from_string(info.application_name.clone());
        let engine_name_cstr = utils::cstr_from_string(info.engine_name.clone());

        VkApplicationInfoWrapper {
            application_info: vk_sys::VkApplicationInfo {
                sType: vk_sys::VK_STRUCTURE_TYPE_APPLICATION_INFO,
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

#[derive(Debug, Clone)]
pub enum InstanceCreateInfoChainElement {
}

#[derive(Debug, Clone)]
pub struct InstanceCreateInfo {
    pub chain: Vec<InstanceCreateInfoChainElement>,
    pub flags: vk_sys::VkInstanceCreateFlags,
    pub application_info: Option<ApplicationInfo>,
    pub enabled_layers: Vec<String>,
    pub enabled_extensions: Vec<InstanceExtension>,
}

impl<'a> From<&'a vk_sys::VkInstanceCreateInfo> for InstanceCreateInfo {
    fn from(create_info: &'a vk_sys::VkInstanceCreateInfo) -> Self {
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
    create_info: vk_sys::VkInstanceCreateInfo,
    application_info: Option<Box<VkApplicationInfoWrapper>>,
    enabled_layers: Vec<CString>,
    enabled_layers_ptrs: Vec<*const c_char>,
    enabled_extensions: Vec<CString>,
    enabled_extensions_ptrs: Vec<*const c_char>,
}

impl Deref for VkInstanceCreateInfoWrapper {
    type Target = vk_sys::VkInstanceCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vk_sys::VkInstanceCreateInfo> for VkInstanceCreateInfoWrapper {
    fn as_ref(&self) -> &vk_sys::VkInstanceCreateInfo {
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
            create_info: vk_sys::VkInstanceCreateInfo {
                sType: vk_sys::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
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

pub trait Allocator: Send {
    fn alloc(&mut self, size: usize, alignment: usize, allocation_scope: SystemAllocationSope) -> *mut c_void;
    fn realloc(&mut self, original: *mut c_void, size: usize, alignment: usize, allocation_scope: SystemAllocationSope) -> *mut c_void;
    fn free(&mut self, memory: *mut c_void);

    fn has_internal_alloc(&self) -> bool {
        false
    }

    #[allow(unused_variables)]
    fn internal_alloc(&mut self, size: usize, allocation_type: InternalAllocationType, allocation_scope: SystemAllocationSope) {
        panic!();
    }

    #[allow(unused_variables)]
    fn internal_free(&mut self, size: usize, allocation_type: InternalAllocationType, allocation_scope: SystemAllocationSope) {
        panic!();
    }
}

#[derive(Debug, Copy, Clone)]
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

impl<'a> From<&'a vk_sys::VkPhysicalDeviceFeatures> for PhysicalDeviceFeatures {
    fn from(featurs: &'a vk_sys::VkPhysicalDeviceFeatures) -> Self {
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

impl<'a> From<&'a PhysicalDeviceFeatures> for vk_sys::VkPhysicalDeviceFeatures {
    fn from(featurs: &'a PhysicalDeviceFeatures) -> Self {
        vk_sys::VkPhysicalDeviceFeatures {
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

#[derive(Debug, Copy, Clone)]
pub struct FormatProperties {
    pub linear_tiling_features: vk_sys::VkFormatFeatureFlags,
    pub optimal_tiling_features: vk_sys::VkFormatFeatureFlags,
    pub buffer_features: vk_sys::VkFormatFeatureFlags,
}

impl<'a> From<&'a vk_sys::VkFormatProperties> for FormatProperties {
    fn from(properties: &'a vk_sys::VkFormatProperties) -> Self {
        FormatProperties {
            linear_tiling_features: properties.linearTilingFeatures,
            optimal_tiling_features: properties.optimalTilingFeatures,
            buffer_features: properties.bufferFeatures,
        }
    }
}

impl<'a> From<&'a FormatProperties> for vk_sys::VkFormatProperties {
    fn from(properties: &'a FormatProperties) -> Self {
        vk_sys::VkFormatProperties {
            linearTilingFeatures: properties.linear_tiling_features,
            optimalTilingFeatures: properties.optimal_tiling_features,
            bufferFeatures: properties.buffer_features,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Extent3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

impl<'a> From<&'a vk_sys::VkExtent3D> for Extent3D {
    fn from(extent: &'a vk_sys::VkExtent3D) -> Self {
        Extent3D {
            width: extent.width,
            height: extent.height,
            depth: extent.depth,
        }
    }
}

impl<'a> From<&'a Extent3D> for vk_sys::VkExtent3D {
    fn from(extent: &'a Extent3D) -> Self {
        vk_sys::VkExtent3D {
            width: extent.width,
            height: extent.height,
            depth: extent.depth,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ImageFormatProperties {
    pub max_extent: Extent3D,
    pub max_mip_levels: u32,
    pub max_array_layers: u32,
    pub sample_counts: vk_sys::VkSampleCountFlags,
    pub max_resource_size: u64,
}

impl<'a> From<&'a vk_sys::VkImageFormatProperties> for ImageFormatProperties {
    fn from(properties: &'a vk_sys::VkImageFormatProperties) -> Self {
        ImageFormatProperties {
            max_extent: (&properties.maxExtent).into(),
            max_mip_levels: properties.maxMipLevels,
            max_array_layers: properties.maxArrayLayers,
            sample_counts: properties.sampleCounts,
            max_resource_size: properties.maxResourceSize,
        }
    }
}

impl<'a> From<&'a ImageFormatProperties> for vk_sys::VkImageFormatProperties {
    fn from(properties: &'a ImageFormatProperties) -> Self {
        vk_sys::VkImageFormatProperties {
            maxExtent: (&properties.max_extent).into(),
            maxMipLevels: properties.max_mip_levels,
            maxArrayLayers: properties.max_array_layers,
            sampleCounts: properties.sample_counts,
            maxResourceSize: properties.max_resource_size,
        }
    }
}

#[derive(Debug, Copy, Clone)]
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
    pub framebuffer_color_sample_counts: vk_sys::VkSampleCountFlags,
    pub framebuffer_depth_sample_counts: vk_sys::VkSampleCountFlags,
    pub framebuffer_stencil_sample_counts: vk_sys::VkSampleCountFlags,
    pub framebuffer_no_attachments_sample_counts: vk_sys::VkSampleCountFlags,
    pub max_color_attachments: u32,
    pub sampled_image_color_sample_counts: vk_sys::VkSampleCountFlags,
    pub sampled_image_integer_sample_counts: vk_sys::VkSampleCountFlags,
    pub sampled_image_depth_sample_counts: vk_sys::VkSampleCountFlags,
    pub sampled_image_stencil_sample_counts: vk_sys::VkSampleCountFlags,
    pub storage_image_sample_counts: vk_sys::VkSampleCountFlags,
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

impl<'a> From<&'a vk_sys::VkPhysicalDeviceLimits> for PhysicalDeviceLimits {
    fn from(limits: &'a vk_sys::VkPhysicalDeviceLimits) -> Self {
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

impl<'a> From<&'a PhysicalDeviceLimits> for vk_sys::VkPhysicalDeviceLimits {
    fn from(limits: &'a PhysicalDeviceLimits) -> Self {
        vk_sys::VkPhysicalDeviceLimits {
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

#[derive(Debug, Copy, Clone)]
pub struct PhysicalDeviceSparseProperties {
    pub residency_standard_2d_block_shape: bool,
    pub residency_standard_2d_multisample_block_shape: bool,
    pub residency_standard_3d_block_shape: bool,
    pub residency_aligned_mip_size: bool,
    pub residency_non_resident_strict: bool,
}

impl<'a> From<&'a vk_sys::VkPhysicalDeviceSparseProperties> for PhysicalDeviceSparseProperties {
    fn from(properties: &'a vk_sys::VkPhysicalDeviceSparseProperties) -> Self {
        PhysicalDeviceSparseProperties {
            residency_standard_2d_block_shape: utils::from_vk_bool(properties.residencyStandard2DBlockShape),
            residency_standard_2d_multisample_block_shape: utils::from_vk_bool(properties.residencyStandard2DMultisampleBlockShape),
            residency_standard_3d_block_shape: utils::from_vk_bool(properties.residencyStandard3DBlockShape),
            residency_aligned_mip_size: utils::from_vk_bool(properties.residencyAlignedMipSize),
            residency_non_resident_strict: utils::from_vk_bool(properties.residencyNonResidentStrict),
        }
    }
}

impl<'a> From<&'a PhysicalDeviceSparseProperties> for vk_sys::VkPhysicalDeviceSparseProperties {
    fn from(properties: &'a PhysicalDeviceSparseProperties) -> Self {
        vk_sys::VkPhysicalDeviceSparseProperties {
            residencyStandard2DBlockShape: utils::to_vk_bool(properties.residency_standard_2d_block_shape),
            residencyStandard2DMultisampleBlockShape: utils::to_vk_bool(properties.residency_standard_2d_multisample_block_shape),
            residencyStandard3DBlockShape: utils::to_vk_bool(properties.residency_standard_3d_block_shape),
            residencyAlignedMipSize: utils::to_vk_bool(properties.residency_aligned_mip_size),
            residencyNonResidentStrict: utils::to_vk_bool(properties.residency_non_resident_strict),
        }
    }
}

#[derive(Debug, Clone)]
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

impl<'a> From<&'a vk_sys::VkPhysicalDeviceProperties> for PhysicalDeviceProperties {
    fn from(properties: &'a vk_sys::VkPhysicalDeviceProperties) -> Self {
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

impl<'a> From<&'a PhysicalDeviceProperties> for vk_sys::VkPhysicalDeviceProperties {
    fn from(properties: &'a PhysicalDeviceProperties) -> Self {
        let mut res = vk_sys::VkPhysicalDeviceProperties {
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

#[derive(Debug, Copy, Clone)]
pub struct QueueFamilyProperties {
    pub queue_flags: vk_sys::VkQueueFlags,
    pub queue_count: u32,
    pub timestamp_valid_bits: u32,
    pub min_image_transfer_granularity: Extent3D,
}

impl<'a> From<&'a vk_sys::VkQueueFamilyProperties> for QueueFamilyProperties {
    fn from(properties: &'a vk_sys::VkQueueFamilyProperties) -> Self {
        QueueFamilyProperties {
            queue_flags: properties.queueFlags,
            queue_count: properties.queueCount,
            timestamp_valid_bits: properties.timestampValidBits,
            min_image_transfer_granularity: (&properties.minImageTransferGranularity).into(),
        }
    }
}

impl<'a> From<&'a QueueFamilyProperties> for vk_sys::VkQueueFamilyProperties {
    fn from(properties: &'a QueueFamilyProperties) -> Self {
        vk_sys::VkQueueFamilyProperties {
            queueFlags: properties.queue_flags,
            queueCount: properties.queue_count,
            timestampValidBits: properties.timestamp_valid_bits,
            minImageTransferGranularity: (&properties.min_image_transfer_granularity).into(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct MemoryType {
    pub property_flags: vk_sys::VkMemoryPropertyFlags,
    pub heap_index: u32,
}

impl<'a> From<&'a vk_sys::VkMemoryType> for MemoryType {
    fn from(memory_type: &'a vk_sys::VkMemoryType) -> Self {
        MemoryType {
            property_flags: memory_type.propertyFlags,
            heap_index: memory_type.heapIndex,
        }
    }
}

impl<'a> From<&'a MemoryType> for vk_sys::VkMemoryType {
    fn from(memory_type: &'a MemoryType) -> Self {
        vk_sys::VkMemoryType {
            propertyFlags: memory_type.property_flags,
            heapIndex: memory_type.heap_index,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct MemoryHeap {
    pub size: u64,
    pub flags: vk_sys::VkMemoryHeapFlags,
}

impl<'a> From<&'a vk_sys::VkMemoryHeap> for MemoryHeap {
    fn from(heap: &'a vk_sys::VkMemoryHeap) -> Self {
        MemoryHeap {
            size: heap.size,
            flags: heap.flags,
        }
    }
}

impl<'a> From<&'a MemoryHeap> for vk_sys::VkMemoryHeap {
    fn from(heap: &'a MemoryHeap) -> Self {
        vk_sys::VkMemoryHeap {
            size: heap.size,
            flags: heap.flags,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PhysicalDeviceMemoryProperties {
    pub memory_types: Vec<MemoryType>,
    pub memory_heaps: Vec<MemoryHeap>,
}

impl<'a> From<&'a vk_sys::VkPhysicalDeviceMemoryProperties> for PhysicalDeviceMemoryProperties {
    fn from(properties: &'a vk_sys::VkPhysicalDeviceMemoryProperties) -> Self {
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

impl<'a> From<&'a PhysicalDeviceMemoryProperties> for vk_sys::VkPhysicalDeviceMemoryProperties {
    fn from(properties: &'a PhysicalDeviceMemoryProperties) -> Self {
        debug_assert!(properties.memory_types.len() <= vk_sys::VK_MAX_MEMORY_TYPES);
        debug_assert!(properties.memory_heaps.len() <= vk_sys::VK_MAX_MEMORY_HEAPS);

        let mut res: vk_sys::VkPhysicalDeviceMemoryProperties = unsafe { mem::uninitialized() };

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

#[derive(Debug, Clone)]
pub enum DeviceQueueCreateInfoChainElement {
}

#[derive(Debug, Clone)]
pub struct DeviceQueueCreateInfo {
    pub chain: Vec<DeviceQueueCreateInfoChainElement>,
    pub flags: vk_sys::VkDeviceQueueCreateFlags,
    pub queue_family_index: u32,
    pub queue_priorities: Vec<f32>,
}

impl<'a> From<&'a vk_sys::VkDeviceQueueCreateInfo> for DeviceQueueCreateInfo {
    fn from(create_info: &'a vk_sys::VkDeviceQueueCreateInfo) -> Self {
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
    create_info: vk_sys::VkDeviceQueueCreateInfo,
    queue_priorities: Vec<f32>,
}

impl Deref for VkDeviceQueueCreateInfoWrapper {
    type Target = vk_sys::VkDeviceQueueCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vk_sys::VkDeviceQueueCreateInfo> for VkDeviceQueueCreateInfoWrapper {
    fn as_ref(&self) -> &vk_sys::VkDeviceQueueCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a DeviceQueueCreateInfo> for VkDeviceQueueCreateInfoWrapper {
    fn from(create_info: &'a DeviceQueueCreateInfo) -> Self {
        let queue_priorities = create_info.queue_priorities.clone();

        VkDeviceQueueCreateInfoWrapper {
            create_info: vk_sys::VkDeviceQueueCreateInfo {
                sType: vk_sys::VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
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

#[derive(Debug, Clone)]
pub enum DeviceCreateInfoChainElement {
}

#[derive(Debug, Clone)]
pub struct DeviceCreateInfo {
    pub chain: Vec<DeviceCreateInfoChainElement>,
    pub flags: vk_sys::VkDeviceCreateFlags,
    pub queue_create_infos: Vec<DeviceQueueCreateInfo>,
    pub enabled_layers: Vec<String>,
    pub enabled_extensions: Vec<DeviceExtension>,
    pub enabled_features: Option<PhysicalDeviceFeatures>,
}

impl<'a> From<&'a vk_sys::VkDeviceCreateInfo> for DeviceCreateInfo {
    fn from(create_info: &'a vk_sys::VkDeviceCreateInfo) -> Self {
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
    create_info: vk_sys::VkDeviceCreateInfo,
    queue_create_infos_wrappers: Vec<VkDeviceQueueCreateInfoWrapper>,
    queue_create_infos: Vec<vk_sys::VkDeviceQueueCreateInfo>,
    enabled_layers: Vec<CString>,
    enabled_layers_ptrs: Vec<*const c_char>,
    enabled_extensions: Vec<CString>,
    enabled_extensions_ptrs: Vec<*const c_char>,
    enabled_features: Option<Box<vk_sys::VkPhysicalDeviceFeatures>>,
}

impl Deref for VkDeviceCreateInfoWrapper {
    type Target = vk_sys::VkDeviceCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vk_sys::VkDeviceCreateInfo> for VkDeviceCreateInfoWrapper {
    fn as_ref(&self) -> &vk_sys::VkDeviceCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a DeviceCreateInfo> for VkDeviceCreateInfoWrapper {
    fn from(create_info: &'a DeviceCreateInfo) -> Self {
        let queue_create_infos_wrappers: Vec<VkDeviceQueueCreateInfoWrapper> = create_info.queue_create_infos
            .iter()
            .map(From::from)
            .collect();

        let queue_create_infos: Vec<vk_sys::VkDeviceQueueCreateInfo> = queue_create_infos_wrappers
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
                let enabled_features: Box<vk_sys::VkPhysicalDeviceFeatures> = Box::new(enabled_features.into());
                enabled_features_ptr = &*enabled_features as *const _;
                Some(enabled_features)
            }

            None => {
                enabled_features_ptr = ptr::null();
                None
            }
        };

        VkDeviceCreateInfoWrapper {
            create_info: vk_sys::VkDeviceCreateInfo {
                sType: vk_sys::VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
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

#[derive(Debug, Clone)]
pub enum InstanceExtension {
    Unknown(String),
}

impl From<String> for InstanceExtension {
    fn from(name: String) -> Self {
        InstanceExtension::Unknown(name)
    }
}

impl<'a> From<&'a str> for InstanceExtension {
    fn from(name: &'a str) -> Self {
        InstanceExtension::Unknown(name.to_owned())
    }
}

impl From<InstanceExtension> for String {
    fn from(extension: InstanceExtension) -> Self {
        match extension {
            InstanceExtension::Unknown(name) => name,
        }
    }
}

impl<'a> From<&'a InstanceExtension> for &'a str {
    fn from(extension: &'a InstanceExtension) -> Self {
        match *extension {
            InstanceExtension::Unknown(ref name) => name,
        }
    }
}

#[derive(Debug, Clone)]
pub struct InstanceExtensionProperties {
    pub extension: InstanceExtension,
    pub spec_version: u32,
}

impl<'a> From<&'a vk_sys::VkExtensionProperties> for InstanceExtensionProperties {
    fn from(properties: &'a vk_sys::VkExtensionProperties) -> Self {
        let name = unsafe { CStr::from_ptr(properties.extensionName.as_ptr()).to_str().unwrap() };

        InstanceExtensionProperties {
            extension: name.into(),
            spec_version: properties.specVersion,
        }
    }
}

impl<'a> From<&'a InstanceExtensionProperties> for vk_sys::VkExtensionProperties {
    fn from(properties: &'a InstanceExtensionProperties) -> Self {
        unsafe {
            let name: &str = (&properties.extension).into();
            debug_assert!(name.len() < vk_sys::VK_MAX_EXTENSION_NAME_SIZE);

            let mut res: vk_sys::VkExtensionProperties = mem::uninitialized();
            ptr::copy_nonoverlapping(name.as_ptr() as *const _, res.extensionName.as_mut_ptr(), name.len());
            res.extensionName[name.len()] = 0;
            res.specVersion = properties.spec_version;

            res
        }
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct DeviceExtensionProperties {
    pub extension: DeviceExtension,
    pub spec_version: u32,
}

impl<'a> From<&'a vk_sys::VkExtensionProperties> for DeviceExtensionProperties {
    fn from(properties: &'a vk_sys::VkExtensionProperties) -> Self {
        let name = unsafe { CStr::from_ptr(properties.extensionName.as_ptr()).to_str().unwrap() };

        DeviceExtensionProperties {
            extension: name.into(),
            spec_version: properties.specVersion,
        }
    }
}

impl<'a> From<&'a DeviceExtensionProperties> for vk_sys::VkExtensionProperties {
    fn from(properties: &'a DeviceExtensionProperties) -> Self {
        unsafe {
            let name: &str = (&properties.extension).into();
            debug_assert!(name.len() < vk_sys::VK_MAX_EXTENSION_NAME_SIZE);

            let mut res: vk_sys::VkExtensionProperties = mem::uninitialized();
            ptr::copy_nonoverlapping(name.as_ptr() as *const _, res.extensionName.as_mut_ptr(), name.len());
            res.extensionName[name.len()] = 0;
            res.specVersion = properties.spec_version;

            res
        }
    }
}

#[derive(Debug, Clone)]
pub struct LayerProperties {
    pub layer_name: String,
    pub spec_version: Version,
    pub implementation_version: u32,
    pub description: String,
}

impl<'a> From<&'a vk_sys::VkLayerProperties> for LayerProperties {
    fn from(layer_properties: &'a vk_sys::VkLayerProperties) -> Self {
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

impl<'a> From<&'a LayerProperties> for vk_sys::VkLayerProperties {
    fn from(properties: &'a LayerProperties) -> Self {
        unsafe {
            debug_assert!(properties.layer_name.len() < vk_sys::VK_MAX_EXTENSION_NAME_SIZE);
            debug_assert!(properties.description.len() < vk_sys::VK_MAX_DESCRIPTION_SIZE);

            let mut res: vk_sys::VkLayerProperties = mem::uninitialized();

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

#[derive(Debug, Copy, Clone)]
pub struct MemoryRequirements {
    pub size: u64,
    pub alignment: u64,
    pub memory_type_bits: u32,
}

impl<'a> From<&'a vk_sys::VkMemoryRequirements> for MemoryRequirements {
    fn from(requirements: &'a vk_sys::VkMemoryRequirements) -> Self {
        MemoryRequirements {
            size: requirements.size,
            alignment: requirements.alignment,
            memory_type_bits: requirements.memoryTypeBits,
        }
    }
}

impl<'a> From<&'a MemoryRequirements> for vk_sys::VkMemoryRequirements {
    fn from(requirements: &'a MemoryRequirements) -> Self {
        vk_sys::VkMemoryRequirements {
            size: requirements.size,
            alignment: requirements.alignment,
            memoryTypeBits: requirements.memory_type_bits,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SparseImageFormatProperties {
    pub aspect_mask: vk_sys::VkImageAspectFlags,
    pub image_granularity: Extent3D,
    pub flags: vk_sys::VkSparseImageFormatFlags,
}

impl<'a> From<&'a vk_sys::VkSparseImageFormatProperties> for SparseImageFormatProperties {
    fn from(properties: &'a vk_sys::VkSparseImageFormatProperties) -> Self {
        SparseImageFormatProperties {
            aspect_mask: properties.aspectMask,
            image_granularity: (&properties.imageGranularity).into(),
            flags: properties.flags,
        }
    }
}

impl<'a> From<&'a SparseImageFormatProperties> for vk_sys::VkSparseImageFormatProperties {
    fn from(properties: &'a SparseImageFormatProperties) -> Self {
        vk_sys::VkSparseImageFormatProperties {
            aspectMask: properties.aspect_mask,
            imageGranularity: (&properties.image_granularity).into(),
            flags: properties.flags,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SparseImageMemoryRequirements {
    pub format_properties: SparseImageFormatProperties,
    pub image_mip_tail_first_lod: u32,
    pub image_mip_tail_size: u64,
    pub image_mip_tail_offset: u64,
    pub image_mip_tail_stride: u64,
}

impl<'a> From<&'a vk_sys::VkSparseImageMemoryRequirements> for SparseImageMemoryRequirements {
    fn from(requirements: &'a vk_sys::VkSparseImageMemoryRequirements) -> Self {
        SparseImageMemoryRequirements {
            format_properties: (&requirements.formatProperties).into(),
            image_mip_tail_first_lod: requirements.imageMipTailFirstLod,
            image_mip_tail_size: requirements.imageMipTailSize,
            image_mip_tail_offset: requirements.imageMipTailOffset,
            image_mip_tail_stride: requirements.imageMipTailStride,
        }
    }
}

impl<'a> From<&'a SparseImageMemoryRequirements> for vk_sys::VkSparseImageMemoryRequirements {
    fn from(requirements: &'a SparseImageMemoryRequirements) -> Self {
        vk_sys::VkSparseImageMemoryRequirements {
            formatProperties: (&requirements.format_properties).into(),
            imageMipTailFirstLod: requirements.image_mip_tail_first_lod,
            imageMipTailSize: requirements.image_mip_tail_size,
            imageMipTailOffset: requirements.image_mip_tail_offset,
            imageMipTailStride: requirements.image_mip_tail_stride,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ImageSubresource {
    pub aspect_mask: vk_sys::VkImageAspectFlags,
    pub mip_level: u32,
    pub array_layer: u32,
}

impl<'a> From<&'a vk_sys::VkImageSubresource> for ImageSubresource {
    fn from(subresource: &'a vk_sys::VkImageSubresource) -> Self {
        ImageSubresource {
            aspect_mask: subresource.aspectMask,
            mip_level: subresource.mipLevel,
            array_layer: subresource.arrayLayer,
        }
    }
}

impl<'a> From<&'a ImageSubresource> for vk_sys::VkImageSubresource {
    fn from(subresource: &'a ImageSubresource) -> Self {
        vk_sys::VkImageSubresource {
            aspectMask: subresource.aspect_mask,
            mipLevel: subresource.mip_level,
            arrayLayer: subresource.array_layer,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Offset3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl<'a> From<&'a vk_sys::VkOffset3D> for Offset3D {
    fn from(offset: &'a vk_sys::VkOffset3D) -> Self {
        Offset3D {
            x: offset.x,
            y: offset.y,
            z: offset.z,
        }
    }
}

impl<'a> From<&'a Offset3D> for vk_sys::VkOffset3D {
    fn from(offset: &'a Offset3D) -> Self {
        vk_sys::VkOffset3D {
            x: offset.x,
            y: offset.y,
            z: offset.z,
        }
    }
}

#[derive(Debug, Clone)]
pub enum FenceCreateInfoChainElement {
}

#[derive(Debug, Clone)]
pub struct FenceCreateInfo {
    pub chain: Vec<FenceCreateInfoChainElement>,
    pub flags: vk_sys::VkFenceCreateFlags,
}

impl<'a> From<&'a vk_sys::VkFenceCreateInfo> for FenceCreateInfo {
    fn from(create_info: &'a vk_sys::VkFenceCreateInfo) -> Self {
        debug_assert_eq!(create_info.pNext, ptr::null());

        FenceCreateInfo {
            chain: vec![],
            flags: create_info.flags,
        }
    }
}

#[derive(Debug)]
struct VkFenceCreateInfoWrapper {
    create_info: vk_sys::VkFenceCreateInfo,
}

impl Deref for VkFenceCreateInfoWrapper {
    type Target = vk_sys::VkFenceCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vk_sys::VkFenceCreateInfo> for VkFenceCreateInfoWrapper {
    fn as_ref(&self) -> &vk_sys::VkFenceCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a FenceCreateInfo> for VkFenceCreateInfoWrapper {
    fn from(create_info: &'a FenceCreateInfo) -> VkFenceCreateInfoWrapper {
        VkFenceCreateInfoWrapper {
            create_info: vk_sys::VkFenceCreateInfo {
                sType: vk_sys::VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum SemaphoreCreateInfoChainElement {
}

#[derive(Debug, Clone)]
pub struct SemaphoreCreateInfo {
    pub chain: Vec<SemaphoreCreateInfoChainElement>,
    pub flags: vk_sys::VkSemaphoreCreateFlags,
}

impl<'a> From<&'a vk_sys::VkSemaphoreCreateInfo> for SemaphoreCreateInfo {
    fn from(create_info: &'a vk_sys::VkSemaphoreCreateInfo) -> Self {
        debug_assert_eq!(create_info.pNext, ptr::null());

        SemaphoreCreateInfo {
            chain: vec![],
            flags: create_info.flags,
        }
    }
}

#[derive(Debug)]
struct VkSemaphoreCreateInfoWrapper {
    create_info: vk_sys::VkSemaphoreCreateInfo,
}

impl Deref for VkSemaphoreCreateInfoWrapper {
    type Target = vk_sys::VkSemaphoreCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vk_sys::VkSemaphoreCreateInfo> for VkSemaphoreCreateInfoWrapper {
    fn as_ref(&self) -> &vk_sys::VkSemaphoreCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a SemaphoreCreateInfo> for VkSemaphoreCreateInfoWrapper {
    fn from(create_info: &'a SemaphoreCreateInfo) -> VkSemaphoreCreateInfoWrapper {
        VkSemaphoreCreateInfoWrapper {
            create_info: vk_sys::VkSemaphoreCreateInfo {
                sType: vk_sys::VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum EventCreateInfoChainElement {
}

#[derive(Debug, Clone)]
pub struct EventCreateInfo {
    pub chain: Vec<EventCreateInfoChainElement>,
    pub flags: vk_sys::VkEventCreateFlags,
}

impl<'a> From<&'a vk_sys::VkEventCreateInfo> for EventCreateInfo {
    fn from(create_info: &'a vk_sys::VkEventCreateInfo) -> Self {
        debug_assert_eq!(create_info.pNext, ptr::null());

        EventCreateInfo {
            chain: vec![],
            flags: create_info.flags,
        }
    }
}

#[derive(Debug)]
struct VkEventCreateInfoWrapper {
    create_info: vk_sys::VkEventCreateInfo,
}

impl Deref for VkEventCreateInfoWrapper {
    type Target = vk_sys::VkEventCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vk_sys::VkEventCreateInfo> for VkEventCreateInfoWrapper {
    fn as_ref(&self) -> &vk_sys::VkEventCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a EventCreateInfo> for VkEventCreateInfoWrapper {
    fn from(create_info: &'a EventCreateInfo) -> VkEventCreateInfoWrapper {
        VkEventCreateInfoWrapper {
            create_info: vk_sys::VkEventCreateInfo {
                sType: vk_sys::VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum QueryPoolCreateInfoChainElement {
}

#[derive(Debug, Clone)]
pub struct QueryPoolCreateInfo {
    pub chain: Vec<QueryPoolCreateInfoChainElement>,
    pub flags: vk_sys::VkQueryPoolCreateFlags,
    pub query_type: QueryType,
    pub query_count: u32,
    pub pipeline_statistics: vk_sys::VkQueryPipelineStatisticFlags,
}

impl<'a> From<&'a vk_sys::VkQueryPoolCreateInfo> for QueryPoolCreateInfo {
    fn from(create_info: &'a vk_sys::VkQueryPoolCreateInfo) -> Self {
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
    create_info: vk_sys::VkQueryPoolCreateInfo,
}

impl Deref for VkQueryPoolCreateInfoWrapper {
    type Target = vk_sys::VkQueryPoolCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vk_sys::VkQueryPoolCreateInfo> for VkQueryPoolCreateInfoWrapper {
    fn as_ref(&self) -> &vk_sys::VkQueryPoolCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a QueryPoolCreateInfo> for VkQueryPoolCreateInfoWrapper {
    fn from(create_info: &'a QueryPoolCreateInfo) -> VkQueryPoolCreateInfoWrapper {
        VkQueryPoolCreateInfoWrapper {
            create_info: vk_sys::VkQueryPoolCreateInfo {
                sType: vk_sys::VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
                queryType: create_info.query_type.into(),
                queryCount: create_info.query_count,
                pipelineStatistics: create_info.pipeline_statistics,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum BufferCreateInfoChainElement {
}

#[derive(Debug, Clone)]
pub struct BufferCreateInfo {
    pub chain: Vec<BufferCreateInfoChainElement>,
    pub flags: vk_sys::VkBufferCreateFlags,
    pub size: u64,
    pub usage: vk_sys::VkBufferUsageFlags,
    pub sharing_mode: SharingMode,
    pub queue_family_indices: Option<Vec<u32>>,
}

impl<'a> From<&'a vk_sys::VkBufferCreateInfo> for BufferCreateInfo {
    fn from(create_info: &'a vk_sys::VkBufferCreateInfo) -> Self {
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
    create_info: vk_sys::VkBufferCreateInfo,
    queue_family_indices: Option<Vec<u32>>,
}

impl Deref for VkBufferCreateInfoWrapper {
    type Target = vk_sys::VkBufferCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vk_sys::VkBufferCreateInfo> for VkBufferCreateInfoWrapper {
    fn as_ref(&self) -> &vk_sys::VkBufferCreateInfo {
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
            create_info: vk_sys::VkBufferCreateInfo {
                sType: vk_sys::VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
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

#[derive(Debug, Clone)]
pub enum BufferViewCreateInfoChainElement {
}

#[derive(Debug, Clone)]
pub struct BufferViewCreateInfo {
    pub chain: Vec<BufferViewCreateInfoChainElement>,
    pub flags: vk_sys::VkBufferViewCreateFlags,
    pub buffer: Buffer,
    pub format: Format,
    pub offset: u64,
    pub range: u64,
}

#[derive(Debug)]
struct VkBufferViewCreateInfoWrapper {
    create_info: vk_sys::VkBufferViewCreateInfo,
    buffer: Buffer,
}

impl Deref for VkBufferViewCreateInfoWrapper {
    type Target = vk_sys::VkBufferViewCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vk_sys::VkBufferViewCreateInfo> for VkBufferViewCreateInfoWrapper {
    fn as_ref(&self) -> &vk_sys::VkBufferViewCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a BufferViewCreateInfo> for VkBufferViewCreateInfoWrapper {
    fn from(create_info: &'a BufferViewCreateInfo) -> Self {
        VkBufferViewCreateInfoWrapper {
            create_info: vk_sys::VkBufferViewCreateInfo {
                sType: vk_sys::VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
                buffer: create_info.buffer.handle(),
                format: create_info.format.into(),
                offset: create_info.offset,
                range: create_info.range,
            },
            buffer: create_info.buffer.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ImageCreateInfoChainElement {
}

#[derive(Debug, Clone)]
pub struct ImageCreateInfo {
    pub chain: Vec<ImageCreateInfoChainElement>,
    pub flags: vk_sys::VkImageCreateFlags,
    pub image_type: ImageType,
    pub format: Format,
    pub extent: Extent3D,
    pub mip_levels: u32,
    pub array_layers: u32,
    pub samples: vk_sys::VkSampleCountFlagBits,
    pub tiling: ImageTiling,
    pub usage: vk_sys::VkImageUsageFlags,
    pub sharing_mode: SharingMode,
    pub queue_family_indices: Option<Vec<u32>>,
    pub initial_layout: ImageLayout,
}

impl<'a> From<&'a vk_sys::VkImageCreateInfo> for ImageCreateInfo {
    fn from(create_info: &'a vk_sys::VkImageCreateInfo) -> Self {
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

struct VkImageCreateInfoWrapper {
    create_info: vk_sys::VkImageCreateInfo,

    // Rust complains about this field, even though it doesn't do it for all the other unused
    // fields in other Wrapper structs. No idea why.
    #[allow(dead_code)]
    queue_family_indices: Option<Vec<u32>>,
}

impl Deref for VkImageCreateInfoWrapper {
    type Target = vk_sys::VkImageCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vk_sys::VkImageCreateInfo> for VkImageCreateInfoWrapper {
    fn as_ref(&self) -> &vk_sys::VkImageCreateInfo {
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
            create_info: vk_sys::VkImageCreateInfo {
                sType: vk_sys::VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO,
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

#[derive(Debug, Copy, Clone)]
pub struct SubresourceLayout {
    pub offset: u64,
    pub size: u64,
    pub row_pitch: u64,
    pub array_pitch: u64,
    pub depth_pitch: u64,
}

impl<'a> From<&'a vk_sys::VkSubresourceLayout> for SubresourceLayout {
    fn from(layout: &'a vk_sys::VkSubresourceLayout) -> Self {
        SubresourceLayout {
            offset: layout.offset,
            size: layout.size,
            row_pitch: layout.rowPitch,
            array_pitch: layout.arrayPitch,
            depth_pitch: layout.depthPitch,
        }
    }
}

impl<'a> From<&'a SubresourceLayout> for vk_sys::VkSubresourceLayout {
    fn from(layout: &'a SubresourceLayout) -> Self {
        vk_sys::VkSubresourceLayout {
            offset: layout.offset,
            size: layout.size,
            rowPitch: layout.row_pitch,
            arrayPitch: layout.array_pitch,
            depthPitch: layout.depth_pitch,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ComponentMapping {
    pub r: ComponentSwizzle,
    pub g: ComponentSwizzle,
    pub b: ComponentSwizzle,
    pub a: ComponentSwizzle,
}

impl<'a> From<&'a vk_sys::VkComponentMapping> for ComponentMapping {
    fn from(mapping: &'a vk_sys::VkComponentMapping) -> Self {
        ComponentMapping {
            r: mapping.r.into(),
            g: mapping.g.into(),
            b: mapping.b.into(),
            a: mapping.a.into(),
        }
    }
}

impl<'a> From<&'a ComponentMapping> for vk_sys::VkComponentMapping {
    fn from(mapping: &'a ComponentMapping) -> Self {
        vk_sys::VkComponentMapping {
            r: mapping.r.into(),
            g: mapping.g.into(),
            b: mapping.b.into(),
            a: mapping.a.into(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ImageSubresourceRange {
    pub aspect_mask: vk_sys::VkImageAspectFlags,
    pub base_mip_level: u32,
    pub level_count: u32,
    pub base_array_layer: u32,
    pub layer_count: u32,
}

impl<'a> From<&'a vk_sys::VkImageSubresourceRange> for ImageSubresourceRange {
    fn from(range: &'a vk_sys::VkImageSubresourceRange) -> Self {
        ImageSubresourceRange {
            aspect_mask: range.aspectMask,
            base_mip_level: range.baseMipLevel,
            level_count: range.levelCount,
            base_array_layer: range.baseArrayLayer,
            layer_count: range.layerCount,
        }
    }
}

impl<'a> From<&'a ImageSubresourceRange> for vk_sys::VkImageSubresourceRange {
    fn from(range: &'a ImageSubresourceRange) -> Self {
        vk_sys::VkImageSubresourceRange {
            aspectMask: range.aspect_mask,
            baseMipLevel: range.base_mip_level,
            levelCount: range.level_count,
            baseArrayLayer: range.base_array_layer,
            layerCount: range.layer_count,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ImageViewCreateInfoChainElement {
}

#[derive(Debug, Clone)]
pub struct ImageViewCreateInfo {
    pub chain: Vec<ImageViewCreateInfoChainElement>,
    pub flags: vk_sys::VkImageViewCreateFlags,
    pub image: Image,
    pub view_type: ImageViewType,
    pub format: Format,
    pub components: ComponentMapping,
    pub subresource_range: ImageSubresourceRange,
}

#[derive(Debug)]
struct VkImageViewCreateInfoWrapper {
    create_info: vk_sys::VkImageViewCreateInfo,
    image: Image,
}

impl Deref for VkImageViewCreateInfoWrapper {
    type Target = vk_sys::VkImageViewCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vk_sys::VkImageViewCreateInfo> for VkImageViewCreateInfoWrapper {
    fn as_ref(&self) -> &vk_sys::VkImageViewCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a ImageViewCreateInfo> for VkImageViewCreateInfoWrapper {
    fn from(create_info: &'a ImageViewCreateInfo) -> Self {
        VkImageViewCreateInfoWrapper {
            create_info: vk_sys::VkImageViewCreateInfo {
                sType: vk_sys::VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
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

#[derive(Debug, Clone)]
pub enum ShaderModuleCreateInfoChainElement {
}

#[derive(Debug, Clone)]
pub struct ShaderModuleCreateInfo {
    pub chain: Vec<ShaderModuleCreateInfoChainElement>,
    pub flags: vk_sys::VkShaderModuleCreateFlags,
    pub code_size: usize,
    pub code: Vec<u32>,
}

impl<'a> From<&'a vk_sys::VkShaderModuleCreateInfo> for ShaderModuleCreateInfo {
    fn from(create_info: &'a vk_sys::VkShaderModuleCreateInfo) -> Self {
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
    create_info: vk_sys::VkShaderModuleCreateInfo,
    code: Vec<u32>,
}

impl Deref for VkShaderModuleCreateInfoWrapper {
    type Target = vk_sys::VkShaderModuleCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vk_sys::VkShaderModuleCreateInfo> for VkShaderModuleCreateInfoWrapper {
    fn as_ref(&self) -> &vk_sys::VkShaderModuleCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a ShaderModuleCreateInfo> for VkShaderModuleCreateInfoWrapper {
    fn from(create_info: &'a ShaderModuleCreateInfo) -> Self {
        let code = create_info.code.clone();

        VkShaderModuleCreateInfoWrapper {
            create_info: vk_sys::VkShaderModuleCreateInfo {
                sType: vk_sys::VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
                codeSize: create_info.code_size,
                pCode: code.as_ptr(),
            },
            code: code,
        }
    }
}

#[derive(Debug, Clone)]
pub enum PipelineCacheCreateInfoChainElement {
}

#[derive(Debug, Clone)]
pub struct PipelineCacheCreateInfo {
    pub chain: Vec<PipelineCacheCreateInfoChainElement>,
    pub flags: vk_sys::VkPipelineCacheCreateFlags,
    pub initial_data: Option<Vec<u8>>,
}

impl<'a> From<&'a vk_sys::VkPipelineCacheCreateInfo> for PipelineCacheCreateInfo {
    fn from(create_info: &'a vk_sys::VkPipelineCacheCreateInfo) -> Self {
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
    create_info: vk_sys::VkPipelineCacheCreateInfo,
    initial_data: Option<Vec<u8>>,
}

impl Deref for VkPipelineCacheCreateInfoWrapper {
    type Target = vk_sys::VkPipelineCacheCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vk_sys::VkPipelineCacheCreateInfo> for VkPipelineCacheCreateInfoWrapper {
    fn as_ref(&self) -> &vk_sys::VkPipelineCacheCreateInfo {
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
            create_info: vk_sys::VkPipelineCacheCreateInfo {
                sType: vk_sys::VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
                initialDataSize: initial_data_size,
                pInitialData: initial_data_ptr,
            },
            initial_data: initial_data,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SpecializationMapEntry {
    pub constant_id: u32,
    pub offset: u32,
    pub size: usize,
}

impl<'a> From<&'a vk_sys::VkSpecializationMapEntry> for SpecializationMapEntry {
    fn from(entry: &'a vk_sys::VkSpecializationMapEntry) -> Self {
        SpecializationMapEntry {
            constant_id: entry.constantID,
            offset: entry.offset,
            size: entry.size,
        }
    }
}

impl<'a> From<&'a SpecializationMapEntry> for vk_sys::VkSpecializationMapEntry {
    fn from(entry: &'a SpecializationMapEntry) -> Self {
        vk_sys::VkSpecializationMapEntry {
            constantID: entry.constant_id,
            offset: entry.offset,
            size: entry.size,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct VertexInputBindingDescription {
    pub binding: u32,
    pub stride: u32,
    pub input_rate: VertexInputRate,
}

impl<'a> From<&'a vk_sys::VkVertexInputBindingDescription> for VertexInputBindingDescription {
    fn from(description: &'a vk_sys::VkVertexInputBindingDescription) -> Self {
        VertexInputBindingDescription {
            binding: description.binding,
            stride: description.stride,
            input_rate: description.inputRate.into(),
        }
    }
}

impl<'a> From<&'a VertexInputBindingDescription> for vk_sys::VkVertexInputBindingDescription {
    fn from(description: &'a VertexInputBindingDescription) -> Self {
        vk_sys::VkVertexInputBindingDescription {
            binding: description.binding,
            stride: description.stride,
            inputRate: description.input_rate.into(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct VertexInputAttributeDescription {
    pub location: u32,
    pub binding: u32,
    pub format: Format,
    pub offset: u32,
}

impl<'a> From<&'a vk_sys::VkVertexInputAttributeDescription> for VertexInputAttributeDescription {
    fn from(description: &'a vk_sys::VkVertexInputAttributeDescription) -> Self {
        VertexInputAttributeDescription {
            location: description.location,
            binding: description.binding,
            format: description.format.into(),
            offset: description.offset,
        }
    }
}

impl<'a> From<&'a VertexInputAttributeDescription> for vk_sys::VkVertexInputAttributeDescription {
    fn from(description: &'a VertexInputAttributeDescription) -> Self {
        vk_sys::VkVertexInputAttributeDescription {
            location: description.location,
            binding: description.binding,
            format: description.format.into(),
            offset: description.offset,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub min_depth: f32,
    pub max_depth: f32,
}

impl<'a> From<&'a vk_sys::VkViewport> for Viewport {
    fn from(viewport: &'a vk_sys::VkViewport) -> Self {
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

impl<'a> From<&'a Viewport> for vk_sys::VkViewport {
    fn from(viewport: &'a Viewport) -> Self {
        vk_sys::VkViewport {
            x: viewport.x,
            y: viewport.y,
            width: viewport.width,
            height: viewport.height,
            minDepth: viewport.min_depth,
            maxDepth: viewport.max_depth,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Offset2D {
    pub x: i32,
    pub y: i32,
}

impl<'a> From<&'a vk_sys::VkOffset2D> for Offset2D {
    fn from(offset: &'a vk_sys::VkOffset2D) -> Self {
        Offset2D {
            x: offset.x,
            y: offset.y,
        }
    }
}

impl<'a> From<&'a Offset2D> for vk_sys::VkOffset2D {
    fn from(offset: &'a Offset2D) -> Self {
        vk_sys::VkOffset2D {
            x: offset.x,
            y: offset.y,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Extent2D {
    pub width: u32,
    pub height: u32,
}

impl<'a> From<&'a vk_sys::VkExtent2D> for Extent2D {
    fn from(extent: &'a vk_sys::VkExtent2D) -> Self {
        Extent2D {
            width: extent.width,
            height: extent.height,
        }
    }
}

impl<'a> From<&'a Extent2D> for vk_sys::VkExtent2D {
    fn from(extent: &'a Extent2D) -> Self {
        vk_sys::VkExtent2D {
            width: extent.width,
            height: extent.height,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Rect2D {
    pub offset: Offset2D,
    pub extent: Extent2D,
}

impl<'a> From<&'a vk_sys::VkRect2D> for Rect2D {
    fn from(rect: &'a vk_sys::VkRect2D) -> Self {
        Rect2D {
            offset: (&rect.offset).into(),
            extent: (&rect.extent).into(),
        }
    }
}

impl<'a> From<&'a Rect2D> for vk_sys::VkRect2D {
    fn from(rect: &'a Rect2D) -> Self {
        vk_sys::VkRect2D {
            offset: (&rect.offset).into(),
            extent: (&rect.extent).into(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct StencilOpState {
    pub fail_op: StencilOp,
    pub pass_op: StencilOp,
    pub depth_fail_op: StencilOp,
    pub compare_op: CompareOp,
    pub compare_mask: u32,
    pub write_mask: u32,
    pub reference: u32,
}

impl<'a> From<&'a vk_sys::VkStencilOpState> for StencilOpState {
    fn from(state: &'a vk_sys::VkStencilOpState) -> Self {
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

impl<'a> From<&'a StencilOpState> for vk_sys::VkStencilOpState {
    fn from(state: &'a StencilOpState) -> Self {
        vk_sys::VkStencilOpState {
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

#[derive(Debug, Copy, Clone)]
pub struct PipelineColorBlendAttachmentState {
    pub blend_enable: bool,
    pub src_color_blend_factor: BlendFactor,
    pub dst_color_blend_factor: BlendFactor,
    pub color_blend_op: BlendOp,
    pub src_alpha_blend_factor: BlendFactor,
    pub dst_alpha_blend_factor: BlendFactor,
    pub alpha_blend_op: BlendOp,
    pub color_write_mask: vk_sys::VkColorComponentFlags,
}

impl<'a> From<&'a vk_sys::VkPipelineColorBlendAttachmentState> for PipelineColorBlendAttachmentState {
    fn from(state: &'a vk_sys::VkPipelineColorBlendAttachmentState) -> Self {
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

impl<'a> From<&'a PipelineColorBlendAttachmentState> for vk_sys::VkPipelineColorBlendAttachmentState {
    fn from(state: &'a PipelineColorBlendAttachmentState) -> Self {
        vk_sys::VkPipelineColorBlendAttachmentState {
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

#[derive(Debug, Copy, Clone)]
pub struct PushConstantRange {
    pub stage_flags: vk_sys::VkShaderStageFlags,
    pub offset: u32,
    pub size: u32,
}

impl<'a> From<&'a vk_sys::VkPushConstantRange> for PushConstantRange {
    fn from(range: &'a vk_sys::VkPushConstantRange) -> Self {
        PushConstantRange {
            stage_flags: range.stageFlags,
            offset: range.offset,
            size: range.size,
        }
    }
}

impl<'a> From<&'a PushConstantRange> for vk_sys::VkPushConstantRange {
    fn from(range: &'a PushConstantRange) -> Self {
        vk_sys::VkPushConstantRange {
            stageFlags: range.stage_flags,
            offset: range.offset,
            size: range.size,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SamplerCreateInfoChainElement {
}

#[derive(Debug, Clone)]
pub struct SamplerCreateInfo {
    pub chain: Vec<SamplerCreateInfoChainElement>,
    pub flags: vk_sys::VkSamplerCreateFlags,
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

impl<'a> From<&'a vk_sys::VkSamplerCreateInfo> for SamplerCreateInfo {
    fn from(create_info: &'a vk_sys::VkSamplerCreateInfo) -> Self {
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
    create_info: vk_sys::VkSamplerCreateInfo,
}

impl Deref for VkSamplerCreateInfoWrapper {
    type Target = vk_sys::VkSamplerCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vk_sys::VkSamplerCreateInfo> for VkSamplerCreateInfoWrapper {
    fn as_ref(&self) -> &vk_sys::VkSamplerCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a SamplerCreateInfo> for VkSamplerCreateInfoWrapper {
    fn from(create_info: &'a SamplerCreateInfo) -> Self {
        VkSamplerCreateInfoWrapper {
            create_info: vk_sys::VkSamplerCreateInfo {
                sType: vk_sys::VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO,
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

#[derive(Debug, Clone)]
pub struct DescriptorSetLayoutBinding {
    pub binding: u32,
    pub descriptor_type: DescriptorType,
    pub descriptor_count: u32,
    pub stage_flags: vk_sys::VkShaderStageFlags,
    pub immutable_samplers: Option<Vec<Sampler>>,
}

#[derive(Debug)]
struct VkDescriptorSetLayoutBindingWrapper {
    binding: vk_sys::VkDescriptorSetLayoutBinding,
    immutable_samplers: Option<Vec<Sampler>>,
    immutable_vk_samplers: Option<Vec<vk_sys::VkSampler>>,
}

impl Deref for VkDescriptorSetLayoutBindingWrapper {
    type Target = vk_sys::VkDescriptorSetLayoutBinding;

    fn deref(&self) -> &Self::Target {
        &self.binding
    }
}

impl AsRef<vk_sys::VkDescriptorSetLayoutBinding> for VkDescriptorSetLayoutBindingWrapper {
    fn as_ref(&self) -> &vk_sys::VkDescriptorSetLayoutBinding {
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
            binding: vk_sys::VkDescriptorSetLayoutBinding {
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

#[derive(Debug, Clone)]
pub enum DescriptorSetLayoutCreateInfoChainElement {
}

#[derive(Debug, Clone)]
pub struct DescriptorSetLayoutCreateInfo {
    pub chain: Vec<DescriptorSetLayoutCreateInfoChainElement>,
    pub flags: vk_sys::VkDescriptorSetLayoutCreateFlags,
    pub bindings: Option<Vec<DescriptorSetLayoutBinding>>,
}

#[derive(Debug)]
struct VkDescriptorSetLayoutCreateInfoWrapper {
    create_info: vk_sys::VkDescriptorSetLayoutCreateInfo,
    bindings: Option<Vec<VkDescriptorSetLayoutBindingWrapper>>,
    vk_bindings: Option<Vec<vk_sys::VkDescriptorSetLayoutBinding>>,
}

impl Deref for VkDescriptorSetLayoutCreateInfoWrapper {
    type Target = vk_sys::VkDescriptorSetLayoutCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vk_sys::VkDescriptorSetLayoutCreateInfo> for VkDescriptorSetLayoutCreateInfoWrapper {
    fn as_ref(&self) -> &vk_sys::VkDescriptorSetLayoutCreateInfo {
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
            create_info: vk_sys::VkDescriptorSetLayoutCreateInfo {
                sType: vk_sys::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
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

#[derive(Debug, Copy, Clone)]
pub struct DescriptorPoolSize {
    pub descriptor_type: DescriptorType,
    pub descriptor_count: u32,
}

impl<'a> From<&'a vk_sys::VkDescriptorPoolSize> for DescriptorPoolSize {
    fn from(size: &'a vk_sys::VkDescriptorPoolSize) -> Self {
        DescriptorPoolSize {
            descriptor_type: size.type_.into(),
            descriptor_count: size.descriptorCount,
        }
    }
}

impl<'a> From<&'a DescriptorPoolSize> for vk_sys::VkDescriptorPoolSize {
    fn from(size: &'a DescriptorPoolSize) -> Self {
        vk_sys::VkDescriptorPoolSize {
            type_: size.descriptor_type.into(),
            descriptorCount: size.descriptor_count,
        }
    }
}

#[derive(Debug, Clone)]
pub enum DescriptorPoolCreateInfoChainElement {
}

#[derive(Debug, Clone)]
pub struct DescriptorPoolCreateInfo {
    pub chain: Vec<DescriptorPoolCreateInfoChainElement>,
    pub flags: vk_sys::VkDescriptorPoolCreateFlags,
    pub max_sets: u32,
    pub pool_sizes: Vec<DescriptorPoolSize>,
}

impl<'a> From<&'a vk_sys::VkDescriptorPoolCreateInfo> for DescriptorPoolCreateInfo {
    fn from(create_info: &'a vk_sys::VkDescriptorPoolCreateInfo) -> Self {
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
    create_info: vk_sys::VkDescriptorPoolCreateInfo,
    pool_sizes: Vec<vk_sys::VkDescriptorPoolSize>,
}

impl Deref for VkDescriptorPoolCreateInfoWrapper {
    type Target = vk_sys::VkDescriptorPoolCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vk_sys::VkDescriptorPoolCreateInfo> for VkDescriptorPoolCreateInfoWrapper {
    fn as_ref(&self) -> &vk_sys::VkDescriptorPoolCreateInfo {
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
            create_info: vk_sys::VkDescriptorPoolCreateInfo {
                sType: vk_sys::VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
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

#[derive(Debug, Copy, Clone)]
pub struct AttachmentDescription {
    pub flags: vk_sys::VkAttachmentDescriptionFlags,
    pub format: Format,
    pub samples: vk_sys::VkSampleCountFlagBits,
    pub load_op: AttachmentLoadOp,
    pub store_op: AttachmentStoreOp,
    pub stencil_load_op: AttachmentLoadOp,
    pub stencil_store_op: AttachmentStoreOp,
    pub initial_layout: ImageLayout,
    pub final_layout: ImageLayout,
}

impl<'a> From<&'a vk_sys::VkAttachmentDescription> for AttachmentDescription {
    fn from(description: &'a vk_sys::VkAttachmentDescription) -> Self {
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

impl<'a> From<&'a AttachmentDescription> for vk_sys::VkAttachmentDescription {
    fn from(description: &'a AttachmentDescription) -> Self {
        vk_sys::VkAttachmentDescription {
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

#[derive(Debug, Copy, Clone)]
pub struct AttachmentReference {
    pub attachment: u32,
    pub layout: ImageLayout,
}

impl<'a> From<&'a vk_sys::VkAttachmentReference> for AttachmentReference {
    fn from(reference: &'a vk_sys::VkAttachmentReference) -> Self {
        AttachmentReference {
            attachment: reference.attachment,
            layout: reference.layout.into(),
        }
    }
}

impl<'a> From<&'a AttachmentReference> for vk_sys::VkAttachmentReference {
    fn from(reference: &'a AttachmentReference) -> Self {
        vk_sys::VkAttachmentReference {
            attachment: reference.attachment,
            layout: reference.layout.into(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SubpassDependency {
    pub src_subpass: u32,
    pub dst_subpass: u32,
    pub src_stage_mask: vk_sys::VkPipelineStageFlags,
    pub dst_stage_mask: vk_sys::VkPipelineStageFlags,
    pub src_access_mask: vk_sys::VkAccessFlags,
    pub dst_access_mask: vk_sys::VkAccessFlags,
    pub dependency_flags: vk_sys::VkDependencyFlags,
}

impl<'a> From<&'a vk_sys::VkSubpassDependency> for SubpassDependency {
    fn from(dependency: &'a vk_sys::VkSubpassDependency) -> Self {
        SubpassDependency {
            src_subpass: dependency.srcSubpass,
            dst_subpass: dependency.dstSubpass,
            src_stage_mask: dependency.srcStageMask,
            dst_stage_mask: dependency.dstStageMask,
            src_access_mask: dependency.srcAccessMask,
            dst_access_mask: dependency.dstAccessMask,
            dependency_flags: dependency.dependencyFlags,
        }
    }
}

impl<'a> From<&'a SubpassDependency> for vk_sys::VkSubpassDependency {
    fn from(dependency: &'a SubpassDependency) -> Self {
        vk_sys::VkSubpassDependency {
            srcSubpass: dependency.src_subpass,
            dstSubpass: dependency.dst_subpass,
            srcStageMask: dependency.src_stage_mask,
            dstStageMask: dependency.dst_stage_mask,
            srcAccessMask: dependency.src_access_mask,
            dstAccessMask: dependency.dst_access_mask,
            dependencyFlags: dependency.dependency_flags,
        }
    }
}

#[derive(Debug, Clone)]
pub enum CommandPoolCreateInfoChainElement {
}

#[derive(Debug, Clone)]
pub struct CommandPoolCreateInfo {
    pub chain: Vec<CommandPoolCreateInfoChainElement>,
    pub flags: vk_sys::VkCommandPoolCreateFlags,
    pub queue_family_index: u32,
}

impl<'a> From<&'a vk_sys::VkCommandPoolCreateInfo> for CommandPoolCreateInfo {
    fn from(create_info: &'a vk_sys::VkCommandPoolCreateInfo) -> Self {
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
    create_info: vk_sys::VkCommandPoolCreateInfo,
}

impl Deref for VkCommandPoolCreateInfoWrapper {
    type Target = vk_sys::VkCommandPoolCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.create_info
    }
}

impl AsRef<vk_sys::VkCommandPoolCreateInfo> for VkCommandPoolCreateInfoWrapper {
    fn as_ref(&self) -> &vk_sys::VkCommandPoolCreateInfo {
        &self.create_info
    }
}

impl<'a> From<&'a CommandPoolCreateInfo> for VkCommandPoolCreateInfoWrapper {
    fn from(create_info: &'a CommandPoolCreateInfo) -> Self {
        VkCommandPoolCreateInfoWrapper {
            create_info: vk_sys::VkCommandPoolCreateInfo {
                sType: vk_sys::VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
                pNext: ptr::null(),
                flags: create_info.flags,
                queueFamilyIndex: create_info.queue_family_index,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum CommandBufferAllocateInfoChainElement {
}

#[derive(Debug, Clone)]
pub struct CommandBufferAllocateInfo {
    pub chain: Vec<CommandBufferAllocateInfoChainElement>,
    pub level: CommandBufferLevel,
    pub command_buffer_count: u32,
}

#[derive(Debug)]
struct VkCommandBufferAllocateInfoWrapper {
    info: vk_sys::VkCommandBufferAllocateInfo,
    command_pool: Option<CommandPool>,
}

impl Deref for VkCommandBufferAllocateInfoWrapper {
    type Target = vk_sys::VkCommandBufferAllocateInfo;

    fn deref(&self) -> &Self::Target {
        &self.info
    }
}

impl AsRef<vk_sys::VkCommandBufferAllocateInfo> for VkCommandBufferAllocateInfoWrapper {
    fn as_ref(&self) -> &vk_sys::VkCommandBufferAllocateInfo {
        &self.info
    }
}

impl<'a> From<&'a CommandBufferAllocateInfo> for VkCommandBufferAllocateInfoWrapper {
    fn from(info: &'a CommandBufferAllocateInfo) -> Self {
        VkCommandBufferAllocateInfoWrapper {
            info: vk_sys::VkCommandBufferAllocateInfo {
                sType: vk_sys::VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
                pNext: ptr::null(),
                commandPool: ptr::null_mut(),
                level: info.level.into(),
                commandBufferCount: info.command_buffer_count,
            },
            command_pool: None,
        }
    }
}

impl VkCommandBufferAllocateInfoWrapper {
    fn set_command_pool(&mut self, command_pool: Option<CommandPool>) {
        match command_pool {
            Some(command_pool) => {
                self.info.commandPool = command_pool.handle();
                self.command_pool = Some(command_pool);
            }

            None => {
                self.info.commandPool = ptr::null_mut();
                self.command_pool = None;
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BufferCopy {
    pub src_offset: u64,
    pub dst_offset: u64,
    pub size: u64,
}

impl<'a> From<&'a vk_sys::VkBufferCopy> for BufferCopy {
    fn from(copy: &'a vk_sys::VkBufferCopy) -> Self {
        BufferCopy {
            src_offset: copy.srcOffset,
            dst_offset: copy.dstOffset,
            size: copy.size,
        }
    }
}

impl<'a> From<&'a BufferCopy> for vk_sys::VkBufferCopy {
    fn from(copy: &'a BufferCopy) -> Self {
        vk_sys::VkBufferCopy {
            srcOffset: copy.src_offset,
            dstOffset: copy.dst_offset,
            size: copy.size,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ImageSubresourceLayers {
    pub aspect_mask: vk_sys::VkImageAspectFlags,
    pub mip_level: u32,
    pub base_array_layer: u32,
    pub layer_count: u32,
}

impl<'a> From<&'a vk_sys::VkImageSubresourceLayers> for ImageSubresourceLayers {
    fn from(layers: &'a vk_sys::VkImageSubresourceLayers) -> Self {
        ImageSubresourceLayers {
            aspect_mask: layers.aspectMask,
            mip_level: layers.mipLevel,
            base_array_layer: layers.baseArrayLayer,
            layer_count: layers.layerCount,
        }
    }
}

impl<'a> From<&'a ImageSubresourceLayers> for vk_sys::VkImageSubresourceLayers {
    fn from(layers: &'a ImageSubresourceLayers) -> Self {
        vk_sys::VkImageSubresourceLayers {
            aspectMask: layers.aspect_mask,
            mipLevel: layers.mip_level,
            baseArrayLayer: layers.base_array_layer,
            layerCount: layers.layer_count,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ImageCopy {
    pub src_subresource: ImageSubresourceLayers,
    pub src_offset: Offset3D,
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offset: Offset3D,
    pub extent: Extent3D,
}

impl<'a> From<&'a vk_sys::VkImageCopy> for ImageCopy {
    fn from(copy: &'a vk_sys::VkImageCopy) -> Self {
        ImageCopy {
            src_subresource: (&copy.srcSubresource).into(),
            src_offset: (&copy.srcOffset).into(),
            dst_subresource: (&copy.dstSubresource).into(),
            dst_offset: (&copy.dstOffset).into(),
            extent: (&copy.extent).into(),
        }
    }
}

impl<'a> From<&'a ImageCopy> for vk_sys::VkImageCopy {
    fn from(copy: &'a ImageCopy) -> Self {
        vk_sys::VkImageCopy {
            srcSubresource: (&copy.src_subresource).into(),
            srcOffset: (&copy.src_offset).into(),
            dstSubresource: (&copy.dst_subresource).into(),
            dstOffset: (&copy.dst_offset).into(),
            extent: (&copy.extent).into(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ImageBlit {
    pub src_subresource: ImageSubresourceLayers,
    pub src_offsets: [Offset3D; 2],
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offsets: [Offset3D; 2],
}

impl<'a> From<&'a vk_sys::VkImageBlit> for ImageBlit {
    fn from(blit: &'a vk_sys::VkImageBlit) -> Self {
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

impl<'a> From<&'a ImageBlit> for vk_sys::VkImageBlit {
    fn from(blit: &'a ImageBlit) -> Self {
        let src_offsets = [(&blit.src_offsets[0]).into(), (&blit.src_offsets[1]).into()];
        let dst_offsets = [(&blit.dst_offsets[0]).into(), (&blit.dst_offsets[1]).into()];

        vk_sys::VkImageBlit {
            srcSubresource: (&blit.src_subresource).into(),
            srcOffsets: src_offsets,
            dstSubresource: (&blit.dst_subresource).into(),
            dstOffsets: dst_offsets,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BufferImageCopy {
    pub buffer_offset: u64,
    pub buffer_row_length: u32,
    pub buffer_image_height: u32,
    pub image_subresource: ImageSubresourceLayers,
    pub image_offset: Offset3D,
    pub image_extent: Extent3D,
}

impl<'a> From<&'a vk_sys::VkBufferImageCopy> for BufferImageCopy {
    fn from(copy: &'a vk_sys::VkBufferImageCopy) -> Self {
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

impl<'a> From<&'a BufferImageCopy> for vk_sys::VkBufferImageCopy {
    fn from(copy: &'a BufferImageCopy) -> Self {
        vk_sys::VkBufferImageCopy {
            bufferOffset: copy.buffer_offset,
            bufferRowLength: copy.buffer_row_length,
            bufferImageHeight: copy.buffer_image_height,
            imageSubresource: (&copy.image_subresource).into(),
            imageOffset: (&copy.image_offset).into(),
            imageExtent: (&copy.image_extent).into(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ClearColorValue {
    Float32([f32; 4]),
    Int32([i32; 4]),
    UInt32([u32; 4]),
}

impl<'a> From<&'a ClearColorValue> for vk_sys::VkClearColorValue {
    fn from(value: &'a ClearColorValue) -> Self {
        let mut res = vk_sys::VkClearColorValue::default();

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

#[derive(Debug, Copy, Clone)]
pub struct ClearDepthStencilValue {
    pub depth: f32,
    pub stencil: u32,
}

impl<'a> From<&'a vk_sys::VkClearDepthStencilValue> for ClearDepthStencilValue {
    fn from(value: &'a vk_sys::VkClearDepthStencilValue) -> Self {
        ClearDepthStencilValue {
            depth: value.depth,
            stencil: value.stencil,
        }
    }
}

impl<'a> From<&'a ClearDepthStencilValue> for vk_sys::VkClearDepthStencilValue {
    fn from(value: &'a ClearDepthStencilValue) -> Self {
        vk_sys::VkClearDepthStencilValue {
            depth: value.depth,
            stencil: value.stencil,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ClearValue {
    Color(ClearColorValue),
    DepthStencil(ClearDepthStencilValue),
}

impl<'a> From<&'a ClearValue> for vk_sys::VkClearValue {
    fn from(value: &'a ClearValue) -> Self {
        let mut res = vk_sys::VkClearValue::default();

        unsafe {
            match *value {
                ClearValue::Color(ref value) => { *res.color.as_mut() = value.into(); }
                ClearValue::DepthStencil(ref value) => { *res.depthStencil.as_mut() = value.into(); }
            }
        }

        res
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ClearAttachment {
    pub aspect_mask: vk_sys::VkImageAspectFlags,
    pub color_attachment: u32,
    pub clear_value: ClearValue,
}

impl<'a> From<&'a ClearAttachment> for vk_sys::VkClearAttachment {
    fn from(foobar: &'a ClearAttachment) -> Self {
        vk_sys::VkClearAttachment {
            aspectMask: foobar.aspect_mask,
            colorAttachment: foobar.color_attachment,
            clearValue: (&foobar.clear_value).into(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ClearRect {
    pub rect: Rect2D,
    pub base_array_layer: u32,
    pub layer_count: u32,
}

impl<'a> From<&'a vk_sys::VkClearRect> for ClearRect {
    fn from(rect: &'a vk_sys::VkClearRect) -> Self {
        ClearRect {
            rect: (&rect.rect).into(),
            base_array_layer: rect.baseArrayLayer,
            layer_count: rect.layerCount,
        }
    }
}

impl<'a> From<&'a ClearRect> for vk_sys::VkClearRect {
    fn from(rect: &'a ClearRect) -> Self {
        vk_sys::VkClearRect {
            rect: (&rect.rect).into(),
            baseArrayLayer: rect.base_array_layer,
            layerCount: rect.layer_count,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ImageResolve {
    pub src_subresource: ImageSubresourceLayers,
    pub src_offset: Offset3D,
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offset: Offset3D,
    pub extent: Extent3D,
}

impl<'a> From<&'a vk_sys::VkImageResolve> for ImageResolve {
    fn from(resolve: &'a vk_sys::VkImageResolve) -> Self {
        ImageResolve {
            src_subresource: (&resolve.srcSubresource).into(),
            src_offset: (&resolve.srcOffset).into(),
            dst_subresource: (&resolve.dstSubresource).into(),
            dst_offset: (&resolve.dstOffset).into(),
            extent: (&resolve.extent).into(),
        }
    }
}

impl<'a> From<&'a ImageResolve> for vk_sys::VkImageResolve {
    fn from(resolve: &'a ImageResolve) -> Self {
        vk_sys::VkImageResolve {
            srcSubresource: (&resolve.src_subresource).into(),
            srcOffset: (&resolve.src_offset).into(),
            dstSubresource: (&resolve.dst_subresource).into(),
            dstOffset: (&resolve.dst_offset).into(),
            extent: (&resolve.extent).into(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct DispatchIndirectCommand {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl<'a> From<&'a vk_sys::VkDispatchIndirectCommand> for DispatchIndirectCommand {
    fn from(command: &'a vk_sys::VkDispatchIndirectCommand) -> Self {
        DispatchIndirectCommand {
            x: command.x,
            y: command.y,
            z: command.z,
        }
    }
}

impl<'a> From<&'a DispatchIndirectCommand> for vk_sys::VkDispatchIndirectCommand {
    fn from(command: &'a DispatchIndirectCommand) -> Self {
        vk_sys::VkDispatchIndirectCommand {
            x: command.x,
            y: command.y,
            z: command.z,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct DrawIndexedIndirectCommand {
    pub index_count: u32,
    pub instance_count: u32,
    pub first_index: u32,
    pub vertex_offset: i32,
    pub first_instance: u32,
}

impl<'a> From<&'a vk_sys::VkDrawIndexedIndirectCommand> for DrawIndexedIndirectCommand {
    fn from(command: &'a vk_sys::VkDrawIndexedIndirectCommand) -> Self {
        DrawIndexedIndirectCommand {
            index_count: command.indexCount,
            instance_count: command.instanceCount,
            first_index: command.firstIndex,
            vertex_offset: command.vertexOffset,
            first_instance: command.firstInstance,
        }
    }
}

impl<'a> From<&'a DrawIndexedIndirectCommand> for vk_sys::VkDrawIndexedIndirectCommand {
    fn from(command: &'a DrawIndexedIndirectCommand) -> Self {
        vk_sys::VkDrawIndexedIndirectCommand {
            indexCount: command.index_count,
            instanceCount: command.instance_count,
            firstIndex: command.first_index,
            vertexOffset: command.vertex_offset,
            firstInstance: command.first_instance,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct DrawIndirectCommand {
    pub vertex_count: u32,
    pub instance_count: u32,
    pub first_vertex: u32,
    pub first_instance: u32,
}

impl<'a> From<&'a vk_sys::VkDrawIndirectCommand> for DrawIndirectCommand {
    fn from(command: &'a vk_sys::VkDrawIndirectCommand) -> Self {
        DrawIndirectCommand {
            vertex_count: command.vertexCount,
            instance_count: command.instanceCount,
            first_vertex: command.firstVertex,
            first_instance: command.firstInstance,
        }
    }
}

impl<'a> From<&'a DrawIndirectCommand> for vk_sys::VkDrawIndirectCommand {
    fn from(command: &'a DrawIndirectCommand) -> Self {
        vk_sys::VkDrawIndirectCommand {
            vertexCount: command.vertex_count,
            instanceCount: command.instance_count,
            firstVertex: command.first_vertex,
            firstInstance: command.first_instance,
        }
    }
}
