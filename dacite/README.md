# dacite

Mostly safe Vulkan library for Rust.

[![build status](https://gitlab.com/dennis-hamester/dacite/badges/master/build.svg)](https://gitlab.com/dennis-hamester/dacite)
[![dacite on crates.io](https://img.shields.io/crates/v/dacite.svg)](https://crates.io/crates/dacite)
[![dacite on docs.rs](https://docs.rs/dacite/badge.svg)](https://docs.rs/dacite)
[![dacite license](https://img.shields.io/badge/license-ISC-blue.svg)](LICENSE)

## Quick Links

 - Crate on crates.io: <https://crates.io/crates/dacite>
 - Documentation: <https://docs.rs/dacite>

## Current Status

 - Version: 0.7.0
 - Supported Vulkan specification: 1.0.30

Development is active and ongoing. Support for new Vulkan revisions will be added one-by-one. New
extensions will also be supported along the way.

Dacite is sorely lacking in the documentation department. I don't plan to fully document every type
and function (and thus effectively repeat the Vulkan specification). But at least the parts where
dacite is different from the C API will be documented over time.

## Usage

Rust 1.18.0 or newer is required.

Dacite is available on [crates.io]. Add this to your `Cargo.toml`:

```toml
[dependencies]
dacite = "0.7"
```

Check out the [examples] subdirectory to get started.

The Vulkan library (or more specifically, the Vulkan icd loader) is loaded dynamically at runtime.
This means, that development with dacite works out-of-the-box, without installing anything else
(like C headers or the LunarG Vulkan SDK on Windows).

Dacite uses [vks] as its Vulkan FFI bindings and symbol loader.

[crates.io]: https://crates.io/crates/dacite
[examples]: https://gitlab.com/dennis-hamester/dacite/tree/master/examples
[vks]: https://gitlab.com/dennis-hamester/vks

## Goals and Non-Goals

 - Map the whole Vulkan API almost one-to-one to equivalent Rust types and functions, unless there
   is a good reason not to.
 - Support multi-threaded usage.
 - Don't hide the flexibility and complexity of Vulkan.
 - Don't reimplement the validation layers: valid usage is not checked, but assumed.

Completeness and a simple mapping between the C and Rust API are probably the most important goals.
As such, most of dacites responsibility lies in converting the provided Rust types to and from their
C counterparts.

The Vulkan specification is very strict about what constitutes valid usage and what doesn't. The
general rule is, that invalid usage results in undefined behaviour. Dacite does not in any way
enforce valid API usage by its design.

I strongly suggest to check out the [vulkano] project, if dacite doesn't suit your requirements.

[vulkano]: https://github.com/tomaka/vulkano

## Safety

Dacite is safe, as long as it is used according to the Vulkan specification. However, invalid usage
will result in undefined behaviour. Enabling the validation layers during development is a good way
to prevent bad things from happening. Dacite does very little on its own in this regard.

One exception to this rule is, that the lifetime of an object is bound to its direct parent object
(dynamically, via reference counting) from which it was created. For example, an `Image` will always
outlive its parent `Device`. However, other dependencies are not modelled. For instance, binding
some `DeviceMemory` to a `Buffer`, will not ensure, that the `DeviceMemory` outlives the `Buffer`.
The same is true for essentially all such relationships in the Vulkan API.

## Stability

Dacite follows the [Semantic Versioning] scheme. Although the current version is still below 1.0.0,
the minor version component will be incremented whenever breaking changes are made. Version 1.0.0
will not be released, before dacite supports the most recent Vulkan revision and all extensions.

The main source of breaking API changes are new Vulkan extensions. More specifically, new variants
added to an existing enum by a new extension must be considered a breaking change.

Dacites has been designed such, that all other additions by new extensions should not break existing
applications. However, the development of [vks] has shown, that many Vulkan revisions contain minor
mistakes, which are then fixed later. If one of these mistakes makes it into dacite, its removal
might be breaking change.

[Semantic Versioning]: http://semver.org/
[vks]: https://gitlab.com/dennis-hamester/vks

## Supported Extensions

### `KHR` Extensions

| Extension | Revision |
| --- | --- |
| `VK_KHR_android_surface` | 6 |
| `VK_KHR_display_swapchain` | 9 |
| `VK_KHR_display` | 21 |
| `VK_KHR_get_physical_device_properties2` | 1 |
| `VK_KHR_mir_surface` | 4 |
| `VK_KHR_sampler_mirror_clamp_to_edge` | 1 |
| `VK_KHR_surface` | 25 |
| `VK_KHR_swapchain` | 68 |
| `VK_KHR_wayland_surface` | 5 |
| `VK_KHR_win32_surface` | 5 |
| `VK_KHR_xcb_surface` | 6 |
| `VK_KHR_xlib_surface` | 6 |

### `EXT` Extensions

| Extension | Revision |
| --- | --- |
| `VK_EXT_debug_marker` | 3 |
| `VK_EXT_debug_report` | 3 |
| `VK_EXT_validation_flags` | 1 |

### `NV` Extensions

| Extension | Revision |
| --- | --- |
| `VK_NV_dedicated_allocation` | 1 |
| `VK_NV_external_memory_capabilities` | 1 |
| `VK_NV_external_memory_win32` | 1 |
| `VK_NV_external_memory` | 1 |
| `VK_NV_glsl_shader` | 1 |
| `VK_NV_win32_keyed_mutex` | 1 |

### `AMD` Extensions

| Extension | Revision |
| --- | --- |
| `VK_AMD_draw_indirect_count` | 1 |
| `VK_AMD_gcn_shader` | 1 |
| `VK_AMD_gpu_shader_half_float` | 1 |
| `VK_AMD_rasterization_order` | 1 |
| `VK_AMD_shader_explicit_vertex_parameter` | 1 |
| `VK_AMD_shader_trinary_minmax` | 1 |

### `IMG` Extensions

| Extension | Revision |
| --- | --- |
| `VK_IMG_filter_cubic` | 1 |
| `VK_IMG_format_pvrtc` | 1 |

## License

Dacite is licensed under the ISC license:

```
Copyright (c) 2017, Dennis Hamester <dennis.hamester@startmail.com>

Permission to use, copy, modify, and/or distribute this software for any
purpose with or without fee is hereby granted, provided that the above
copyright notice and this permission notice appear in all copies.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND
FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
PERFORMANCE OF THIS SOFTWARE.
```
