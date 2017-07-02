# dacite Changelog

## Current Git master branch

 - Vulkan 1.0.14, 1.0.15, 1.0.16
 - Extension `VK_AMD_shader_trinary_minmax` (revision 1)

### Breaking changes

 - All occurrences of `Option<Vec<T>>` have been changed to just `Vec<T>` in the following structs:
   - `core::SubmitInfo`
   - `core::BindSparseInfo`
   - `core::BufferCreateInfo`
   - `core::ImageCreateInfo`
   - `core::PipelineCacheCreateInfo`
   - `core::SpecializationInfo`
   - `core::PipelineVertexInputStateCreateInfo`
   - `core::PipelineMultisampleStateCreateInfo`
   - `core::PipelineColorBlendStateCreateInfo`
   - `core::PipelineLayoutCreateInfo`
   - `core::DescriptorSetLayoutBinding`
   - `core::DescriptorSetLayoutCreateInfo`
   - `core::FramebufferCreateInfo`
   - `core::SubpassDescription`
   - `core::RenderPassCreateInfo`
   - `core::RenderPassBeginInfo`
   - `khr_swapchain::SwapchainCreateInfoKhr`
   - `khr_swapchain::PresentInfoKhr`

 - The builder `core::SpecializationInfoBuilder` has been removed. Entries to a
   `core::SpecializationInfo` can now be added conveniently with `core::SpecializationInfo::push`.


## Version 0.4.0, released on 25.06.2017

 - Vulkan 1.0.13
 - Added several convenience functions:
   - `core::Offset2D`: `new()`, `zero()`, `from_3d()`
   - `core::Offset3D`: `new()`, `zero()`, `from_2d()`
   - `core::Extent2D`: `new()`, `zero()`, `from_3d()`
   - `core::Extent3D`: `new()`, `zero()`, `from_2d()`
   - `core::ComponentMapping`: `identity()`
   - `core::Rect2D`: `new()`
 - Added `VulkanObject::id()`, which returns the handle cast to a `u64`. See the documentation of
   that function for more information.
 - Added a new trait `FromNativeObject`, which can be used to create dacite Vulkan objects from
   native FFI objects. The trait is implemented for types except `Instance` and `Device`.
 - Added version constant `DACITE_API_VERSION_1_0`.

### Breaking changes

 - Vks was updated to 0.19.x.
 - `core::Queue::bind_sparse()` takes `Fence` as a reference.
 - `core::Queue::submit()` takes `Fence` as a reference.


## Version 0.3.6, released on 11.06.2017

 - Vulkan 1.0.12
 - Extension `VK_AMD_rasterization_order` (revision 1)


## Version 0.3.5, released on 09.06.2017

 - Vulkan 1.0.11
 - Bump `VK_KHR_swapchain` to revision 68.


## Version 0.3.4, released on 09.06.2017

 - Vulkan 1.0.10


## Version 0.3.3, released on 08.06.2017

 - Vulkan 1.0.9


## Version 0.3.2, released on 08.06.2017

 - Vulkan 1.0.8
 - Dacite can now be built with stable Rust 1.18.0.


## Version 0.3.1, released on 07.06.2017

 - Vulkan 1.0.7


## Version 0.3.0, released on 06.06.2017

 - Vulkan 1.0.6
 - Extension `VK_IMG_filter_cubic` (revision 1)

### Breaking changes

 - New variant `CubicImg` added to `core::Filter` (`VK_IMG_filter_cubic`)

 - The following functions of `core::PhysicalDevice` have been renamed:
   - `properties` -> `get_properties`
   - `features` -> `get_features`
   - `format_properties` -> `get_format_properties`
   - `image_format_properties` -> `get_image_format_properties`
   - `sparse_image_format_properties` -> `get_sparse_image_format_properties`
   - `queue_family_properties` -> `get_queue_family_properties`
   - `memory_properties` -> `get_memory_properties`

 - The following functions no longer return an iterator, but instead behave like
   `std::iter::Iterator::collect` (all iterator types have been removed in the process):
   - `core::Instance::enumerate_instance_layer_properties`
   - `core::PhysicalDevice::enumerate_device_layer_properties`
   - `core::PhysicalDevice::get_sparse_image_format_properties`
   - `core::PhysicalDevice::get_queue_family_properties`
   - `core::PhysicalDevice::get_surface_support_khr`
   - `core::PhysicalDevice::get_surface_present_modes_khr`
   - `core::Image::get_sparse_memory_requirements`

 - The following functions have also been changed to behave like `std::iter::Iterator::collect`
   instead of always returning a `Vec`:
   - `core::PhysicalDevice::get_queue_family_properties2_khr`
   - `core::PhysicalDevice::get_sparse_image_format_properties2_khr`


## Version 0.2.0, released on 05.06.2017

 - Vulkan 1.0.5
 - Extension `VK_NV_glsl_shader` (revision 1)

### Breaking changes

 - New variant `InvalidShaderNv` added to `core::Error` (`VK_NV_glsl_shader`)


## Version 0.1.0, released on 05.06.2017

 - This is the initial release of dacite.
 - Includes support for Vulkan 1.0.4, all (mostly WSI) extensions up to that point and additionally
   `VK_KHR_get_surface_capabilities2`.
