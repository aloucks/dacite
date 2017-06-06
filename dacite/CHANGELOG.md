# dacite Changelog

## Current Git master branch

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
