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
