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
use core::instance_handle::InstanceHandle;
use core;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::sync::Arc;
use vk_sys;

#[derive(Debug, Clone)]
pub struct Instance(Arc<InstanceHandle>);

impl Instance {
    pub fn create(mut create_info: core::InstanceCreateInfo, allocator: Option<Box<core::Allocator>>) -> Result<Instance> {
        let mut application_name_cstr = None;
        let mut engine_name_cstr = None;

        let application_info = create_info.application_info.map(|a| {
            let application_name_ptr = match a.application_name {
                Some(application_name) => {
                    let tmp = CString::new(application_name).unwrap();
                    let ptr = tmp.as_ptr();
                    application_name_cstr = Some(tmp);
                    ptr
                }

                None => ptr::null(),
            };

            let engine_name_ptr = match a.engine_name {
                Some(engine_name) => {
                    let tmp = CString::new(engine_name).unwrap();
                    let ptr = tmp.as_ptr();
                    engine_name_cstr = Some(tmp);
                    ptr
                }

                None => ptr::null(),
            };

            let api_version = match a.api_version {
                Some(version) => vk_sys::vk_make_version(version.major as u32, version.minor as u32, version.patch as u32),
                None => 0,
            };

            vk_sys::VkApplicationInfo {
                sType: vk_sys::VK_STRUCTURE_TYPE_APPLICATION_INFO,
                pNext: ptr::null(),
                pApplicationName: application_name_ptr,
                applicationVersion: a.application_version,
                pEngineName: engine_name_ptr,
                engineVersion: a.engine_version,
                apiVersion: api_version,
            }
        });

        let application_info_ptr = match application_info {
            Some(ref application_info) => application_info,
            None => ptr::null(),
        };

        let enabled_layer_names_cstr: Vec<_> = create_info.enabled_layers
            .drain(..)
            .map(|l| CString::new(l).unwrap())
            .collect();
        let enabled_layer_names_ptrs: Vec<_> = enabled_layer_names_cstr
            .iter()
            .map(|l| l.as_ptr())
            .collect();

        let enabled_extension_names_cstr: Vec<_> = create_info.enabled_extensions
            .drain(..)
            .map(|e| CString::new(String::from(e).into_bytes()).unwrap())
            .collect();
        let enabled_extension_names_ptrs: Vec<_> = enabled_extension_names_cstr
            .iter()
            .map(|l| l.as_ptr())
            .collect();

        let create_info = vk_sys::VkInstanceCreateInfo {
            sType: vk_sys::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            pNext: ptr::null(),
            flags: create_info.flags,
            pApplicationInfo: application_info_ptr,
            enabledLayerCount: enabled_layer_names_ptrs.len() as u32,
            ppEnabledLayerNames: enabled_layer_names_ptrs.as_ptr(),
            enabledExtensionCount: enabled_extension_names_ptrs.len() as u32,
            ppEnabledExtensionNames: enabled_extension_names_ptrs.as_ptr(),
        };

        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), |a| &a.callbacks);

        let mut loader = vk_sys::InstanceProcAddrLoader::new(vk_sys::vkGetInstanceProcAddr);
        unsafe {
            loader.load_core_null_instance();
        }

        let mut instance = ptr::null_mut();
        let res = unsafe {
            (loader.core_null_instance.vkCreateInstance)(&create_info, allocation_callbacks, &mut instance)
        };
        if res != vk_sys::VK_SUCCESS {
            return Err(res.into());
        }

        unsafe {
            loader.load_core(instance);
        }

        Ok(Instance(Arc::new(InstanceHandle {
            instance: instance,
            allocator: allocator_helper,
            loader: loader,
        })))
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

    pub fn enumerate_instance_extension_properties(layer_name: Option<String>) -> Result<Vec<core::InstanceExtensionProperties>> {
        unsafe {
            let mut loader = vk_sys::instance_proc_addr_loader::CoreNullInstance::new();
            loader.load(vk_sys::vkGetInstanceProcAddr, ptr::null_mut());

            let mut layer_name_cstr = None;
            let layer_name_ptr = match layer_name {
                Some(layer_name) => {
                    let tmp = CString::new(layer_name).unwrap();
                    let ptr = tmp.as_ptr();
                    layer_name_cstr = Some(tmp);
                    ptr
                }

                None => ptr::null(),
            };

            let mut num_extension_properties = 0;
            let res = (loader.vkEnumerateInstanceExtensionProperties)(layer_name_ptr, &mut num_extension_properties, ptr::null_mut());
            if res != vk_sys::VK_SUCCESS {
                return Err(res.into());
            }

            let mut extension_properties = Vec::with_capacity(num_extension_properties as usize);
            let res = (loader.vkEnumerateInstanceExtensionProperties)(layer_name_ptr, &mut num_extension_properties, extension_properties.as_mut_ptr());
            if res != vk_sys::VK_SUCCESS {
                return Err(res.into());
            }
            extension_properties.set_len(num_extension_properties as usize);

            // This is unnecessary, but silences two warnings about layer_name_cstr.
            // No idea why there are warnings here, but not above in Instance::create.
            mem::drop(layer_name_cstr);

            Ok(extension_properties.iter().map(|p| p.into()).collect())
        }
    }
}
