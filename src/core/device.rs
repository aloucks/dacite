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
use core::{
    self,
    Buffer,
    BufferView,
    CommandPool,
    DescriptorPool,
    DescriptorSetLayout,
    Event,
    Fence,
    Image,
    ImageView,
    Instance,
    PipelineCache,
    QueryPool,
    Queue,
    Sampler,
    Semaphore,
    ShaderModule,
};
use std::ptr;
use std::sync::Arc;
use vk_sys;

#[derive(Debug)]
struct Inner {
    handle: vk_sys::VkDevice,
    instance: Instance,
    allocator: Option<AllocatorHelper>,
    loader: vk_sys::DeviceProcAddrLoader,
}

impl Drop for Inner {
    fn drop(&mut self) {
        let allocator = match self.allocator {
            Some(ref allocator) => &allocator.callbacks,
            None => ptr::null(),
        };

        unsafe {
            (self.loader.core.vkDestroyDevice)(self.handle, allocator);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Device(Arc<Inner>);

impl Device {
    pub(crate) fn new(handle: vk_sys::VkDevice, instance: Instance, allocator: Option<AllocatorHelper>) -> Self {
        let mut loader = vk_sys::DeviceProcAddrLoader::new(instance.loader().core.vkGetDeviceProcAddr);
        unsafe {
            loader.load_core(handle);
        }

        Device(Arc::new(Inner {
            handle: handle,
            instance: instance,
            allocator: allocator,
            loader: loader,
        }))
    }

    #[inline]
    pub(crate) fn handle(&self) -> vk_sys::VkDevice {
        self.0.handle
    }

    #[inline]
    pub(crate) fn loader(&self) -> &vk_sys::DeviceProcAddrLoader {
        &self.0.loader
    }

    pub fn get_queue(&self, queue_family_index: u32, queue_index: u32) -> Queue {
        let mut queue = ptr::null_mut();
        unsafe {
            (self.loader().core.vkGetDeviceQueue)(self.handle(), queue_family_index, queue_index, &mut queue);
        }

        Queue::new(queue, self.clone())
    }

    pub fn create_command_pool(&self, create_info: &core::CommandPoolCreateInfo, allocator: Option<Box<core::Allocator>>) -> Result<CommandPool> {
        let create_info: core::VkCommandPoolCreateInfoWrapper = create_info.into();

        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), |a| &a.callbacks);

        let mut command_pool = ptr::null_mut();
        let res = unsafe {
            (self.loader().core.vkCreateCommandPool)(self.handle(), create_info.as_ref(), allocation_callbacks, &mut command_pool)
        };

        if res == vk_sys::VK_SUCCESS {
            Ok(CommandPool::new(command_pool, self.clone(), allocator_helper))
        }
        else {
            Err(res.into())
        }
    }

    pub fn create_fence(&self, create_info: &core::FenceCreateInfo, allocator: Option<Box<core::Allocator>>) -> Result<Fence> {
        let create_info: core::VkFenceCreateInfoWrapper = create_info.into();

        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), |a| &a.callbacks);

        let mut fence = ptr::null_mut();
        let res = unsafe {
            (self.loader().core.vkCreateFence)(self.handle(), create_info.as_ref(), allocation_callbacks, &mut fence)
        };

        if res == vk_sys::VK_SUCCESS {
            Ok(Fence::new(fence, self.clone(), allocator_helper))
        }
        else {
            Err(res.into())
        }
    }

    pub fn create_semaphore(&self, create_info: &core::SemaphoreCreateInfo, allocator: Option<Box<core::Allocator>>) -> Result<Semaphore> {
        let create_info: core::VkSemaphoreCreateInfoWrapper = create_info.into();

        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), |a| &a.callbacks);

        let mut semaphore = ptr::null_mut();
        let res = unsafe {
            (self.loader().core.vkCreateSemaphore)(self.handle(), create_info.as_ref(), allocation_callbacks, &mut semaphore)
        };

        if res == vk_sys::VK_SUCCESS {
            Ok(Semaphore::new(semaphore, self.clone(), allocator_helper))
        }
        else {
            Err(res.into())
        }
    }

    pub fn create_event(&self, create_info: &core::EventCreateInfo, allocator: Option<Box<core::Allocator>>) -> Result<Event> {
        let create_info: core::VkEventCreateInfoWrapper = create_info.into();

        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), |a| &a.callbacks);

        let mut event = ptr::null_mut();
        let res = unsafe {
            (self.loader().core.vkCreateEvent)(self.handle(), create_info.as_ref(), allocation_callbacks, &mut event)
        };

        if res == vk_sys::VK_SUCCESS {
            Ok(Event::new(event, self.clone(), allocator_helper))
        }
        else {
            Err(res.into())
        }
    }

    pub fn create_query_pool(&self, create_info: &core::QueryPoolCreateInfo, allocator: Option<Box<core::Allocator>>) -> Result<QueryPool> {
        let create_info: core::VkQueryPoolCreateInfoWrapper = create_info.into();

        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), |a| &a.callbacks);

        let mut query_pool = ptr::null_mut();
        let res = unsafe {
            (self.loader().core.vkCreateQueryPool)(self.handle(), create_info.as_ref(), allocation_callbacks, &mut query_pool)
        };

        if res == vk_sys::VK_SUCCESS {
            Ok(QueryPool::new(query_pool, self.clone(), allocator_helper))
        }
        else {
            Err(res.into())
        }
    }

    pub fn create_buffer(&self, create_info: &core::BufferCreateInfo, allocator: Option<Box<core::Allocator>>) -> Result<Buffer> {
        let create_info: core::VkBufferCreateInfoWrapper = create_info.into();

        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), |a| &a.callbacks);

        let mut buffer = ptr::null_mut();
        let res = unsafe {
            (self.loader().core.vkCreateBuffer)(self.handle(), create_info.as_ref(), allocation_callbacks, &mut buffer)
        };

        if res == vk_sys::VK_SUCCESS {
            Ok(Buffer::new(buffer, self.clone(), allocator_helper))
        }
        else {
            Err(res.into())
        }
    }

    pub fn create_image(&self, create_info: &core::ImageCreateInfo, allocator: Option<Box<core::Allocator>>) -> Result<Image> {
        let create_info: core::VkImageCreateInfoWrapper = create_info.into();

        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), |a| &a.callbacks);

        let mut image = ptr::null_mut();
        let res = unsafe {
            (self.loader().core.vkCreateImage)(self.handle(), create_info.as_ref(), allocation_callbacks, &mut image)
        };

        if res == vk_sys::VK_SUCCESS {
            Ok(Image::new(image, self.clone(), allocator_helper))
        }
        else {
            Err(res.into())
        }
    }

    pub fn create_buffer_view(&self, create_info: &core::BufferViewCreateInfo, allocator: Option<Box<core::Allocator>>) -> Result<BufferView> {
        let create_info_wrapper: core::VkBufferViewCreateInfoWrapper = create_info.into();

        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), |a| &a.callbacks);

        let mut buffer_view = ptr::null_mut();
        let res = unsafe {
            (self.loader().core.vkCreateBufferView)(self.handle(), create_info_wrapper.as_ref(), allocation_callbacks, &mut buffer_view)
        };

        if res == vk_sys::VK_SUCCESS {
            Ok(BufferView::new(buffer_view, self.clone(), allocator_helper, create_info.buffer.clone()))
        }
        else {
            Err(res.into())
        }
    }

    pub fn create_image_view(&self, create_info: &core::ImageViewCreateInfo, allocator: Option<Box<core::Allocator>>) -> Result<ImageView> {
        let create_info_wrapper: core::VkImageViewCreateInfoWrapper = create_info.into();

        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), |a| &a.callbacks);

        let mut image_view = ptr::null_mut();
        let res = unsafe {
            (self.loader().core.vkCreateImageView)(self.handle(), create_info_wrapper.as_ref(), allocation_callbacks, &mut image_view)
        };

        if res == vk_sys::VK_SUCCESS {
            Ok(ImageView::new(image_view, self.clone(), allocator_helper, create_info.image.clone()))
        }
        else {
            Err(res.into())
        }
    }

    pub fn create_shader_module(&self, create_info: &core::ShaderModuleCreateInfo, allocator: Option<Box<core::Allocator>>) -> Result<ShaderModule> {
        let create_info: core::VkShaderModuleCreateInfoWrapper = create_info.into();

        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), |a| &a.callbacks);

        let mut shader_module = ptr::null_mut();
        let res = unsafe {
            (self.loader().core.vkCreateShaderModule)(self.handle(), create_info.as_ref(), allocation_callbacks, &mut shader_module)
        };

        if res == vk_sys::VK_SUCCESS {
            Ok(ShaderModule::new(shader_module, self.clone(), allocator_helper))
        }
        else {
            Err(res.into())
        }
    }

    pub fn create_pipeline_cache(&self, create_info: &core::PipelineCacheCreateInfo, allocator: Option<Box<core::Allocator>>) -> Result<PipelineCache> {
        let create_info: core::VkPipelineCacheCreateInfoWrapper = create_info.into();

        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), |a| &a.callbacks);

        let mut pipeline_cache = ptr::null_mut();
        let res = unsafe {
            (self.loader().core.vkCreatePipelineCache)(self.handle(), create_info.as_ref(), allocation_callbacks, &mut pipeline_cache)
        };

        if res == vk_sys::VK_SUCCESS {
            Ok(PipelineCache::new(pipeline_cache, self.clone(), allocator_helper))
        }
        else {
            Err(res.into())
        }
    }

    pub fn create_sampler(&self, create_info: &core::SamplerCreateInfo, allocator: Option<Box<core::Allocator>>) -> Result<Sampler> {
        let create_info: core::VkSamplerCreateInfoWrapper = create_info.into();

        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), |a| &a.callbacks);

        let mut sampler = ptr::null_mut();
        let res = unsafe {
            (self.loader().core.vkCreateSampler)(self.handle(), create_info.as_ref(), allocation_callbacks, &mut sampler)
        };

        if res == vk_sys::VK_SUCCESS {
            Ok(Sampler::new(sampler, self.clone(), allocator_helper))
        }
        else {
            Err(res.into())
        }
    }

    pub fn create_descriptor_pool(&self, create_info: &core::DescriptorPoolCreateInfo, allocator: Option<Box<core::Allocator>>) -> Result<DescriptorPool> {
        let create_info: core::VkDescriptorPoolCreateInfoWrapper = create_info.into();

        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), |a| &a.callbacks);

        let mut descriptor_pool = ptr::null_mut();
        let res = unsafe {
            (self.loader().core.vkCreateDescriptorPool)(self.handle(), create_info.as_ref(), allocation_callbacks, &mut descriptor_pool)
        };

        if res == vk_sys::VK_SUCCESS {
            Ok(DescriptorPool::new(descriptor_pool, self.clone(), allocator_helper))
        }
        else {
            Err(res.into())
        }
    }

    pub fn create_descriptor_set_layout(&self, create_info: &core::DescriptorSetLayoutCreateInfo, allocator: Option<Box<core::Allocator>>) -> Result<DescriptorSetLayout> {
        let create_info_wrapper: core::VkDescriptorSetLayoutCreateInfoWrapper = create_info.into();

        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), |a| &a.callbacks);

        let mut descriptor_set_layout = ptr::null_mut();
        let res = unsafe {
            (self.loader().core.vkCreateDescriptorSetLayout)(self.handle(), create_info_wrapper.as_ref(), allocation_callbacks, &mut descriptor_set_layout)
        };

        if res == vk_sys::VK_SUCCESS {
            let mut samplers = Vec::new();
            if let Some(ref bindings) = create_info.bindings {
                for binding in bindings {
                    if let Some(ref immutable_samplers) = binding.immutable_samplers {
                        samplers.extend_from_slice(immutable_samplers);
                    }
                }
            }

            Ok(DescriptorSetLayout::new(descriptor_set_layout, self.clone(), allocator_helper, samplers))
        }
        else {
            Err(res.into())
        }
    }
}
