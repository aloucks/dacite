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

use FromNativeObject;
use TryDestroyError;
use TryDestroyErrorKind;
use VulkanObject;
use core::{
    self,
    Buffer,
    CommandPool,
    DescriptorSet,
    Event,
    Image,
    Pipeline,
    PipelineLayout,
    QueryPool,
};
use ext_debug_marker;
use libc::c_void;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ptr;
use std::sync::Arc;
use vks;

/// See [`VkCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkCommandBuffer)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandBuffer(Arc<Inner>);

impl VulkanObject for CommandBuffer {
    type NativeVulkanObject = vks::vk::VkCommandBuffer;

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

pub struct FromNativeCommandBufferParameters {
    /// `true`, if this `CommandBuffer` should freed the underlying Vulkan object, when it is dropped.
    pub owned: bool,

    /// The `CommandPool`, from which this `CommandBuffer` was allocated.
    pub command_pool: CommandPool,
}

impl FromNativeCommandBufferParameters {
    #[inline]
    pub fn new(owned: bool, command_pool: CommandPool) -> Self {
        FromNativeCommandBufferParameters {
            owned: owned,
            command_pool: command_pool,
        }
    }
}

impl FromNativeObject for CommandBuffer {
    type Parameters = FromNativeCommandBufferParameters;

    unsafe fn from_native_object(object: Self::NativeVulkanObject, params: Self::Parameters) -> Self {
        CommandBuffer::new(object, params.owned, params.command_pool)
    }
}

impl CommandBuffer {
    pub(crate) fn new(handle: vks::vk::VkCommandBuffer, owned: bool, command_pool: CommandPool) -> Self {
        CommandBuffer(Arc::new(Inner {
            handle: handle,
            owned: owned,
            command_pool: command_pool,
        }))
    }

    #[inline]
    pub(crate) fn handle(&self) -> vks::vk::VkCommandBuffer {
        self.0.handle
    }

    #[inline]
    pub(crate) fn loader(&self) -> &vks::DeviceProcAddrLoader {
        self.0.command_pool.loader()
    }

    /// See [`vkBeginCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkBeginCommandBuffer)
    pub fn begin(&self, begin_info: &core::CommandBufferBeginInfo) -> Result<(), core::Error> {
        let begin_info_wrapper = core::VkCommandBufferBeginInfoWrapper::new(begin_info, true);

        let res = unsafe {
            self.loader().vk.vkBeginCommandBuffer(self.handle(), &begin_info_wrapper.vks_struct)
        };

        if res == vks::vk::VK_SUCCESS {
            Ok(())
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkEndCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkEndCommandBuffer)
    pub fn end(&self) -> Result<(), core::Error> {
        let res = unsafe {
            self.loader().vk.vkEndCommandBuffer(self.handle())
        };

        if res == vks::vk::VK_SUCCESS {
            Ok(())
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkResetCommandBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkResetCommandBuffer)
    pub fn reset(&self, flags: core::CommandBufferResetFlags) -> Result<(), core::Error> {
        let res = unsafe {
            self.loader().vk.vkResetCommandBuffer(self.handle(), flags.bits())
        };

        if res == vks::vk::VK_SUCCESS {
            Ok(())
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkCmdBindPipeline`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBindPipeline)
    pub fn bind_pipeline(&self, pipeline_bind_point: core::PipelineBindPoint, pipeline: &Pipeline) {
        unsafe {
            self.loader().vk.vkCmdBindPipeline(self.handle(), pipeline_bind_point.into(), pipeline.handle());
        }
    }

    /// See [`vkCmdSetViewport`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetViewport)
    pub fn set_viewport(&self, first_viewport: u32, viewports: &[core::Viewport]) {
        let viewports: Vec<_> = viewports.iter().map(From::from).collect();
        unsafe {
            self.loader().vk.vkCmdSetViewport(self.handle(), first_viewport, viewports.len() as u32, viewports.as_ptr());
        }
    }

    /// See [`vkCmdSetScissor`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetScissor)
    pub fn set_scissor(&self, first_scissor: u32, scissors: &[core::Rect2D]) {
        let scissors: Vec<_> = scissors.iter().map(From::from).collect();
        unsafe {
            self.loader().vk.vkCmdSetScissor(self.handle(), first_scissor, scissors.len() as u32, scissors.as_ptr());
        }
    }

    /// See [`vkCmdSetLineWidth`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetLineWidth)
    pub fn set_line_width(&self, line_width: f32) {
        unsafe {
            self.loader().vk.vkCmdSetLineWidth(self.handle(), line_width);
        }
    }

    /// See [`vkCmdSetDepthBias`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetDepthBias)
    pub fn set_depth_bias(&self, depth_bias_constant_factor: f32, depth_bias_clamp: f32, depth_bias_slope_factor: f32) {
        unsafe {
            self.loader().vk.vkCmdSetDepthBias(self.handle(), depth_bias_constant_factor, depth_bias_clamp, depth_bias_slope_factor);
        }
    }

    /// See [`vkCmdSetBlendConstants`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetBlendConstants)
    pub fn set_blend_constants(&self, blend_constants: &[f32]) {
        unsafe {
            self.loader().vk.vkCmdSetBlendConstants(self.handle(), blend_constants.as_ptr());
        }
    }

    /// See [`vkCmdSetDepthBounds`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetDepthBounds)
    pub fn set_depth_bounds(&self, min_depth_bounds: f32, max_depth_bounds: f32) {
        unsafe {
            self.loader().vk.vkCmdSetDepthBounds(self.handle(), min_depth_bounds, max_depth_bounds);
        }
    }

    /// See [`vkCmdSetStencilCompareMask`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetStencilCompareMask)
    pub fn set_stencil_compare_mask(&self, face_mask: core::StencilFaceFlags, compare_mask: u32) {
        unsafe {
            self.loader().vk.vkCmdSetStencilCompareMask(self.handle(), face_mask.bits(), compare_mask);
        }
    }

    /// See [`vkCmdSetStencilWriteMask`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetStencilWriteMask)
    pub fn set_stencil_write_mask(&self, face_mask: core::StencilFaceFlags, write_mask: u32) {
        unsafe {
            self.loader().vk.vkCmdSetStencilWriteMask(self.handle(), face_mask.bits(), write_mask);
        }
    }

    /// See [`vkCmdSetStencilReference`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetStencilReference)
    pub fn set_stencil_reference(&self, face_mask: core::StencilFaceFlags, reference: u32) {
        unsafe {
            self.loader().vk.vkCmdSetStencilReference(self.handle(), face_mask.bits(), reference);
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
            self.loader().vk.vkCmdBindDescriptorSets(self.handle(), pipeline_bind_point.into(), layout.handle(), first_set, descriptor_sets.len() as u32, descriptor_sets.as_ptr(), dynamic_offsets_count, dynamic_offsets_ptr);
        }
    }

    /// See [`vkCmdBindIndexBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBindIndexBuffer)
    pub fn bind_index_buffer(&self, buffer: &Buffer, offset: u64, index_type: core::IndexType) {
        unsafe {
            self.loader().vk.vkCmdBindIndexBuffer(self.handle(), buffer.handle(), offset, index_type.into());
        }
    }

    /// See [`vkCmdBindVertexBuffers`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBindVertexBuffers)
    pub fn bind_vertex_buffers(&self, first_binding: u32, buffers: &[Buffer], offsets: &[u64]) {
        let buffers: Vec<_> = buffers.iter().map(Buffer::handle).collect();
        unsafe {
            self.loader().vk.vkCmdBindVertexBuffers(self.handle(), first_binding, buffers.len() as u32, buffers.as_ptr(), offsets.as_ptr());
        }
    }

    /// See [`vkCmdDraw`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDraw)
    pub fn draw(&self, vertex_count: u32, instance_count: u32, first_vertex: u32, first_instance: u32) {
        unsafe {
            self.loader().vk.vkCmdDraw(self.handle(), vertex_count, instance_count, first_vertex, first_instance);
        }
    }

    /// See [`vkCmdDrawIndexed`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDrawIndexed)
    pub fn draw_indexed(&self, index_count: u32, instance_count: u32, first_index: u32, vertex_offset: i32, first_instance: u32) {
        unsafe {
            self.loader().vk.vkCmdDrawIndexed(self.handle(), index_count, instance_count, first_index, vertex_offset, first_instance);
        }
    }

    /// See [`vkCmdDrawIndirect`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDrawIndirect)
    pub fn draw_indirect(&self, buffer: &Buffer, offset: u64, draw_count: u32, stride: u32) {
        unsafe {
            self.loader().vk.vkCmdDrawIndirect(self.handle(), buffer.handle(), offset, draw_count, stride);
        }
    }

    /// See [`vkCmdDrawIndexedIndirect`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDrawIndexedIndirect)
    pub fn draw_indexed_indirect(&self, buffer: &Buffer, offset: u64, draw_count: u32, stride: u32) {
        unsafe {
            self.loader().vk.vkCmdDrawIndexedIndirect(self.handle(), buffer.handle(), offset, draw_count, stride);
        }
    }

    /// See [`vkCmdDispatch`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDispatch)
    pub fn dispatch(&self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        unsafe {
            self.loader().vk.vkCmdDispatch(self.handle(), group_count_x, group_count_y, group_count_z);
        }
    }

    /// See [`vkCmdDispatchIndirect`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDispatchIndirect)
    pub fn dispatch_indirect(&self, buffer: &Buffer, offset: u64) {
        unsafe {
            self.loader().vk.vkCmdDispatchIndirect(self.handle(), buffer.handle(), offset);
        }
    }

    /// See [`vkCmdCopyBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdCopyBuffer)
    pub fn copy_buffer(&self, src_buffer: &Buffer, dst_buffer: &Buffer, regions: &[core::BufferCopy]) {
        let regions: Vec<_> = regions.iter().map(From::from).collect();
        unsafe {
            self.loader().vk.vkCmdCopyBuffer(self.handle(), src_buffer.handle(), dst_buffer.handle(), regions.len() as u32, regions.as_ptr());
        }
    }

    /// See [`vkCmdCopyImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdCopyImage)
    pub fn copy_image(&self, src_image: &Image, src_image_layout: core::ImageLayout, dst_image: &Image, dst_image_layout: core::ImageLayout, regions: &[core::ImageCopy]) {
        let regions: Vec<_> = regions.iter().map(From::from).collect();
        unsafe {
            self.loader().vk.vkCmdCopyImage(self.handle(), src_image.handle(), src_image_layout.into(), dst_image.handle(), dst_image_layout.into(), regions.len() as u32, regions.as_ptr());
        }
    }

    /// See [`vkCmdBlitImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBlitImage)
    pub fn blit_image(&self, src_image: &Image, src_image_layout: core::ImageLayout, dst_image: &Image, dst_image_layout: core::ImageLayout, regions: &[core::ImageBlit], filter: core::Filter) {
        let regions: Vec<_> = regions.iter().map(From::from).collect();
        unsafe {
            self.loader().vk.vkCmdBlitImage(self.handle(), src_image.handle(), src_image_layout.into(), dst_image.handle(), dst_image_layout.into(), regions.len() as u32, regions.as_ptr(), filter.into());
        }
    }

    /// See [`vkCmdCopyBufferToImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdCopyBufferToImage)
    pub fn copy_buffer_to_image(&self, src_buffer: &Buffer, dst_image: &Image, dst_image_layout: core::ImageLayout, regions: &[core::BufferImageCopy]) {
        let regions: Vec<_> = regions.iter().map(From::from).collect();
        unsafe {
            self.loader().vk.vkCmdCopyBufferToImage(self.handle(), src_buffer.handle(), dst_image.handle(), dst_image_layout.into(), regions.len() as u32, regions.as_ptr());
        }
    }

    /// See [`vkCmdCopyImageToBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdCopyImageToBuffer)
    pub fn copy_image_to_buffer(&self, src_image: &Image, src_image_layout: core::ImageLayout, dst_buffer: &Buffer, regions: &[core::BufferImageCopy]) {
        let regions: Vec<_> = regions.iter().map(From::from).collect();
        unsafe {
            self.loader().vk.vkCmdCopyImageToBuffer(self.handle(), src_image.handle(), src_image_layout.into(), dst_buffer.handle(), regions.len() as u32, regions.as_ptr());
        }
    }

    /// See [`vkCmdUpdateBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdUpdateBuffer)
    pub fn update_buffer(&self, dst_buffer: &Buffer, dst_offset: u64, data: &[u8]) {
        unsafe {
            self.loader().vk.vkCmdUpdateBuffer(self.handle(), dst_buffer.handle(), dst_offset, data.len() as u64, data.as_ptr() as *const _);
        }
    }

    /// See [`vkCmdFillBuffer`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdFillBuffer)
    pub fn fill_buffer(&self, dst_buffer: &Buffer, dst_offset: u64, size: core::OptionalDeviceSize, data: u32) {
        unsafe {
            self.loader().vk.vkCmdFillBuffer(self.handle(), dst_buffer.handle(), dst_offset, size.into(), data);
        }
    }

    /// See [`vkCmdClearColorImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdClearColorImage)
    pub fn clear_color_image(&self, image: &Image, image_layout: core::ImageLayout, color: &core::ClearColorValue, ranges: &[core::ImageSubresourceRange]) {
        let color = color.into();
        let ranges: Vec<_> = ranges.iter().map(From::from).collect();
        unsafe {
            self.loader().vk.vkCmdClearColorImage(self.handle(), image.handle(), image_layout.into(), &color, ranges.len() as u32, ranges.as_ptr());
        }
    }

    /// See [`vkCmdClearDepthStencilImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdClearDepthStencilImage)
    pub fn clear_depth_stencil_image(&self, image: &Image, image_layout: core::ImageLayout, depth_stencil: &core::ClearDepthStencilValue, ranges: &[core::ImageSubresourceRange]) {
        let depth_stencil = depth_stencil.into();
        let ranges: Vec<_> = ranges.iter().map(From::from).collect();
        unsafe {
            self.loader().vk.vkCmdClearDepthStencilImage(self.handle(), image.handle(), image_layout.into(), &depth_stencil, ranges.len() as u32, ranges.as_ptr());
        }
    }

    /// See [`vkCmdClearAttachments`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdClearAttachments)
    pub fn clear_attachments(&self, attachments: &[core::ClearAttachment], rects: &[core::ClearRect]) {
        let attachments: Vec<_> = attachments.iter().map(From::from).collect();
        let rects: Vec<_> = rects.iter().map(From::from).collect();
        unsafe {
            self.loader().vk.vkCmdClearAttachments(self.handle(), attachments.len() as u32, attachments.as_ptr(), rects.len() as u32, rects.as_ptr());
        }
    }

    /// See [`vkCmdResolveImage`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdResolveImage)
    pub fn resolve_image(&self, src_image: &Image, src_image_layout: core::ImageLayout, dst_image: &Image, dst_image_layout: core::ImageLayout, regions: &[core::ImageResolve]) {
        let regions: Vec<_> = regions.iter().map(From::from).collect();
        unsafe {
            self.loader().vk.vkCmdResolveImage(self.handle(), src_image.handle(), src_image_layout.into(), dst_image.handle(), dst_image_layout.into(), regions.len() as u32, regions.as_ptr());
        }
    }

    /// See [`vkCmdSetEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdSetEvent)
    pub fn set_event(&self, event: &Event, stage_mask: core::PipelineStageFlags) {
        unsafe {
            self.loader().vk.vkCmdSetEvent(self.handle(), event.handle(), stage_mask.bits());
        }
    }

    /// See [`vkCmdResetEvent`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdResetEvent)
    pub fn reset_event(&self, event: &Event, stage_mask: core::PipelineStageFlags) {
        unsafe {
            self.loader().vk.vkCmdResetEvent(self.handle(), event.handle(), stage_mask.bits());
        }
    }

    /// See [`vkCmdWaitEvents`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdWaitEvents)
    pub fn wait_events(&self, events: &[Event], src_stage_mask: core::PipelineStageFlags, dst_stage_mask: core::PipelineStageFlags, memory_barriers: Option<&[core::MemoryBarrier]>, buffer_memory_barriers: Option<&[core::BufferMemoryBarrier]>, image_memory_barriers: Option<&[core::ImageMemoryBarrier]>) {
        let events: Vec<_> = events.iter().map(Event::handle).collect();

        #[allow(unused_variables)]
        let (memory_barriers_count, memory_barriers_ptr, vk_memory_barriers, memory_barriers_wrappers) = match memory_barriers {
            Some(memory_barriers) => {
                let memory_barriers_wrappers: Vec<_> = memory_barriers.iter().map(|m| core::VkMemoryBarrierWrapper::new(m, true)).collect();
                let vk_memory_barriers: Vec<_> = memory_barriers_wrappers.iter().map(|m| m.vks_struct).collect();
                (memory_barriers.len() as u32, vk_memory_barriers.as_ptr(), Some(vk_memory_barriers), Some(memory_barriers_wrappers))
            }

            None => (0, ptr::null(), None, None),
        };

        #[allow(unused_variables)]
        let (buffer_memory_barriers_count, buffer_memory_barriers_ptr, vk_buffer_memory_barriers, buffer_memory_barriers_wrappers) = match buffer_memory_barriers {
            Some(buffer_memory_barriers) => {
                let buffer_memory_barriers_wrappers: Vec<_> = buffer_memory_barriers.iter().map(|b| core::VkBufferMemoryBarrierWrapper::new(b, true)).collect();
                let vk_buffer_memory_barriers: Vec<_> = buffer_memory_barriers_wrappers.iter().map(|b| b.vks_struct).collect();
                (buffer_memory_barriers.len() as u32, vk_buffer_memory_barriers.as_ptr(), Some(vk_buffer_memory_barriers), Some(buffer_memory_barriers_wrappers))
            }

            None => (0, ptr::null(), None, None),
        };

        #[allow(unused_variables)]
        let (image_memory_barriers_count, image_memory_barriers_ptr, vk_image_memory_barriers, image_memory_barriers_wrappers) = match image_memory_barriers {
            Some(image_memory_barriers) => {
                let image_memory_barriers_wrappers: Vec<_> = image_memory_barriers.iter().map(|i| core::VkImageMemoryBarrierWrapper::new(i, true)).collect();
                let vk_image_memory_barriers: Vec<_> = image_memory_barriers_wrappers.iter().map(|i| i.vks_struct).collect();
                (image_memory_barriers.len() as u32, vk_image_memory_barriers.as_ptr(), Some(vk_image_memory_barriers), Some(image_memory_barriers_wrappers))
            }

            None => (0, ptr::null(), None, None),
        };

        unsafe {
            self.loader().vk.vkCmdWaitEvents(self.handle(), events.len() as u32, events.as_ptr(), src_stage_mask.bits(), dst_stage_mask.bits(), memory_barriers_count, memory_barriers_ptr, buffer_memory_barriers_count, buffer_memory_barriers_ptr, image_memory_barriers_count, image_memory_barriers_ptr);
        }
    }

    /// See [`vkCmdPipelineBarrier`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdPipelineBarrier)
    pub fn pipeline_barrier(&self, src_stage_mask: core::PipelineStageFlags, dst_stage_mask: core::PipelineStageFlags, dependency_flags: core::DependencyFlags, memory_barriers: Option<&[core::MemoryBarrier]>, buffer_memory_barriers: Option<&[core::BufferMemoryBarrier]>, image_memory_barriers: Option<&[core::ImageMemoryBarrier]>) {
        #[allow(unused_variables)]
        let (memory_barriers_count, memory_barriers_ptr, vk_memory_barriers, memory_barriers_wrappers) = match memory_barriers {
            Some(memory_barriers) => {
                let memory_barriers_wrappers: Vec<_> = memory_barriers.iter().map(|m| core::VkMemoryBarrierWrapper::new(m, true)).collect();
                let vk_memory_barriers: Vec<_> = memory_barriers_wrappers.iter().map(|m| m.vks_struct).collect();
                (memory_barriers.len() as u32, vk_memory_barriers.as_ptr(), Some(vk_memory_barriers), Some(memory_barriers_wrappers))
            }

            None => (0, ptr::null(), None, None),
        };

        #[allow(unused_variables)]
        let (buffer_memory_barriers_count, buffer_memory_barriers_ptr, vk_buffer_memory_barriers, buffer_memory_barriers_wrappers) = match buffer_memory_barriers {
            Some(buffer_memory_barriers) => {
                let buffer_memory_barriers_wrappers: Vec<_> = buffer_memory_barriers.iter().map(|b| core::VkBufferMemoryBarrierWrapper::new(b, true)).collect();
                let vk_buffer_memory_barriers: Vec<_> = buffer_memory_barriers_wrappers.iter().map(|b| b.vks_struct).collect();
                (buffer_memory_barriers.len() as u32, vk_buffer_memory_barriers.as_ptr(), Some(vk_buffer_memory_barriers), Some(buffer_memory_barriers_wrappers))
            }

            None => (0, ptr::null(), None, None),
        };

        #[allow(unused_variables)]
        let (image_memory_barriers_count, image_memory_barriers_ptr, vk_image_memory_barriers, image_memory_barriers_wrappers) = match image_memory_barriers {
            Some(image_memory_barriers) => {
                let image_memory_barriers_wrappers: Vec<_> = image_memory_barriers.iter().map(|i| core::VkImageMemoryBarrierWrapper::new(i, true)).collect();
                let vk_image_memory_barriers: Vec<_> = image_memory_barriers_wrappers.iter().map(|i| i.vks_struct).collect();
                (image_memory_barriers.len() as u32, vk_image_memory_barriers.as_ptr(), Some(vk_image_memory_barriers), Some(image_memory_barriers_wrappers))
            }

            None => (0, ptr::null(), None, None),
        };

        unsafe {
            self.loader().vk.vkCmdPipelineBarrier(self.handle(), src_stage_mask.bits(), dst_stage_mask.bits(), dependency_flags.bits(), memory_barriers_count, memory_barriers_ptr, buffer_memory_barriers_count, buffer_memory_barriers_ptr, image_memory_barriers_count, image_memory_barriers_ptr);
        }
    }

    /// See [`vkCmdBeginQuery`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBeginQuery)
    pub fn begin_query(&self, query_pool: &QueryPool, query: u32, flags: core::QueryControlFlags) {
        unsafe {
            self.loader().vk.vkCmdBeginQuery(self.handle(), query_pool.handle(), query, flags.bits());
        }
    }

    /// See [`vkCmdEndQuery`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdEndQuery)
    pub fn end_query(&self, query_pool: &QueryPool, query: u32) {
        unsafe {
            self.loader().vk.vkCmdEndQuery(self.handle(), query_pool.handle(), query);
        }
    }

    /// See [`vkCmdResetQueryPool`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdResetQueryPool)
    pub fn reset_query_pool(&self, query_pool: &QueryPool, first_query: u32, query_count: u32) {
        unsafe {
            self.loader().vk.vkCmdResetQueryPool(self.handle(), query_pool.handle(), first_query, query_count);
        }
    }

    /// See [`vkCmdWriteTimestamp`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdWriteTimestamp)
    pub fn write_timestamp(&self, pipeline_stage: core::PipelineStageFlagBits, query_pool: &QueryPool, query: u32) {
        unsafe {
            self.loader().vk.vkCmdWriteTimestamp(self.handle(), pipeline_stage.bit(), query_pool.handle(), query);
        }
    }

    /// See [`vkCmdCopyQueryPoolResults`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdCopyQueryPoolResults)
    #[cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
    pub fn copy_query_pool_results(&self, query_pool: &QueryPool, first_query: u32, query_count: u32, dst_buffer: &Buffer, dst_offset: u64, stride: u64, flags: core::QueryResultFlags) {
        unsafe {
            self.loader().vk.vkCmdCopyQueryPoolResults(self.handle(), query_pool.handle(), first_query, query_count, dst_buffer.handle(), dst_offset, stride, flags.bits());
        }
    }

    /// See [`vkCmdPushConstants`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdPushConstants)
    pub fn push_constants(&self, layout: &PipelineLayout, stage_flags: core::ShaderStageFlags, offset: u32, values: &[u8]) {
        unsafe {
            self.loader().vk.vkCmdPushConstants(self.handle(), layout.handle(), stage_flags.bits(), offset, values.len() as u32, values.as_ptr() as *const c_void);
        }
    }

    /// See [`vkCmdBeginRenderPass`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdBeginRenderPass)
    pub fn begin_render_pass(&self, render_pass_begin: &core::RenderPassBeginInfo, contents: core::SubpassContents) {
        let render_pass_begin_wrapper = core::VkRenderPassBeginInfoWrapper::new(render_pass_begin, true);
        unsafe {
            self.loader().vk.vkCmdBeginRenderPass(self.handle(), &render_pass_begin_wrapper.vks_struct, contents.into());
        }
    }

    /// See [`vkCmdNextSubpass`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdNextSubpass)
    pub fn next_subpass(&self, contents: core::SubpassContents) {
        unsafe {
            self.loader().vk.vkCmdNextSubpass(self.handle(), contents.into());
        }
    }

    /// See [`vkCmdEndRenderPass`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdEndRenderPass)
    pub fn end_render_pass(&self) {
        unsafe {
            self.loader().vk.vkCmdEndRenderPass(self.handle());
        }
    }

    /// See [`vkCmdExecuteCommands`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdExecuteCommands)
    pub fn execute_commands(&self, command_buffers: &[CommandBuffer]) {
        let command_buffers: Vec<_> = command_buffers.iter().map(CommandBuffer::handle).collect();
        unsafe {
            self.loader().vk.vkCmdExecuteCommands(self.handle(), command_buffers.len() as u32, command_buffers.as_ptr());
        }
    }

    /// See [`vkCmdDrawIndirectCountAMD`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDrawIndirectCountAMD)
    /// and extension [`VK_AMD_draw_indirect_count`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_AMD_draw_indirect_count)
    pub fn draw_indirect_count_amd(&self, buffer: &Buffer, offset: u64, count_buffer: &Buffer, count_buffer_offset: u64, max_draw_count: u32, stride: u32) {
        unsafe {
            self.loader().amd_draw_indirect_count.vkCmdDrawIndirectCountAMD(self.handle(), buffer.handle(), offset, count_buffer.handle(), count_buffer_offset, max_draw_count, stride);
        }
    }

    /// See [`vkCmdDrawIndexedIndirectCountAMD`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDrawIndexedIndirectCountAMD)
    /// and extension [`VK_AMD_draw_indirect_count`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_AMD_draw_indirect_count)
    pub fn draw_indexed_indirect_count_amd(&self, buffer: &Buffer, offset: u64, count_buffer: &Buffer, count_buffer_offset: u64, max_draw_count: u32, stride: u32) {
        unsafe {
            self.loader().amd_draw_indirect_count.vkCmdDrawIndexedIndirectCountAMD(self.handle(), buffer.handle(), offset, count_buffer.handle(), count_buffer_offset, max_draw_count, stride);
        }
    }

    /// See [`vkCmdDebugMarkerBeginEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDebugMarkerBeginEXT)
    /// and extension [`VK_EXT_debug_marker`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_marker)
    pub fn debug_marker_begin_ext(&self, marker_info: &ext_debug_marker::DebugMarkerMarkerInfoExt) {
        let wrapper = ext_debug_marker::VkDebugMarkerMarkerInfoEXTWrapper::new(marker_info, true);
        unsafe { self.loader().ext_debug_marker.vkCmdDebugMarkerBeginEXT(self.handle(), &wrapper.vks_struct as *const _ as _); }
    }

    /// See [`vkCmdDebugMarkerEndEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDebugMarkerEndEXT)
    /// and extension [`VK_EXT_debug_marker`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_marker)
    pub fn debug_marker_end_ext(&self) {
        unsafe { self.loader().ext_debug_marker.vkCmdDebugMarkerEndEXT(self.handle()); }
    }

    /// See [`vkCmdDebugMarkerInsertEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkCmdDebugMarkerInsertEXT)
    /// and extension [`VK_EXT_debug_marker`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_debug_marker)
    pub fn debug_marker_insert_ext(&self, marker_info: &ext_debug_marker::DebugMarkerMarkerInfoExt) {
        let wrapper = ext_debug_marker::VkDebugMarkerMarkerInfoEXTWrapper::new(marker_info, true);
        unsafe { self.loader().ext_debug_marker.vkCmdDebugMarkerInsertEXT(self.handle(), &wrapper.vks_struct as *const _ as _); }
    }
}

#[derive(Debug)]
struct Inner {
    handle: vks::vk::VkCommandBuffer,
    owned: bool,
    command_pool: CommandPool,
}

impl Drop for Inner {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                self.command_pool.loader().vk.vkFreeCommandBuffers(self.command_pool.device_handle(), self.command_pool.handle(), 1, &self.handle);
            }
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
