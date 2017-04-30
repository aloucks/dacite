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
use core::allocator_helper::AllocatorHelper;
use core;
use std::ptr;
use std::sync::Arc;
use utils;
use vk_sys;

#[derive(Debug)]
pub(crate) struct Inner {
    pub handle: vk_sys::VkInstance,
    pub allocator: Option<AllocatorHelper>,
    pub loader: vk_sys::InstanceProcAddrLoader,
}

impl Drop for Inner {
    fn drop(&mut self) {
        let allocator = match self.allocator {
            Some(ref allocator) => &allocator.callbacks,
            None => ptr::null(),
        };

        unsafe {
            (self.loader.core.vkDestroyInstance)(self.handle, allocator);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Instance(pub(crate) Arc<Inner>);

impl Instance {
    pub fn create(create_info: &core::InstanceCreateInfo, allocator: Option<Box<core::Allocator>>) -> Result<Instance> {
        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), |a| &a.callbacks);

        let mut loader = vk_sys::InstanceProcAddrLoader::new(vk_sys::vkGetInstanceProcAddr);
        unsafe {
            loader.load_core_null_instance();
        }

        let create_info: core::VkInstanceCreateInfoWrapper = create_info.into();
        let create_info: &vk_sys::VkInstanceCreateInfo = &create_info;
        let mut instance = ptr::null_mut();
        let res = unsafe {
            (loader.core_null_instance.vkCreateInstance)(create_info, allocation_callbacks, &mut instance)
        };
        if res != vk_sys::VK_SUCCESS {
            return Err(res.into());
        }

        unsafe {
            loader.load_core(instance);
        }

        Ok(Instance(Arc::new(Inner {
            handle: instance,
            allocator: allocator_helper,
            loader: loader,
        })))
    }

    pub fn enumerate_physical_devices(&self) -> Result<Vec<core::PhysicalDevice>> {
        let mut num_physical_devices = 0;
        let res = unsafe {
            (self.0.loader.core.vkEnumeratePhysicalDevices)(self.0.handle, &mut num_physical_devices, ptr::null_mut())
        };
        if res != vk_sys::VK_SUCCESS {
            return Err(res.into());
        }

        let mut physical_devices = Vec::with_capacity(num_physical_devices as usize);
        let res = unsafe {
            (self.0.loader.core.vkEnumeratePhysicalDevices)(self.0.handle, &mut num_physical_devices, physical_devices.as_mut_ptr())
        };
        if res != vk_sys::VK_SUCCESS {
            return Err(res.into());
        }
        unsafe {
            physical_devices.set_len(num_physical_devices as usize);
        }

        let physical_devices: Vec<_> = physical_devices
            .iter()
            .map(|&d| core::PhysicalDevice::new(self.clone(), d))
            .collect();

        Ok(physical_devices)
    }

    pub fn enumerate_instance_layer_properties() -> Result<Vec<core::LayerProperties>> {
        unsafe {
            let mut loader = vk_sys::instance_proc_addr_loader::CoreNullInstance::new();
            loader.load(vk_sys::vkGetInstanceProcAddr, ptr::null_mut());

            let mut num_layer_properties = 0;
            let res = (loader.vkEnumerateInstanceLayerProperties)(&mut num_layer_properties, ptr::null_mut());
            if res != vk_sys::VK_SUCCESS {
                return Err(res.into());
            }

            let mut layer_properties = Vec::with_capacity(num_layer_properties as usize);
            let res = (loader.vkEnumerateInstanceLayerProperties)(&mut num_layer_properties, layer_properties.as_mut_ptr());
            if res != vk_sys::VK_SUCCESS {
                return Err(res.into());
            }
            layer_properties.set_len(num_layer_properties as usize);

            Ok(layer_properties.iter().map(|p| p.into()).collect())
        }
    }

    pub fn enumerate_instance_extension_properties(layer_name: Option<&str>) -> Result<Vec<core::InstanceExtensionProperties>> {
        unsafe {
            let mut loader = vk_sys::instance_proc_addr_loader::CoreNullInstance::new();
            loader.load(vk_sys::vkGetInstanceProcAddr, ptr::null_mut());

            let layer_name_cstr = utils::cstr_from_str(layer_name);

            let mut num_extension_properties = 0;
            let res = (loader.vkEnumerateInstanceExtensionProperties)(layer_name_cstr.1, &mut num_extension_properties, ptr::null_mut());
            if res != vk_sys::VK_SUCCESS {
                return Err(res.into());
            }

            let mut extension_properties = Vec::with_capacity(num_extension_properties as usize);
            let res = (loader.vkEnumerateInstanceExtensionProperties)(layer_name_cstr.1, &mut num_extension_properties, extension_properties.as_mut_ptr());
            if res != vk_sys::VK_SUCCESS {
                return Err(res.into());
            }
            extension_properties.set_len(num_extension_properties as usize);

            Ok(extension_properties.iter().map(|p| p.into()).collect())
        }
    }
}
