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

use Result;
use core::instance_handle::InstanceHandle;
use core;
use std::mem;
use std::ptr;
use std::sync::Arc;
use utils;
use vk_sys;

#[derive(Debug, Clone)]
pub struct PhysicalDevice {
    instance_handle: Arc<InstanceHandle>,
    physical_device: vk_sys::VkPhysicalDevice,
}

impl PhysicalDevice {
    pub(crate) fn new(instance_handle: Arc<InstanceHandle>, physical_device: vk_sys::VkPhysicalDevice) -> Self {
        PhysicalDevice {
            instance_handle,
            physical_device,
        }
    }

    pub fn properties(&self) -> core::PhysicalDeviceProperties {
        unsafe {
            let mut properties = mem::uninitialized();
            (self.instance_handle.loader.core.vkGetPhysicalDeviceProperties)(self.physical_device, &mut properties);
            properties.into()
        }
    }

    pub fn features(&self) -> core::PhysicalDeviceFeatures {
        unsafe {
            let mut features = mem::uninitialized();
            (self.instance_handle.loader.core.vkGetPhysicalDeviceFeatures)(self.physical_device, &mut features);
            features.into()
        }
    }

    pub fn enumerate_device_layer_properties(&self) -> Result<Vec<core::LayerProperties>> {
        unsafe {
            let mut num_layer_properties = 0;
            let res = (self.instance_handle.loader.core.vkEnumerateDeviceLayerProperties)(self.physical_device, &mut num_layer_properties, ptr::null_mut());
            if res != vk_sys::VK_SUCCESS {
                return Err(res.into());
            }

            let mut layer_properties = Vec::with_capacity(num_layer_properties as usize);
            let res = (self.instance_handle.loader.core.vkEnumerateDeviceLayerProperties)(self.physical_device, &mut num_layer_properties, layer_properties.as_mut_ptr());
            if res != vk_sys::VK_SUCCESS {
                return Err(res.into());
            }
            layer_properties.set_len(num_layer_properties as usize);

            Ok(layer_properties.iter().map(|p| p.into()).collect())
        }
    }

    pub fn enumerate_device_extension_properties(&self, layer_name: Option<String>) -> Result<Vec<core::InstanceExtensionProperties>> {
        unsafe {
            let layer_name_cstr = utils::cstr_from_string(layer_name);

            let mut num_extension_properties = 0;
            let res = (self.instance_handle.loader.core.vkEnumerateDeviceExtensionProperties)(self.physical_device, layer_name_cstr.1, &mut num_extension_properties, ptr::null_mut());
            if res != vk_sys::VK_SUCCESS {
                return Err(res.into());
            }

            let mut extension_properties = Vec::with_capacity(num_extension_properties as usize);
            let res = (self.instance_handle.loader.core.vkEnumerateDeviceExtensionProperties)(self.physical_device, layer_name_cstr.1, &mut num_extension_properties, extension_properties.as_mut_ptr());
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
            (self.instance_handle.loader.core.vkGetPhysicalDeviceFormatProperties)(self.physical_device, format.into(), &mut properties);
        }

        properties.into()
    }
}
