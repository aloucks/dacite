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

use core::PhysicalDevice;
use core::allocator_helper::AllocatorHelper;
use core;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ptr;
use std::sync::Arc;
use utils;
use vks;
use {TryDestroyError, TryDestroyErrorKind, VulkanObject};

/// See [`VkInstance`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkInstance)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instance(Arc<Inner>);

impl VulkanObject for Instance {
    type NativeVulkanObject = vks::VkInstance;

    #[inline]
    fn as_native_vulkan_object(&self) -> Self::NativeVulkanObject {
        self.handle()
    }

    fn try_destroy(self) -> Result<(), TryDestroyError<Self>> {
        let strong_count = Arc::strong_count(&self.0);
        if strong_count == 1 {
            Ok(())
        }
        else {
            Err(TryDestroyError::new(self, TryDestroyErrorKind::InUse(Some(strong_count))))
        }
    }
}

impl Instance {
    #[inline]
    pub(crate) fn handle(&self) -> vks::VkInstance {
        self.0.handle
    }

    #[inline]
    pub(crate) fn loader(&self) -> &vks::InstanceProcAddrLoader {
        &self.0.loader
    }

    /// See [`vkCreateInstance`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateInstance)
    pub fn create(create_info: &core::InstanceCreateInfo, allocator: Option<Box<core::Allocator>>) -> Result<Instance, core::Error> {
        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), |a| &a.callbacks);

        let mut loader = vks::InstanceProcAddrLoader::from_get_instance_proc_addr(vks::vkGetInstanceProcAddr);
        unsafe {
            loader.load_core_null_instance();
        }

        let create_info: core::VkInstanceCreateInfoWrapper = create_info.into();
        let mut instance = ptr::null_mut();
        let res = unsafe {
            (loader.core_null_instance.vkCreateInstance)(create_info.as_ref(), allocation_callbacks, &mut instance)
        };
        if res != vks::VK_SUCCESS {
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

    /// See [`vkEnumeratePhysicalDevices`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumeratePhysicalDevices)
    pub fn enumerate_physical_devices(&self) -> Result<Vec<PhysicalDevice>, core::Error> {
        let mut num_physical_devices = 0;
        let res = unsafe {
            (self.loader().core.vkEnumeratePhysicalDevices)(self.handle(), &mut num_physical_devices, ptr::null_mut())
        };
        if res != vks::VK_SUCCESS {
            return Err(res.into());
        }

        let mut physical_devices = Vec::with_capacity(num_physical_devices as usize);
        let res = unsafe {
            (self.loader().core.vkEnumeratePhysicalDevices)(self.handle(), &mut num_physical_devices, physical_devices.as_mut_ptr())
        };
        if res != vks::VK_SUCCESS {
            return Err(res.into());
        }
        unsafe {
            physical_devices.set_len(num_physical_devices as usize);
        }

        let physical_devices: Vec<_> = physical_devices
            .iter()
            .map(|&d| core::PhysicalDevice::new(d, self.clone()))
            .collect();

        Ok(physical_devices)
    }

    /// See [`vkEnumerateInstanceLayerProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumerateInstanceLayerProperties)
    pub fn enumerate_instance_layer_properties() -> Result<Vec<core::LayerProperties>, core::Error> {
        unsafe {
            let mut loader = vks::instance_proc_addr_loader::CoreNullInstance::new();
            loader.load(vks::vkGetInstanceProcAddr, ptr::null_mut());

            let mut num_layer_properties = 0;
            let res = (loader.vkEnumerateInstanceLayerProperties)(&mut num_layer_properties, ptr::null_mut());
            if res != vks::VK_SUCCESS {
                return Err(res.into());
            }

            let mut layer_properties = Vec::with_capacity(num_layer_properties as usize);
            let res = (loader.vkEnumerateInstanceLayerProperties)(&mut num_layer_properties, layer_properties.as_mut_ptr());
            if res != vks::VK_SUCCESS {
                return Err(res.into());
            }
            layer_properties.set_len(num_layer_properties as usize);

            Ok(layer_properties.iter().map(|p| p.into()).collect())
        }
    }

    /// See [`vkEnumerateInstanceExtensionProperties`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEnumerateInstanceExtensionProperties)
    pub fn enumerate_instance_extension_properties(layer_name: Option<&str>) -> Result<Vec<core::InstanceExtensionProperties>, core::Error> {
        unsafe {
            let mut loader = vks::instance_proc_addr_loader::CoreNullInstance::new();
            loader.load(vks::vkGetInstanceProcAddr, ptr::null_mut());

            let layer_name_cstr = utils::cstr_from_str(layer_name);

            let mut num_extension_properties = 0;
            let res = (loader.vkEnumerateInstanceExtensionProperties)(layer_name_cstr.1, &mut num_extension_properties, ptr::null_mut());
            if res != vks::VK_SUCCESS {
                return Err(res.into());
            }

            let mut extension_properties = Vec::with_capacity(num_extension_properties as usize);
            let res = (loader.vkEnumerateInstanceExtensionProperties)(layer_name_cstr.1, &mut num_extension_properties, extension_properties.as_mut_ptr());
            if res != vks::VK_SUCCESS {
                return Err(res.into());
            }
            extension_properties.set_len(num_extension_properties as usize);

            Ok(extension_properties.iter().map(|p| p.into()).collect())
        }
    }
}

#[derive(Debug)]
struct Inner {
    handle: vks::VkInstance,
    allocator: Option<AllocatorHelper>,
    loader: vks::InstanceProcAddrLoader,
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

unsafe impl Send for Inner { }

unsafe impl Sync for Inner { }

impl PartialEq for Inner {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}

impl Eq for Inner { }

impl PartialOrd for Inner {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.handle.partial_cmp(&other.handle)
    }
}

impl Ord for Inner {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.handle.cmp(&other.handle)
    }
}

impl Hash for Inner {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.handle.hash(state);
    }
}
