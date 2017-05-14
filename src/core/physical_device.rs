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

use core::allocator_helper::AllocatorHelper;
use core::{self, Device, Instance};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::mem;
use std::ptr;
use utils;
use vks;
use {TryDestroyError, TryDestroyErrorKind, VulkanObject};

/// See [`VkPhysicalDevice`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkPhysicalDevice)
#[derive(Debug, Clone)]
pub struct PhysicalDevice {
    handle: vks::VkPhysicalDevice,
    instance: Instance,
}

unsafe impl Send for PhysicalDevice { }

unsafe impl Sync for PhysicalDevice { }

impl PartialEq for PhysicalDevice {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}

impl Eq for PhysicalDevice { }

impl PartialOrd for PhysicalDevice {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.handle.partial_cmp(&other.handle)
    }
}

impl Ord for PhysicalDevice {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.handle.cmp(&other.handle)
    }
}

impl Hash for PhysicalDevice {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.handle.hash(state);
    }
}

impl VulkanObject for PhysicalDevice {
    type NativeVulkanObject = vks::VkPhysicalDevice;

    #[inline]
    fn as_native_vulkan_object(&self) -> Self::NativeVulkanObject {
        self.handle
    }

    fn try_destroy(self) -> Result<(), TryDestroyError<Self>> {
        Err(TryDestroyError::new(self, TryDestroyErrorKind::Unsupported))
    }
}

impl PhysicalDevice {
    pub(crate) fn new(handle: vks::VkPhysicalDevice, instance: Instance) -> Self {
        PhysicalDevice {
            handle: handle,
            instance: instance,
        }
    }

    #[inline]
    pub(crate) fn loader(&self) -> &vks::InstanceProcAddrLoader {
        self.instance.loader()
    }

    /// See [`vkGetPhysicalDeviceProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceProperties)
    pub fn properties(&self) -> core::PhysicalDeviceProperties {
        unsafe {
            let mut properties = mem::uninitialized();
            (self.loader().core.vkGetPhysicalDeviceProperties)(self.handle, &mut properties);
            (&properties).into()
        }
    }

    /// See [`vkGetPhysicalDeviceFeatures`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceFeatures)
    pub fn features(&self) -> core::PhysicalDeviceFeatures {
        unsafe {
            let mut features = mem::uninitialized();
            (self.loader().core.vkGetPhysicalDeviceFeatures)(self.handle, &mut features);
            (&features).into()
        }
    }

    /// See [`vkEnumerateDeviceLayerProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumerateDeviceLayerProperties)
    pub fn enumerate_device_layer_properties(&self) -> Result<core::LayerPropertiesIterator, core::Error> {
        unsafe {
            let mut num_layer_properties = 0;
            let res = (self.loader().core.vkEnumerateDeviceLayerProperties)(self.handle, &mut num_layer_properties, ptr::null_mut());
            if res != vks::VK_SUCCESS {
                return Err(res.into());
            }

            let mut layer_properties = Vec::with_capacity(num_layer_properties as usize);
            let res = (self.loader().core.vkEnumerateDeviceLayerProperties)(self.handle, &mut num_layer_properties, layer_properties.as_mut_ptr());
            if res != vks::VK_SUCCESS {
                return Err(res.into());
            }
            layer_properties.set_len(num_layer_properties as usize);

            Ok(core::LayerPropertiesIterator(layer_properties.into_iter()))
        }
    }

    /// See [`vkEnumerateDeviceExtensionProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumerateDeviceExtensionProperties)
    pub fn enumerate_device_extension_properties(&self, layer_name: Option<&str>) -> Result<core::DeviceExtensionPropertiesIterator, core::Error> {
        unsafe {
            let layer_name_cstr = utils::cstr_from_str(layer_name);

            let mut num_extension_properties = 0;
            let res = (self.loader().core.vkEnumerateDeviceExtensionProperties)(self.handle, layer_name_cstr.1, &mut num_extension_properties, ptr::null_mut());
            if res != vks::VK_SUCCESS {
                return Err(res.into());
            }

            let mut extension_properties = Vec::with_capacity(num_extension_properties as usize);
            let res = (self.loader().core.vkEnumerateDeviceExtensionProperties)(self.handle, layer_name_cstr.1, &mut num_extension_properties, extension_properties.as_mut_ptr());
            if res != vks::VK_SUCCESS {
                return Err(res.into());
            }
            extension_properties.set_len(num_extension_properties as usize);

            Ok(core::DeviceExtensionPropertiesIterator(extension_properties.into_iter()))
        }
    }

    /// See [`vkGetPhysicalDeviceFormatProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceFormatProperties)
    pub fn format_properties(&self, format: core::Format) -> core::FormatProperties {
        let mut properties = unsafe { mem::uninitialized() };

        unsafe {
            (self.loader().core.vkGetPhysicalDeviceFormatProperties)(self.handle, format.into(), &mut properties);
        }

        (&properties).into()
    }

    /// See [`vkGetPhysicalDeviceImageFormatProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceImageFormatProperties)
    pub fn image_format_properties(&self, format: core::Format, image_type: core::ImageType, tiling: core::ImageTiling, usage: core::ImageUsageFlags, flags: core::ImageCreateFlags) -> Result<core::ImageFormatProperties, core::Error> {
        let mut properties = unsafe { mem::uninitialized() };

        let res = unsafe {
            (self.loader().core.vkGetPhysicalDeviceImageFormatProperties)(self.handle, format.into(), image_type.into(), tiling.into(), usage, flags, &mut properties)
        };

        if res == vks::VK_SUCCESS {
            Ok((&properties).into())
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkGetPhysicalDeviceSparseImageFormatProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceSparseImageFormatProperties)
    pub fn sparse_image_format_properties(&self, format: core::Format, image_type: core::ImageType, samples: core::SampleCountFlagBits, usage: core::ImageUsageFlags, tiling: core::ImageTiling) -> core::SparseImageFormatPropertiesIterator {
        let mut num_properties = 0;
        unsafe {
            (self.loader().core.vkGetPhysicalDeviceSparseImageFormatProperties)(self.handle, format.into(), image_type.into(), samples, usage, tiling.into(), &mut num_properties, ptr::null_mut());
        }

        let mut properties = Vec::with_capacity(num_properties as usize);
        unsafe {
            (self.loader().core.vkGetPhysicalDeviceSparseImageFormatProperties)(self.handle, format.into(), image_type.into(), samples, usage, tiling.into(), &mut num_properties, properties.as_mut_ptr());
            properties.set_len(num_properties as usize);
        }

        core::SparseImageFormatPropertiesIterator(properties.into_iter())
    }

    /// See [`vkGetPhysicalDeviceQueueFamilyProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceQueueFamilyProperties)
    pub fn queue_family_properties(&self) -> core::QueueFamilyPropertiesIterator {
        let mut num_properties = 0;
        unsafe {
            (self.loader().core.vkGetPhysicalDeviceQueueFamilyProperties)(self.handle, &mut num_properties, ptr::null_mut());
        }

        let mut properties = Vec::with_capacity(num_properties as usize);
        unsafe {
            (self.loader().core.vkGetPhysicalDeviceQueueFamilyProperties)(self.handle, &mut num_properties, properties.as_mut_ptr());
            properties.set_len(num_properties as usize);
        }

        core::QueueFamilyPropertiesIterator(properties.into_iter())
    }

    /// See [`vkGetPhysicalDeviceMemoryProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetPhysicalDeviceMemoryProperties)
    pub fn memory_properties(&self) -> core::PhysicalDeviceMemoryProperties {
        let mut properties = unsafe { mem::uninitialized() };

        unsafe {
            (self.loader().core.vkGetPhysicalDeviceMemoryProperties)(self.handle, &mut properties);
        }

        (&properties).into()
    }

    /// See [`vkCreateDevice`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDevice)
    pub fn create_device(&self, create_info: &core::DeviceCreateInfo, allocator: Option<Box<core::Allocator>>) -> Result<Device, core::Error> {
        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), AllocatorHelper::callbacks);
        let create_info: core::VkDeviceCreateInfoWrapper = create_info.into();

        let mut device = ptr::null_mut();
        let res = unsafe {
            (self.loader().core.vkCreateDevice)(self.handle, create_info.as_ref(), allocation_callbacks, &mut device)
        };

        if res != vks::VK_SUCCESS {
            return Err(res.into());
        }

        Ok(Device::new(device, self.instance.clone(), allocator_helper))
    }
}
