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

#[macro_use]
extern crate bitflags;

extern crate dacite;
extern crate winit;

use dacite::core;
use dacite::khr_surface;
use dacite::khr_wayland_surface;
use dacite::khr_win32_surface;
use dacite::khr_xlib_surface;
use dacite::wayland_wrapper;
use dacite::win32_wrapper;
use dacite::xlib_wrapper;
use std::error;
use std::fmt;
use winit::Window;

pub trait WindowExt {
    fn is_presentation_supported(&self, physical_device: &core::PhysicalDevice, queue_family_indices: u32) -> Result<bool, Error>;
    fn get_required_extensions(&self) -> Result<core::InstanceExtensionsProperties, Error>;
    fn create_surface(&self, instance: &core::Instance, flags: SurfaceCreateFlags, allocator: Option<Box<core::Allocator>>) -> Result<khr_surface::SurfaceKhr, Error>;
}

impl WindowExt for Window {
    fn is_presentation_supported(&self, physical_device: &core::PhysicalDevice, queue_family_indices: u32) -> Result<bool, Error> {
        let backend = get_backend(self)?;

        match backend {
            Backend::Xlib { .. } => Ok(true), // FIXME: This needs a VisualID, which winit does not expose
            Backend::Wayland { display, .. } => Ok(physical_device.get_wayland_presentation_support_khr(queue_family_indices, display)),
            Backend::Win32 { .. } => Ok(physical_device.get_win32_presentation_support_khr(queue_family_indices)),
        }
    }

    fn get_required_extensions(&self) -> Result<core::InstanceExtensionsProperties, Error> {
        let backend = get_backend(self)?;

        let mut extensions = core::InstanceExtensionsProperties::new();
        extensions.add_khr_surface(25);

        match backend {
            Backend::Xlib { .. } => extensions.add_khr_xlib_surface(6),
            Backend::Wayland { .. } => extensions.add_khr_wayland_surface(5),
            Backend::Win32 { .. } => extensions.add_khr_win32_surface(5),
        };

        Ok(extensions)
    }

    fn create_surface(&self, instance: &core::Instance, _: SurfaceCreateFlags, allocator: Option<Box<core::Allocator>>) -> Result<khr_surface::SurfaceKhr, Error> {
        let backend = get_backend(self)?;

        match backend {
            Backend::Xlib { display, window } => {
                let create_info = khr_xlib_surface::XlibSurfaceCreateInfoKhr {
                    flags: khr_xlib_surface::XlibSurfaceCreateFlagsKhr::empty(),
                    dpy: display,
                    window: window,
                    chain: None,
                };

                Ok(instance.create_xlib_surface_khr(&create_info, allocator)?)
            }

            Backend::Wayland { display, surface } => {
                let create_info = khr_wayland_surface::WaylandSurfaceCreateInfoKhr {
                    flags: khr_wayland_surface::WaylandSurfaceCreateFlagsKhr::empty(),
                    display: display,
                    surface: surface,
                    chain: None,
                };

                Ok(instance.create_wayland_surface_khr(&create_info, allocator)?)
            }

            Backend::Win32 { hinstance, hwnd } => {
                let create_info = khr_win32_surface::Win32SurfaceCreateInfoKhr {
                    flags: khr_win32_surface::Win32SurfaceCreateFlagsKhr::empty(),
                    hinstance: hinstance,
                    hwnd: hwnd,
                    chain: None,
                };

                Ok(instance.create_win32_surface_khr(&create_info, allocator)?)
            }
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Unsupported,
    VulkanError(core::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Unsupported => write!(f, "Unsupported"),
            Error::VulkanError(e) => write!(f, "VulkanError({})", e),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Unsupported => "The windowing system is not supported",
            Error::VulkanError(ref e) => e.description(),
        }
    }
}

impl From<core::Error> for Error {
    fn from(e: core::Error) -> Self {
        Error::VulkanError(e)
    }
}

bitflags! {
    pub struct SurfaceCreateFlags: u32 {
        /// Dummy flag
        ///
        /// This flag exists just to satisfy the bitflags! macro, which doesn't support empty
        /// flags. Use `SurfaceCreateFlags::empty()` instead.
        const SURFACE_CREATE_DUMMY = 0;
    }
}

#[allow(dead_code)]
enum Backend {
    Xlib {
        display: *mut xlib_wrapper::Display,
        window: xlib_wrapper::Window,
    },

    Wayland {
        display: *mut wayland_wrapper::wl_display,
        surface: *mut wayland_wrapper::wl_surface,
    },

    Win32 {
        hinstance: win32_wrapper::HINSTANCE,
        hwnd: win32_wrapper::HWND,
    },
}

#[allow(unused_variables)]
#[allow(unreachable_code)]
fn get_backend(window: &Window) -> Result<Backend, Error> {
    #[cfg(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd", target_os = "openbsd"))]
    {
        use winit::os::unix::WindowExt;

        if let (Some(display), Some(window)) = (window.get_xlib_display(), window.get_xlib_window()) {
            return Ok(Backend::Xlib {
                display: display as _,
                window: dacite::xlib_wrapper::Window(window as _),
            });
        }

        if let (Some(display), Some(surface)) = (window.get_wayland_display(), window.get_wayland_surface()) {
            return Ok(Backend::Wayland {
                display: display as _,
                surface: surface as _,
            });
        }
    }

    #[cfg(target_os = "windows")]
    {
        use winit::os::windows::WindowExt;

        return Ok(Backend::Win32 {
            hinstance: ::std::ptr::null_mut(), // FIXME: Need HINSTANCE of the correct module
            hwnd: window.get_hwnd() as _,
        });
    }

    Err(Error::Unsupported)
}