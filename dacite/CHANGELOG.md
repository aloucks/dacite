# dacite Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

Items listed in "Changed" and "Removed" sub-sections are usually breaking changes. Any additional
breaking changes in other sub-sections are prefixed with "**BREAKING**" to increase visibility.


## [0.7.0] - 2017-09-19
This release contains breaking changes.

### Fixed
 - Disabled extensions are no longer loaded

### Changed
 - Bumped lowest supported Rust version to 1.20.
 - Updated `vks` to 0.20.
 - Updated `bitflags` to version 1.0. All bitflags values are now associated consts.
 - The `FlagBits` types are now proper enums instead of simple aliases for `Flags`. This provides an
   additional layer of type safety. Vulkan uses `FlagBits` types, whenever only a single bit is
   supposed to be set.


## [0.6.1] - 2017-07-09

### Added
 - Vulkan 1.0.26, 1.0.27, 1.0.28, 1.0.29, 1.0.30
 - Extension `VK_EXT_validation_flags` (revision 1)
 - Extension `VK_AMD_gpu_shader_half_float` (revision 1)
 - Extension `VK_EXT_debug_marker` (revision 3)


## [0.6.0] - 2017-07-08
This release contains breaking changes.

### Added
 - Vulkan 1.0.22, 1.0.23, 1.0.24, 1.0.25
 - Extension `VK_IMG_format_pvrtc` (revision 1)
 - Extension `VK_AMD_draw_indirect_count` (revision 1)
 - Extension `VK_NV_external_memory_capabilities` (revision 1)
 - Extension `VK_NV_external_memory` (revision 1)
 - Extension `VK_NV_external_memory_win32` (revision 1)
 - Extension `VK_NV_win32_keyed_mutex` (revision 1)
 - **BREAKING**: New variant `FragmentedPool` added to `core::Error`.
 - **BREAKING**: Added several variants to `core::Format` (from `VK_IMG_format_pvrtc`).


## [0.5.1] - 2017-07-08

### Added
 - Vulkan 1.0.21
 - Added the following functions to `core::PhysicalDeviceFeatures`: `new`, `all`, `empty`,
   `is_empty`, `difference`, `intersection` and `union`.


## [0.5.0] - 2017-07-02
This release contains breaking changes.

### Added
 - Vulkan 1.0.14, 1.0.15, 1.0.16, 1.0.17, 1.0.18, 1.0.19, 1.0.20
 - Extension `VK_AMD_shader_trinary_minmax` (revision 1)
 - Extension `VK_AMD_shader_explicit_vertex_parameter` (revision 1)
 - Extension `VK_AMD_gcn_shader` (revision 1)
 - Extension `VK_NV_dedicated_allocation` (revision 1)

### Changed
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
 - Added `core::SpecializationInfo::push`, which can be used to add entries to a
   `core::SpecializationInfo`.

### Removed
 - **BREAKING**: The builder `core::SpecializationInfoBuilder` has been removed. Entries to a
   `core::SpecializationInfo` can now be added conveniently with `core::SpecializationInfo::push`.


## [0.4.0] - 2017-06-25
This release contains breaking changes.

### Added
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

### Changed
 - Vks was updated to 0.19.x.
 - `core::Queue::bind_sparse()` takes `Fence` as a reference.
 - `core::Queue::submit()` takes `Fence` as a reference.


## [0.3.6] - 2017-06-11

### Added
 - Vulkan 1.0.12
 - Extension `VK_AMD_rasterization_order` (revision 1)


## [0.3.5] - 2017-06-09

### Added
 - Vulkan 1.0.11
 - Bump `VK_KHR_swapchain` to revision 68.


## [0.3.4] - 2017-06-09

### Added
 - Vulkan 1.0.10


## [0.3.3] - 2017-06-08

### Added
 - Vulkan 1.0.9


## [0.3.2] - 2017-06-08

### Added
 - Vulkan 1.0.8
 - Dacite can now be built with stable Rust 1.18.0.


## [0.3.1] - 2017-06-07

### Added
 - Vulkan 1.0.7


## [0.3.0] - 2017-06-06
This release contains breaking changes.

### Added
 - Vulkan 1.0.6
 - Extension `VK_IMG_filter_cubic` (revision 1)
 - **BREAKING**: New variant `CubicImg` added to `core::Filter` (from `VK_IMG_filter_cubic`).

### Changed
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


## [0.2.0] - 2017-06-05
This release contains breaking changes.

### Added
 - Vulkan 1.0.5
 - Extension `VK_NV_glsl_shader` (revision 1)
 - **BREAKING**: New variant `InvalidShaderNv` added to `core::Error` (from `VK_NV_glsl_shader`).


## [0.1.0] - 2017-06-05
This is the initial release of dacite.

### Added
 - Support for Vulkan 1.0.4, all (mostly WSI) extensions up to that point and additionally
   `VK_KHR_get_surface_capabilities2`.
