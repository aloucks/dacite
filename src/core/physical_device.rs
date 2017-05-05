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

use AsNativeVkObject;
use Result;
use core::allocator_helper::AllocatorHelper;
use core::{self, Device, Instance};
use std::mem;
use std::ptr;
use utils;
use vk_sys;

#[derive(Debug, Clone)]
pub struct PhysicalDevice {
    handle: vk_sys::VkPhysicalDevice,
    instance: Instance,
}

impl AsNativeVkObject for PhysicalDevice {
    type NativeVkObject = vk_sys::VkPhysicalDevice;

    #[inline]
    fn as_native_vk_object(&self) -> Self::NativeVkObject {
        self.handle
    }
}

impl PhysicalDevice {
    pub(crate) fn new(handle: vk_sys::VkPhysicalDevice, instance: Instance) -> Self {
        PhysicalDevice {
            handle: handle,
            instance: instance,
        }
    }

    #[inline]
    pub(crate) fn loader(&self) -> &vk_sys::InstanceProcAddrLoader {
        self.instance.loader()
    }

    pub fn properties(&self) -> core::PhysicalDeviceProperties {
        unsafe {
            let mut properties = mem::uninitialized();
            (self.loader().core.vkGetPhysicalDeviceProperties)(self.handle, &mut properties);
            (&properties).into()
        }
    }

    pub fn features(&self) -> core::PhysicalDeviceFeatures {
        unsafe {
            let mut features = mem::uninitialized();
            (self.loader().core.vkGetPhysicalDeviceFeatures)(self.handle, &mut features);
            (&features).into()
        }
    }

    pub fn enumerate_device_layer_properties(&self) -> Result<Vec<core::LayerProperties>> {
        unsafe {
            let mut num_layer_properties = 0;
            let res = (self.loader().core.vkEnumerateDeviceLayerProperties)(self.handle, &mut num_layer_properties, ptr::null_mut());
            if res != vk_sys::VK_SUCCESS {
                return Err(res.into());
            }

            let mut layer_properties = Vec::with_capacity(num_layer_properties as usize);
            let res = (self.loader().core.vkEnumerateDeviceLayerProperties)(self.handle, &mut num_layer_properties, layer_properties.as_mut_ptr());
            if res != vk_sys::VK_SUCCESS {
                return Err(res.into());
            }
            layer_properties.set_len(num_layer_properties as usize);

            Ok(layer_properties.iter().map(|p| p.into()).collect())
        }
    }

    pub fn enumerate_device_extension_properties(&self, layer_name: Option<&str>) -> Result<Vec<core::InstanceExtensionProperties>> {
        unsafe {
            let layer_name_cstr = utils::cstr_from_str(layer_name);

            let mut num_extension_properties = 0;
            let res = (self.loader().core.vkEnumerateDeviceExtensionProperties)(self.handle, layer_name_cstr.1, &mut num_extension_properties, ptr::null_mut());
            if res != vk_sys::VK_SUCCESS {
                return Err(res.into());
            }

            let mut extension_properties = Vec::with_capacity(num_extension_properties as usize);
            let res = (self.loader().core.vkEnumerateDeviceExtensionProperties)(self.handle, layer_name_cstr.1, &mut num_extension_properties, extension_properties.as_mut_ptr());
            if res != vk_sys::VK_SUCCESS {
                return Err(res.into());
            }
            extension_properties.set_len(num_extension_properties as usize);

            Ok(extension_properties.iter().map(|p| p.into()).collect())
        }
    }

    pub fn format_properties(&self, format: core::Format) -> core::FormatProperties {
        let mut properties = unsafe { mem::uninitialized() };

        unsafe {
            (self.loader().core.vkGetPhysicalDeviceFormatProperties)(self.handle, format.into(), &mut properties);
        }

        (&properties).into()
    }

    pub fn image_format_properties(&self, format: core::Format, image_type: core::ImageType, tiling: core::ImageTiling, usage: vk_sys::VkImageUsageFlags, flags: vk_sys::VkImageCreateFlags) -> Result<core::ImageFormatProperties> {
        let mut properties = unsafe { mem::uninitialized() };

        let res = unsafe {
            (self.loader().core.vkGetPhysicalDeviceImageFormatProperties)(self.handle, format.into(), image_type.into(), tiling.into(), usage, flags, &mut properties)
        };

        if res == vk_sys::VK_SUCCESS {
            Ok((&properties).into())
        }
        else {
            Err(res.into())
        }
    }

    pub fn sparse_image_format_properties(&self, format: core::Format, image_type: core::ImageType, samples: vk_sys::VkSampleCountFlagBits, usage: vk_sys::VkImageUsageFlags, tiling: core::ImageTiling) -> Vec<core::SparseImageFormatProperties> {
        let mut num_properties = 0;
        unsafe {
            (self.loader().core.vkGetPhysicalDeviceSparseImageFormatProperties)(self.handle, format.into(), image_type.into(), samples, usage, tiling.into(), &mut num_properties, ptr::null_mut());
        }

        let mut properties = Vec::with_capacity(num_properties as usize);
        unsafe {
            (self.loader().core.vkGetPhysicalDeviceSparseImageFormatProperties)(self.handle, format.into(), image_type.into(), samples, usage, tiling.into(), &mut num_properties, properties.as_mut_ptr());
            properties.set_len(num_properties as usize);
        }

        properties.iter().map(From::from).collect()
    }

    pub fn queue_family_properties(&self) -> Vec<core::QueueFamilyProperties> {
        let mut num_properties = 0;
        unsafe {
            (self.loader().core.vkGetPhysicalDeviceQueueFamilyProperties)(self.handle, &mut num_properties, ptr::null_mut());
        }

        let mut properties = Vec::with_capacity(num_properties as usize);
        unsafe {
            (self.loader().core.vkGetPhysicalDeviceQueueFamilyProperties)(self.handle, &mut num_properties, properties.as_mut_ptr());
            properties.set_len(num_properties as usize);
        }

        properties.iter()
            .map(From::from)
            .collect()
    }

    pub fn memory_properties(&self) -> core::PhysicalDeviceMemoryProperties {
        let mut properties = unsafe { mem::uninitialized() };

        unsafe {
            (self.loader().core.vkGetPhysicalDeviceMemoryProperties)(self.handle, &mut properties);
        }

        (&properties).into()
    }

    pub fn create_device(&self, create_info: &core::DeviceCreateInfo, allocator: Option<Box<core::Allocator>>) -> Result<core::Device> {
        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), |a| &a.callbacks);
        let create_info: core::VkDeviceCreateInfoWrapper = create_info.into();

        let mut device = ptr::null_mut();
        let res = unsafe {
            (self.loader().core.vkCreateDevice)(self.handle, create_info.as_ref(), allocation_callbacks, &mut device)
        };

        if res != vk_sys::VK_SUCCESS {
            return Err(res.into());
        }

        Ok(Device::new(device, self.instance.clone(), allocator_helper))
    }
}
