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
mod instance;
mod instance_handle;
mod physical_device;

use libc::c_void;
use std::ffi::CStr;
use std::fmt;
use utils;
use vk_sys;

pub use self::instance::Instance;
pub use self::physical_device::PhysicalDevice;

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

    pub fn as_api_version(&self) -> u32 {
        vk_sys::vk_make_version(self.major, self.minor, self.patch)
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

#[derive(Debug, Clone)]
pub struct ApplicationInfo {
    pub application_name: Option<String>,
    pub application_version: u32,
    pub engine_name: Option<String>,
    pub engine_version: u32,
    pub api_version: Option<Version>,
}

#[derive(Debug, Clone)]
pub struct InstanceCreateInfo {
    pub flags: vk_sys::VkInstanceCreateFlags,
    pub application_info: Option<ApplicationInfo>,
    pub enabled_layers: Vec<String>,
    pub enabled_extensions: Vec<InstanceExtension>,
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

impl From<vk_sys::VkPhysicalDeviceFeatures> for PhysicalDeviceFeatures {
    fn from(featurs: vk_sys::VkPhysicalDeviceFeatures) -> Self {
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

impl From<PhysicalDeviceFeatures> for vk_sys::VkPhysicalDeviceFeatures {
    fn from(featurs: PhysicalDeviceFeatures) -> Self {
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

impl From<vk_sys::VkFormatProperties> for FormatProperties {
    fn from(properties: vk_sys::VkFormatProperties) -> Self {
        FormatProperties {
            linear_tiling_features: properties.linearTilingFeatures,
            optimal_tiling_features: properties.optimalTilingFeatures,
            buffer_features: properties.bufferFeatures,
        }
    }
}

impl From<FormatProperties> for vk_sys::VkFormatProperties {
    fn from(properties: FormatProperties) -> Self {
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

impl From<vk_sys::VkExtent3D> for Extent3D {
    fn from(extent: vk_sys::VkExtent3D) -> Self {
        Extent3D {
            width: extent.width,
            height: extent.height,
            depth: extent.depth,
        }
    }
}

impl From<Extent3D> for vk_sys::VkExtent3D {
    fn from(extent: Extent3D) -> Self {
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

impl From<vk_sys::VkImageFormatProperties> for ImageFormatProperties {
    fn from(properties: vk_sys::VkImageFormatProperties) -> Self {
        ImageFormatProperties {
            max_extent: properties.maxExtent.into(),
            max_mip_levels: properties.maxMipLevels,
            max_array_layers: properties.maxArrayLayers,
            sample_counts: properties.sampleCounts,
            max_resource_size: properties.maxResourceSize,
        }
    }
}

impl From<ImageFormatProperties> for vk_sys::VkImageFormatProperties {
    fn from(properties: ImageFormatProperties) -> Self {
        vk_sys::VkImageFormatProperties {
            maxExtent: properties.max_extent.into(),
            maxMipLevels: properties.max_mip_levels,
            maxArrayLayers: properties.max_array_layers,
            sampleCounts: properties.sample_counts,
            maxResourceSize: properties.max_resource_size,
        }
    }
}

#[derive(Debug, Clone)]
pub enum InstanceExtension {
    Unknown(String),
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

#[derive(Debug, Clone)]
pub enum DeviceExtension {
    Unknown(String),
}

impl<'a> From<&'a str> for DeviceExtension {
    fn from(name: &'a str) -> Self {
        DeviceExtension::Unknown(name.to_owned())
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
