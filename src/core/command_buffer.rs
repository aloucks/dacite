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

use core::{
    self,
    Buffer,
    CommandPool,
    DescriptorSet,
    Pipeline,
    PipelineLayout,
};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ptr;
use std::sync::Arc;
use vks;
use {TryDestroyError, TryDestroyErrorKind, VulkanObject};

/// See [`VkCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBuffer)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandBuffer(Arc<Inner>);

impl VulkanObject for CommandBuffer {
    type NativeVulkanObject = vks::VkCommandBuffer;

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

impl CommandBuffer {
    pub(crate) fn new(handle: vks::VkCommandBuffer, command_pool: CommandPool) -> Self {
        CommandBuffer(Arc::new(Inner {
            handle: handle,
            command_pool: command_pool,
        }))
    }

    #[inline]
    pub(crate) fn handle(&self) -> vks::VkCommandBuffer {
        self.0.handle
    }

    #[inline]
    pub(crate) fn loader(&self) -> &vks::DeviceProcAddrLoader {
        self.0.command_pool.loader()
    }

    /// See [`vkBeginCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkBeginCommandBuffer)
    pub fn begin(&self, begin_info: &core::CommandBufferBeginInfo) -> Result<(), core::Error> {
        let begin_info_wrapper: core::VkCommandBufferBeginInfoWrapper = begin_info.into();

        let res = unsafe {
            (self.loader().core.vkBeginCommandBuffer)(self.handle(), begin_info_wrapper.as_ref())
        };

        if res == vks::VK_SUCCESS {
            Ok(())
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkEndCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEndCommandBuffer)
    pub fn end(&self) -> Result<(), core::Error> {
        let res = unsafe {
            (self.loader().core.vkEndCommandBuffer)(self.handle())
        };

        if res == vks::VK_SUCCESS {
            Ok(())
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkResetCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkResetCommandBuffer)
    pub fn reset(&self, flags: core::CommandBufferResetFlags) -> Result<(), core::Error> {
        let res = unsafe {
            (self.loader().core.vkResetCommandBuffer)(self.handle(), flags)
        };

        if res == vks::VK_SUCCESS {
            Ok(())
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkCmdBindPipeline`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBindPipeline)
    pub fn bind_pipeline(&self, pipeline_bind_point: core::PipelineBindPoint, pipeline: &Pipeline) {
        unsafe {
            (self.loader().core.vkCmdBindPipeline)(self.handle(), pipeline_bind_point.into(), pipeline.handle());
        }
    }

    /// See [`vkCmdSetViewport`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetViewport)
    pub fn set_viewport(&self, first_viewport: u32, viewports: &[core::Viewport]) {
        let viewports: Vec<_> = viewports.iter().map(From::from).collect();
        unsafe {
            (self.loader().core.vkCmdSetViewport)(self.handle(), first_viewport, viewports.len() as u32, viewports.as_ptr());
        }
    }

    /// See [`vkCmdSetScissor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetScissor)
    pub fn set_scissor(&self, first_scissor: u32, scissors: &[core::Rect2D]) {
        let scissors: Vec<_> = scissors.iter().map(From::from).collect();
        unsafe {
            (self.loader().core.vkCmdSetScissor)(self.handle(), first_scissor, scissors.len() as u32, scissors.as_ptr());
        }
    }

    /// See [`vkCmdSetLineWidth`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetLineWidth)
    pub fn set_line_width(&self, line_width: f32) {
        unsafe {
            (self.loader().core.vkCmdSetLineWidth)(self.handle(), line_width);
        }
    }

    /// See [`vkCmdSetDepthBias`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetDepthBias)
    pub fn set_depth_bias(&self, depth_bias_constant_factor: f32, depth_bias_clamp: f32, depth_bias_slope_factor: f32) {
        unsafe {
            (self.loader().core.vkCmdSetDepthBias)(self.handle(), depth_bias_constant_factor, depth_bias_clamp, depth_bias_slope_factor);
        }
    }

    /// See [`vkCmdSetBlendConstants`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetBlendConstants)
    pub fn set_blend_constants(&self, blend_constants: &[f32]) {
        unsafe {
            (self.loader().core.vkCmdSetBlendConstants)(self.handle(), blend_constants.as_ptr());
        }
    }

    /// See [`vkCmdSetDepthBounds`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetDepthBounds)
    pub fn set_depth_bounds(&self, min_depth_bounds: f32, max_depth_bounds: f32) {
        unsafe {
            (self.loader().core.vkCmdSetDepthBounds)(self.handle(), min_depth_bounds, max_depth_bounds);
        }
    }

    /// See [`vkCmdSetStencilCompareMask`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetStencilCompareMask)
    pub fn set_stencil_compare_mask(&self, face_mask: core::StencilFaceFlags, compare_mask: u32) {
        unsafe {
            (self.loader().core.vkCmdSetStencilCompareMask)(self.handle(), face_mask, compare_mask);
        }
    }

    /// See [`vkCmdSetStencilWriteMask`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetStencilWriteMask)
    pub fn set_stencil_write_mask(&self, face_mask: core::StencilFaceFlags, write_mask: u32) {
        unsafe {
            (self.loader().core.vkCmdSetStencilWriteMask)(self.handle(), face_mask, write_mask);
        }
    }

    /// See [`vkCmdSetStencilReference`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetStencilReference)
    pub fn set_stencil_reference(&self, face_mask: core::StencilFaceFlags, reference: u32) {
        unsafe {
            (self.loader().core.vkCmdSetStencilReference)(self.handle(), face_mask, reference);
        }
    }

    /// See [`vkCmdBindDescriptorSets`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBindDescriptorSets)
    pub fn bind_descriptor_sets(&self, pipeline_bind_point: core::PipelineBindPoint, layout: &PipelineLayout, first_set: u32, descriptor_sets: &[DescriptorSet], dynamic_offsets: Option<&[u32]>) {
        let descriptor_sets: Vec<_> = descriptor_sets.iter().map(DescriptorSet::handle).collect();

        let (dynamic_offsets_count, dynamic_offsets_ptr) = match dynamic_offsets {
            Some(dynamic_offsets) => (dynamic_offsets.len() as u32, dynamic_offsets.as_ptr()),
            None => (0, ptr::null()),
        };

        unsafe {
            (self.loader().core.vkCmdBindDescriptorSets)(self.handle(), pipeline_bind_point.into(), layout.handle(), first_set, descriptor_sets.len() as u32, descriptor_sets.as_ptr(), dynamic_offsets_count, dynamic_offsets_ptr);
        }
    }

    /// See [`vkCmdBindIndexBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBindIndexBuffer)
    pub fn bind_index_buffer(&self, buffer: &Buffer, offset: u64, index_type: core::IndexType) {
        unsafe {
            (self.loader().core.vkCmdBindIndexBuffer)(self.handle(), buffer.handle(), offset, index_type.into());
        }
    }
}

#[derive(Debug)]
struct Inner {
    handle: vks::VkCommandBuffer,
    command_pool: CommandPool,
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            (self.command_pool.loader().core.vkFreeCommandBuffers)(self.command_pool.device_handle(), self.command_pool.handle(), 1, &self.handle);
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
