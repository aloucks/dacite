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
use core::{self, Device, Fence};
use khr_swapchain;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ptr;
use vks;

/// See [`VkQueue`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkQueue)
#[derive(Debug, Clone)]
pub struct Queue {
    handle: vks::VkQueue,
    device: Device,
}

unsafe impl Send for Queue { }

unsafe impl Sync for Queue { }

impl PartialEq for Queue {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}

impl Eq for Queue { }

impl PartialOrd for Queue {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.handle.partial_cmp(&other.handle)
    }
}

impl Ord for Queue {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.handle.cmp(&other.handle)
    }
}

impl Hash for Queue {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.handle.hash(state);
    }
}

impl VulkanObject for Queue {
    type NativeVulkanObject = vks::VkQueue;

    #[inline]
    fn id(&self) -> u64 {
        self.as_native_vulkan_object() as u64
    }

    #[inline]
    fn as_native_vulkan_object(&self) -> Self::NativeVulkanObject {
        self.handle
    }

    fn try_destroy(self) -> Result<(), TryDestroyError<Self>> {
        Err(TryDestroyError::new(self, TryDestroyErrorKind::Unsupported))
    }
}

impl FromNativeObject for Queue {
    type Parameters = Device;

    unsafe fn from_native_object(object: Self::NativeVulkanObject, params: Self::Parameters) -> Self {
        Queue::new(object, params)
    }
}

impl Queue {
    pub(crate) fn new(handle: vks::VkQueue, device: Device) -> Self {
        Queue {
            handle: handle,
            device: device,
        }
    }

    pub(crate) fn loader(&self) -> &vks::DeviceProcAddrLoader {
        self.device.loader()
    }

    /// See [`vkQueueSubmit`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkQueueSubmit)
    pub fn submit(&self, submits: Option<&[core::SubmitInfo]>, fence: Option<Fence>) -> Result<(), core::Error> {
        #[allow(unused_variables)]
        let (submits_count, vk_submits_ptr, vk_submits, submits_wrappers) = match submits {
            Some(submits) => {
                let submits_wrappers: Vec<_> = submits.iter().map(|s| core::VkSubmitInfoWrapper::new(s, true)).collect();
                let vk_submits: Vec<vks::VkSubmitInfo> = submits_wrappers.iter().map(|s| s.vks_struct).collect();
                (submits.len() as u32, vk_submits.as_ptr(), Some(vk_submits), Some(submits_wrappers))
            }

            None => (0, ptr::null(), None, None),
        };

        let fence = fence.as_ref().map_or(ptr::null_mut(), Fence::handle);

        let res = unsafe {
            (self.loader().core.vkQueueSubmit)(self.handle, submits_count, vk_submits_ptr, fence)
        };

        if res == vks::VK_SUCCESS {
            Ok(())
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkQueueWaitIdle`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkQueueWaitIdle)
    pub fn wait_idle(&self) -> Result<(), core::Error> {
        let res = unsafe {
            (self.loader().core.vkQueueWaitIdle)(self.handle)
        };

        if res == vks::VK_SUCCESS {
            Ok(())
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkQueueBindSparse`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkQueueBindSparse)
    pub fn bind_sparse(&self, bind_infos: Option<&[core::BindSparseInfo]>, fence: Option<Fence>) -> Result<(), core::Error> {
        #[allow(unused_variables)]
        let (bind_infos_count, vk_bind_infos_ptr, vk_bind_infos, bind_infos_wrappers) = match bind_infos {
            Some(bind_infos) => {
                let bind_infos_wrappers: Vec<_> = bind_infos.iter().map(|b| core::VkBindSparseInfoWrapper::new(b, true)).collect();
                let vk_bind_infos: Vec<vks::VkBindSparseInfo> = bind_infos_wrappers.iter().map(|b| b.vks_struct).collect();
                (bind_infos.len() as u32, vk_bind_infos.as_ptr(), Some(vk_bind_infos), Some(bind_infos_wrappers))
            }

            None => (0, ptr::null(), None, None),
        };

        let fence = fence.as_ref().map_or(ptr::null_mut(), Fence::handle);

        let res = unsafe {
            (self.loader().core.vkQueueBindSparse)(self.handle, bind_infos_count, vk_bind_infos_ptr, fence)
        };

        if res == vks::VK_SUCCESS {
            Ok(())
        }
        else {
            Err(res.into())
        }
    }

    /// See [`vkQueuePresentKHR`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#vkQueuePresentKHR)
    /// and extension [`VK_KHR_swapchain`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_KHR_swapchain)
    pub fn queue_present_khr(&self, present_info: &mut khr_swapchain::PresentInfoKhr) -> Result<khr_swapchain::QueuePresentResultKhr, core::Error> {
        let present_info_wrapper = khr_swapchain::VkPresentInfoKHRWrapper::new(present_info, true);

        let res = unsafe {
            (self.loader().khr_swapchain.vkQueuePresentKHR)(self.handle, &present_info_wrapper.vks_struct)
        };

        if let Some(ref mut results) = present_info.results {
            results.clear();
            for &result in &present_info_wrapper.results.unwrap() {
                match result {
                    vks::VK_SUCCESS => results.push(Ok(khr_swapchain::QueuePresentResultKhr::Ok)),
                    vks::VK_SUBOPTIMAL_KHR => results.push(Ok(khr_swapchain::QueuePresentResultKhr::Suboptimal)),
                    _ => results.push(Err(result.into())),
                }
            }
        }

        match res {
            vks::VK_SUCCESS => Ok(khr_swapchain::QueuePresentResultKhr::Ok),
            vks::VK_SUBOPTIMAL_KHR => Ok(khr_swapchain::QueuePresentResultKhr::Suboptimal),
            _ => Err(res.into()),
        }
    }
}
