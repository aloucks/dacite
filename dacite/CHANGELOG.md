# dacite Changelog

## Current Git master branch

 - Vulkan 1.0.5
 - Extension `VK_NV_glsl_shader` (revision 1)

### Breaking changes

 - New variant `InvalidShaderNv` added to `core::Error` (`VK_NV_glsl_shader`)


## Version 0.1.0, released on 05.06.2017

 - This is the initial release of dacite.
 - Includes support for Vulkan 1.0.4, all (mostly WSI) extensions up to that point and additionally
   `VK_KHR_get_surface_capabilities2`.
