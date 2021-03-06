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
use core::{
    self,
    Buffer,
    BufferView,
    CommandPool,
    DescriptorPool,
    DescriptorSetLayout,
    DeviceMemory,
    Event,
    Fence,
    Framebuffer,
    Image,
    ImageView,
    Instance,
    Pipeline,
    PipelineCache,
    PipelineLayout,
    QueryPool,
    Queue,
    RenderPass,
    Sampler,
    Semaphore,
    ShaderModule,
};
use ext_debug_marker;
use khr_swapchain;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ptr;
use std::sync::Arc;
use vks;
use {TryDestroyError, TryDestroyErrorKind, VulkanObject};

/// See [`VkDevice`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkDevice)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Device(Arc<Inner>);

impl VulkanObject for Device {
    type NativeVulkanObject = vks::vk::VkDevice;

    #[inline]
    fn id(&self) -> u64 {
        self.as_native_vulkan_object() as u64
    }

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

impl Device {
    pub(crate) fn new(handle: vks::vk::VkDevice, instance: Instance, allocator: Option<AllocatorHelper>, loader: vks::DeviceProcAddrLoader, enabled_extensions: core::DeviceExtensions) -> Self {
        Device(Arc::new(Inner {
            handle: handle,
            instance: instance,
            allocator: allocator,
            loader: loader,
            enabled_extensions: enabled_extensions,
        }))
    }

    #[inline]
    pub(crate) fn handle(&self) -> vks::vk::VkDevice {
        self.0.handle
    }

    #[inline]
    pub(crate) fn loader(&self) -> &vks::DeviceProcAddrLoader {
        &self.0.loader
    }

    pub fn get_enabled_instance_extensions(&self) -> &core::InstanceExtensions {
        self.0.instance.get_enabled_extensions()
    }

    pub fn get_enabled_device_extensions(&self) -> &core::DeviceExtensions {
        &self.0.enabled_extensions
    }

    /// See [`vkGetDeviceQueue`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkGetDeviceQueue)
    pub fn get_queue(&self, queue_family_index: u32, queue_index: u32) -> Queue {
        let mut queue = ptr::null_mut();
        unsafe {
            self.loader().vk.vkGetDeviceQueue(self.handle(), queue_family_index, queue_index, &mut queue);
        }

        Queue::new(queue, self.clone())
    }

    /// See [`vkCreateCommandPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateCommandPool)
    pub fn create_command_pool(&self, create_info: &core::CommandPoolCreateInfo, allocator: Option<Box<core::Allocator>>) -> Result<CommandPool, core::Error> {
        let create_info = core::VkCommandPoolCreateInfoWrapper::new(create_info, true);

        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), AllocatorHelper::callbacks);

        let mut command_pool = Default::default();
        let res = unsafe {
            self.loader().vk.vkCreateCommandPool(self.handle(), &create_info.vks_struct, allocation_callbacks, &mut command_pool)
        };

        if res == vks::vk::VK_SUCCESS {
            Ok(CommandPool::new(command_pool, true, self.clone(), allocator_helper))
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkCreateFence`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateFence)
    pub fn create_fence(&self, create_info: &core::FenceCreateInfo, allocator: Option<Box<core::Allocator>>) -> Result<Fence, core::Error> {
        let create_info = core::VkFenceCreateInfoWrapper::new(create_info, true);

        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), AllocatorHelper::callbacks);

        let mut fence = Default::default();
        let res = unsafe {
            self.loader().vk.vkCreateFence(self.handle(), &create_info.vks_struct, allocation_callbacks, &mut fence)
        };

        if res == vks::vk::VK_SUCCESS {
            Ok(Fence::new(fence, true, self.clone(), allocator_helper))
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkCreateSemaphore`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateSemaphore)
    pub fn create_semaphore(&self, create_info: &core::SemaphoreCreateInfo, allocator: Option<Box<core::Allocator>>) -> Result<Semaphore, core::Error> {
        let create_info = core::VkSemaphoreCreateInfoWrapper::new(create_info, true);

        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), AllocatorHelper::callbacks);

        let mut semaphore = Default::default();
        let res = unsafe {
            self.loader().vk.vkCreateSemaphore(self.handle(), &create_info.vks_struct, allocation_callbacks, &mut semaphore)
        };

        if res == vks::vk::VK_SUCCESS {
            Ok(Semaphore::new(semaphore, true, self.clone(), allocator_helper))
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkCreateEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateEvent)
    pub fn create_event(&self, create_info: &core::EventCreateInfo, allocator: Option<Box<core::Allocator>>) -> Result<Event, core::Error> {
        let create_info = core::VkEventCreateInfoWrapper::new(create_info, true);

        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), AllocatorHelper::callbacks);

        let mut event = Default::default();
        let res = unsafe {
            self.loader().vk.vkCreateEvent(self.handle(), &create_info.vks_struct, allocation_callbacks, &mut event)
        };

        if res == vks::vk::VK_SUCCESS {
            Ok(Event::new(event, true, self.clone(), allocator_helper))
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkCreateQueryPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateQueryPool)
    pub fn create_query_pool(&self, create_info: &core::QueryPoolCreateInfo, allocator: Option<Box<core::Allocator>>) -> Result<QueryPool, core::Error> {
        let create_info = core::VkQueryPoolCreateInfoWrapper::new(create_info, true);

        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), AllocatorHelper::callbacks);

        let mut query_pool = Default::default();
        let res = unsafe {
            self.loader().vk.vkCreateQueryPool(self.handle(), &create_info.vks_struct, allocation_callbacks, &mut query_pool)
        };

        if res == vks::vk::VK_SUCCESS {
            Ok(QueryPool::new(query_pool, true, self.clone(), allocator_helper))
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkCreateBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateBuffer)
    pub fn create_buffer(&self, create_info: &core::BufferCreateInfo, allocator: Option<Box<core::Allocator>>) -> Result<Buffer, core::Error> {
        let create_info = core::VkBufferCreateInfoWrapper::new(create_info, true);

        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), AllocatorHelper::callbacks);

        let mut buffer = Default::default();
        let res = unsafe {
            self.loader().vk.vkCreateBuffer(self.handle(), &create_info.vks_struct, allocation_callbacks, &mut buffer)
        };

        if res == vks::vk::VK_SUCCESS {
            Ok(Buffer::new(buffer, true, self.clone(), allocator_helper))
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkCreateImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateImage)
    pub fn create_image(&self, create_info: &core::ImageCreateInfo, allocator: Option<Box<core::Allocator>>) -> Result<Image, core::Error> {
        let create_info = core::VkImageCreateInfoWrapper::new(create_info, true);

        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), AllocatorHelper::callbacks);

        let mut image = Default::default();
        let res = unsafe {
            self.loader().vk.vkCreateImage(self.handle(), &create_info.vks_struct, allocation_callbacks, &mut image)
        };

        if res == vks::vk::VK_SUCCESS {
            Ok(Image::new(image, true, self.clone(), allocator_helper))
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkCreateBufferView`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateBufferView)
    pub fn create_buffer_view(&self, create_info: &core::BufferViewCreateInfo, allocator: Option<Box<core::Allocator>>) -> Result<BufferView, core::Error> {
        let create_info_wrapper = core::VkBufferViewCreateInfoWrapper::new(create_info, true);

        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), AllocatorHelper::callbacks);

        let mut buffer_view = Default::default();
        let res = unsafe {
            self.loader().vk.vkCreateBufferView(self.handle(), &create_info_wrapper.vks_struct, allocation_callbacks, &mut buffer_view)
        };

        if res == vks::vk::VK_SUCCESS {
            Ok(BufferView::new(buffer_view, true, self.clone(), allocator_helper))
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkCreateImageView`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateImageView)
    pub fn create_image_view(&self, create_info: &core::ImageViewCreateInfo, allocator: Option<Box<core::Allocator>>) -> Result<ImageView, core::Error> {
        let create_info_wrapper = core::VkImageViewCreateInfoWrapper::new(create_info, true);

        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), AllocatorHelper::callbacks);

        let mut image_view = Default::default();
        let res = unsafe {
            self.loader().vk.vkCreateImageView(self.handle(), &create_info_wrapper.vks_struct, allocation_callbacks, &mut image_view)
        };

        if res == vks::vk::VK_SUCCESS {
            Ok(ImageView::new(image_view, true, self.clone(), allocator_helper))
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkCreateShaderModule`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateShaderModule)
    pub fn create_shader_module(&self, create_info: &core::ShaderModuleCreateInfo, allocator: Option<Box<core::Allocator>>) -> Result<ShaderModule, core::Error> {
        let create_info = core::VkShaderModuleCreateInfoWrapper::new(create_info, true);

        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), AllocatorHelper::callbacks);

        let mut shader_module = Default::default();
        let res = unsafe {
            self.loader().vk.vkCreateShaderModule(self.handle(), &create_info.vks_struct, allocation_callbacks, &mut shader_module)
        };

        if res == vks::vk::VK_SUCCESS {
            Ok(ShaderModule::new(shader_module, true, self.clone(), allocator_helper))
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkCreatePipelineCache`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreatePipelineCache)
    pub fn create_pipeline_cache(&self, create_info: &core::PipelineCacheCreateInfo, allocator: Option<Box<core::Allocator>>) -> Result<PipelineCache, core::Error> {
        let create_info = core::VkPipelineCacheCreateInfoWrapper::new(create_info, true);

        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), AllocatorHelper::callbacks);

        let mut pipeline_cache = Default::default();
        let res = unsafe {
            self.loader().vk.vkCreatePipelineCache(self.handle(), &create_info.vks_struct, allocation_callbacks, &mut pipeline_cache)
        };

        if res == vks::vk::VK_SUCCESS {
            Ok(PipelineCache::new(pipeline_cache, true, self.clone(), allocator_helper))
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkCreateSampler`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateSampler)
    pub fn create_sampler(&self, create_info: &core::SamplerCreateInfo, allocator: Option<Box<core::Allocator>>) -> Result<Sampler, core::Error> {
        let create_info = core::VkSamplerCreateInfoWrapper::new(create_info, true);

        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), AllocatorHelper::callbacks);

        let mut sampler = Default::default();
        let res = unsafe {
            self.loader().vk.vkCreateSampler(self.handle(), &create_info.vks_struct, allocation_callbacks, &mut sampler)
        };

        if res == vks::vk::VK_SUCCESS {
            Ok(Sampler::new(sampler, true, self.clone(), allocator_helper))
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkCreateDescriptorPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDescriptorPool)
    pub fn create_descriptor_pool(&self, create_info: &core::DescriptorPoolCreateInfo, allocator: Option<Box<core::Allocator>>) -> Result<DescriptorPool, core::Error> {
        let create_info = core::VkDescriptorPoolCreateInfoWrapper::new(create_info, true);

        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), AllocatorHelper::callbacks);

        let mut descriptor_pool = Default::default();
        let res = unsafe {
            self.loader().vk.vkCreateDescriptorPool(self.handle(), &create_info.vks_struct, allocation_callbacks, &mut descriptor_pool)
        };

        if res == vks::vk::VK_SUCCESS {
            Ok(DescriptorPool::new(descriptor_pool, true, self.clone(), allocator_helper))
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkCreateDescriptorSetLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateDescriptorSetLayout)
    pub fn create_descriptor_set_layout(&self, create_info: &core::DescriptorSetLayoutCreateInfo, allocator: Option<Box<core::Allocator>>) -> Result<DescriptorSetLayout, core::Error> {
        let create_info_wrapper = core::VkDescriptorSetLayoutCreateInfoWrapper::new(create_info, true);

        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), AllocatorHelper::callbacks);

        let mut descriptor_set_layout = Default::default();
        let res = unsafe {
            self.loader().vk.vkCreateDescriptorSetLayout(self.handle(), &create_info_wrapper.vks_struct, allocation_callbacks, &mut descriptor_set_layout)
        };

        if res == vks::vk::VK_SUCCESS {
            Ok(DescriptorSetLayout::new(descriptor_set_layout, true, self.clone(), allocator_helper))
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkAllocateMemory`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkAllocateMemory)
    pub fn allocate_memory(&self, allocate_info: &core::MemoryAllocateInfo, allocator: Option<Box<core::Allocator>>) -> Result<DeviceMemory, core::Error> {
        let allocate_info_wrapper = core::VkMemoryAllocateInfoWrapper::new(allocate_info, true);

        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), AllocatorHelper::callbacks);

        let mut memory = Default::default();
        let res = unsafe {
            self.loader().vk.vkAllocateMemory(self.handle(), &allocate_info_wrapper.vks_struct, allocation_callbacks, &mut memory)
        };

        if res == vks::vk::VK_SUCCESS {
            Ok(DeviceMemory::new(memory, true, self.clone(), allocator_helper, allocate_info.allocation_size))
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkCreateGraphicsPipelines`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateGraphicsPipelines)
    pub fn create_graphics_pipelines(&self, pipeline_cache: Option<PipelineCache>, create_infos: &[core::GraphicsPipelineCreateInfo], allocator: Option<Box<core::Allocator>>) -> Result<Vec<Pipeline>, (core::Error, Vec<Option<Pipeline>>)> {
        let pipeline_cache_handle = match pipeline_cache {
            Some(ref pipeline_cache) => pipeline_cache.handle(),
            None => Default::default(),
        };

        let create_info_wrappers: Vec<_> = create_infos.iter().map(|c| core::VkGraphicsPipelineCreateInfoWrapper::new(c, true)).collect();
        let vk_create_infos: Vec<vks::vk::VkGraphicsPipelineCreateInfo> = create_info_wrappers.iter().map(|c| c.vks_struct).collect();

        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), AllocatorHelper::callbacks);

        let mut pipelines = Vec::with_capacity(create_infos.len());
        let res = unsafe {
            pipelines.set_len(create_infos.len());
            self.loader().vk.vkCreateGraphicsPipelines(self.handle(), pipeline_cache_handle, create_infos.len() as u32, vk_create_infos.as_ptr(), allocation_callbacks, pipelines.as_mut_ptr())
        };

        if res == vks::vk::VK_SUCCESS {
            Ok(pipelines.iter().map(|p| Pipeline::new(*p, true, self.clone(), allocator_helper.clone())).collect())
        }
        else {
            let pipelines = pipelines.iter().map(|&p| {
                if p != 0 {
                    Some(Pipeline::new(p, true, self.clone(), allocator_helper.clone()))
                }
                else {
                    None
                }
            }).collect();
            Err((res.into(), pipelines))
        }
    }

    /// See [`vkCreateComputePipelines`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateComputePipelines)
    pub fn create_compute_pipelines(&self, pipeline_cache: Option<PipelineCache>, create_infos: &[core::ComputePipelineCreateInfo], allocator: Option<Box<core::Allocator>>) -> Result<Vec<Pipeline>, (core::Error, Vec<Option<Pipeline>>)> {
        let pipeline_cache_handle = match pipeline_cache {
            Some(ref pipeline_cache) => pipeline_cache.handle(),
            None => Default::default(),
        };

        let create_info_wrappers: Vec<_> = create_infos.iter().map(|c| core::VkComputePipelineCreateInfoWrapper::new(c, true)).collect();
        let vk_create_infos: Vec<vks::vk::VkComputePipelineCreateInfo> = create_info_wrappers.iter().map(|c| c.vks_struct).collect();

        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), AllocatorHelper::callbacks);

        let mut pipelines = Vec::with_capacity(create_infos.len());
        let res = unsafe {
            pipelines.set_len(create_infos.len());
            self.loader().vk.vkCreateComputePipelines(self.handle(), pipeline_cache_handle, create_infos.len() as u32, vk_create_infos.as_ptr(), allocation_callbacks, pipelines.as_mut_ptr())
        };

        if res == vks::vk::VK_SUCCESS {
            Ok(pipelines.iter().map(|p| Pipeline::new(*p, true, self.clone(), allocator_helper.clone())).collect())
        }
        else {
            let pipelines = pipelines.iter().map(|&p| {
                if p != 0 {
                    Some(Pipeline::new(p, true, self.clone(), allocator_helper.clone()))
                }
                else {
                    None
                }
            }).collect();
            Err((res.into(), pipelines))
        }
    }

    /// See [`vkCreatePipelineLayout`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreatePipelineLayout)
    pub fn create_pipeline_layout(&self, create_info: &core::PipelineLayoutCreateInfo, allocator: Option<Box<core::Allocator>>) -> Result<PipelineLayout, core::Error> {
        let create_info_wrapper = core::VkPipelineLayoutCreateInfoWrapper::new(create_info, true);

        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), AllocatorHelper::callbacks);

        let mut pipeline_layout = Default::default();
        let res = unsafe {
            self.loader().vk.vkCreatePipelineLayout(self.handle(), &create_info_wrapper.vks_struct, allocation_callbacks, &mut pipeline_layout)
        };

        if res == vks::vk::VK_SUCCESS {
            Ok(PipelineLayout::new(pipeline_layout, true, self.clone(), allocator_helper))
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkCreateFramebuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateFramebuffer)
    pub fn create_framebuffer(&self, create_info: &core::FramebufferCreateInfo, allocator: Option<Box<core::Allocator>>) -> Result<Framebuffer, core::Error> {
        let create_info_wrapper = core::VkFramebufferCreateInfoWrapper::new(create_info, true);

        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), AllocatorHelper::callbacks);

        let mut framebuffer = Default::default();
        let res = unsafe {
            self.loader().vk.vkCreateFramebuffer(self.handle(), &create_info_wrapper.vks_struct, allocation_callbacks, &mut framebuffer)
        };

        if res == vks::vk::VK_SUCCESS {
            Ok(Framebuffer::new(framebuffer, true, self.clone(), allocator_helper))
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkCreateRenderPass`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateRenderPass)
    pub fn create_render_pass(&self, create_info: &core::RenderPassCreateInfo, allocator: Option<Box<core::Allocator>>) -> Result<RenderPass, core::Error> {
        let create_info_wrapper = core::VkRenderPassCreateInfoWrapper::new(create_info, true);

        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), AllocatorHelper::callbacks);

        let mut render_pass = Default::default();
        let res = unsafe {
            self.loader().vk.vkCreateRenderPass(self.handle(), &create_info_wrapper.vks_struct, allocation_callbacks, &mut render_pass)
        };

        if res == vks::vk::VK_SUCCESS {
            Ok(RenderPass::new(render_pass, true, self.clone(), allocator_helper))
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkDeviceWaitIdle`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDeviceWaitIdle)
    pub fn wait_idle(&self) -> Result<(), core::Error> {
        let res = unsafe {
            self.loader().vk.vkDeviceWaitIdle(self.handle())
        };

        if res == vks::vk::VK_SUCCESS {
            Ok(())
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkCreateSwapchainKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateSwapchainKHR)
    /// and extension [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)
    pub fn create_swapchain_khr(&self, create_info: &khr_swapchain::SwapchainCreateInfoKhr, allocator: Option<Box<core::Allocator>>) -> Result<khr_swapchain::SwapchainKhr, core::Error> {
        let create_info_wrapper = khr_swapchain::VkSwapchainCreateInfoKHRWrapper::new(create_info, true);
        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), AllocatorHelper::callbacks);

        let mut swapchain = Default::default();
        let res = unsafe {
            self.loader().khr_swapchain.vkCreateSwapchainKHR(self.handle(), &create_info_wrapper.vks_struct, allocation_callbacks, &mut swapchain)
        };

        if res == vks::vk::VK_SUCCESS {
            Ok(khr_swapchain::SwapchainKhr::new(swapchain, true, self.clone(), allocator_helper))
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkCreateSharedSwapchainsKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCreateSharedSwapchainsKHR)
    /// and extension [`VK_KHR_display_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_display_swapchain)
    pub fn create_shared_swapchains_khr(&self, create_infos: &[khr_swapchain::SwapchainCreateInfoKhr], allocator: Option<Box<core::Allocator>>) -> Result<Vec<khr_swapchain::SwapchainKhr>, core::Error> {
        let create_info_wrappers: Vec<_> = create_infos.iter().map(|c| khr_swapchain::VkSwapchainCreateInfoKHRWrapper::new(c, true)).collect();
        let vk_create_infos: Vec<_> = create_info_wrappers.iter().map(|c| c.vks_struct).collect();
        let allocator_helper = allocator.map(AllocatorHelper::new);
        let allocation_callbacks = allocator_helper.as_ref().map_or(ptr::null(), AllocatorHelper::callbacks);

        let mut swapchains = Vec::with_capacity(create_infos.len());
        let res = unsafe {
            swapchains.set_len(create_infos.len());
            self.loader().khr_display_swapchain.vkCreateSharedSwapchainsKHR(self.handle(), create_infos.len() as u32, vk_create_infos.as_ptr(), allocation_callbacks, swapchains.as_mut_ptr())
        };

        if res == vks::vk::VK_SUCCESS {
            Ok(swapchains.iter().map(|&s| khr_swapchain::SwapchainKhr::new(s, true, self.clone(), allocator_helper.clone())).collect())
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkDebugMarkerSetObjectTagEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDebugMarkerSetObjectTagEXT)
    /// and extension [`VK_EXT_debug_marker`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_marker)
    pub fn debug_marker_set_object_tag_ext(&self, tag_info: &ext_debug_marker::DebugMarkerObjectTagInfoExt) -> Result<(), core::Error> {
        let wrapper = ext_debug_marker::VkDebugMarkerObjectTagInfoEXTWrapper::new(tag_info, true);

        let res = unsafe { self.loader().ext_debug_marker.vkDebugMarkerSetObjectTagEXT(self.handle(), &wrapper.vks_struct as *const _ as _) };
        if res == vks::vk::VK_SUCCESS {
            Ok(())
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkDebugMarkerSetObjectNameEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkDebugMarkerSetObjectNameEXT)
    /// and extension [`VK_EXT_debug_marker`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_marker)
    pub fn debug_marker_set_object_name_ext(&self, name_info: &ext_debug_marker::DebugMarkerObjectNameInfoExt) -> Result<(), core::Error> {
        let wrapper = ext_debug_marker::VkDebugMarkerObjectNameInfoEXTWrapper::new(name_info, true);

        let res = unsafe { self.loader().ext_debug_marker.vkDebugMarkerSetObjectNameEXT(self.handle(), &wrapper.vks_struct as *const _ as _) };
        if res == vks::vk::VK_SUCCESS {
            Ok(())
        }
        else {
            Err(res.into())
        }
    }
}

#[derive(Debug)]
struct Inner {
    handle: vks::vk::VkDevice,
    instance: Instance,
    allocator: Option<AllocatorHelper>,
    loader: vks::DeviceProcAddrLoader,
    enabled_extensions: core::DeviceExtensions,
}

impl Drop for Inner {
    fn drop(&mut self) {
        let allocator = match self.allocator {
            Some(ref allocator) => allocator.callbacks(),
            None => ptr::null(),
        };

        unsafe {
            self.loader.vk.vkDestroyDevice(self.handle, allocator);
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
