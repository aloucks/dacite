extern crate dacite;
extern crate winit;

use std::process;

#[cfg(target_os = "linux")]
use winit::os::unix::WindowExt;

enum WindowBackend {
    #[cfg(feature = "khr_xlib_surface_6")]
    Xlib {
        display: *mut dacite::xlib_wrapper::Display,
        window: dacite::xlib_wrapper::Window,
    },
}

struct Window {
    events_loop: winit::EventsLoop,
    window: winit::Window,
    backend: WindowBackend,
}

struct QueueFamilyIndices {
    graphics: u32,
    present: u32,
}

struct DeviceSettings {
    physical_device: dacite::core::PhysicalDevice,
    queue_family_indices: QueueFamilyIndices,
    device_extensions: Vec<dacite::core::DeviceExtension>,
}

#[allow(unused_mut)]
fn create_window() -> Result<Window, ()> {
    let width = 800;
    let height = 600;

    let events_loop = winit::EventsLoop::new();
    let window = winit::WindowBuilder::new()
        .with_dimensions(width, height)
        .with_min_dimensions(width, height)
        .with_max_dimensions(width, height)
        .with_visibility(false)
        .build(&events_loop);

    let window = window.map_err(|e| {
        match e {
            winit::CreationError::OsError(e) => println!("Failed to create window ({})", e),
            winit::CreationError::NotSupported => println!("Failed to create window (not supported)"),
        }
    })?;

    let mut backend = None;

    #[cfg(all(target_os = "linux", feature = "khr_xlib_surface_6"))]
    {
        if backend.is_none() {
            if let (Some(xlib_display), Some(xlib_window)) = (window.get_xlib_display(), window.get_xlib_window()) {
                backend = Some(WindowBackend::Xlib {
                    display: xlib_display as _,
                    window: dacite::xlib_wrapper::Window(xlib_window as _),
                });
            }
        }
    }

    if let Some(backend) = backend {
        Ok(Window {
            events_loop: events_loop,
            window: window,
            backend: backend,
        })
    }
    else {
        println!("Failed to create window (backend is not supported)");
        Err(())
    }
}

fn compute_instance_extensions(backend: &WindowBackend) -> Result<Vec<dacite::core::InstanceExtension>, ()> {
    let mut required_instance_extensions = vec![dacite::core::InstanceExtensionProperties {
        extension: dacite::core::InstanceExtension::KhrSurface,
        spec_version: 25,
    }];

    #[cfg(feature = "khr_xlib_surface_6")]
    match *backend {
        WindowBackend::Xlib { .. } => required_instance_extensions.push(dacite::core::InstanceExtensionProperties {
            extension: dacite::core::InstanceExtension::KhrXlibSurface,
            spec_version: 6,
        }),
    }

    dacite::core::Instance::check_instance_extensions(required_instance_extensions).map_err(|e| {
        match e {
            dacite::core::CheckInstanceExtensionsError::Missing(missing) => {
                for missing in missing {
                    let name: String = missing.extension.into();
                    println!("Extension {} (revision {}) missing", name, missing.spec_version);
                }
            }

            dacite::core::CheckInstanceExtensionsError::VulkanError(e) => println!("Failed to query instance extensions ({})", e),
        }
    })
}

fn create_instance(instance_extensions: Vec<dacite::core::InstanceExtension>) -> Result<dacite::core::Instance, ()> {
    let application_info = dacite::core::ApplicationInfo {
        application_name: Some("dacite triangle example".to_owned()),
        application_version: 0,
        engine_name: None,
        engine_version: 0,
        api_version: Some(dacite::DACITE_API_VERSION),
        chain: None,
    };

    let create_info = dacite::core::InstanceCreateInfo {
        flags: dacite::core::InstanceCreateFlags::empty(),
        application_info: Some(application_info),
        enabled_layers: vec![],
        enabled_extensions: instance_extensions,
        chain: None,
    };

    dacite::core::Instance::create(&create_info, None).map_err(|e| {
        println!("Failed to create instance ({})", e);
    })
}

fn create_surface(instance: &dacite::core::Instance, backend: &WindowBackend) -> Result<dacite::khr_surface::SurfaceKhr, ()> {
    match *backend {
        #[cfg(feature = "khr_xlib_surface_6")]
        WindowBackend::Xlib { ref display, ref window } => {
            let xlib_surface_create_info = dacite::khr_xlib_surface::XlibSurfaceCreateInfoKhr {
                flags: dacite::khr_xlib_surface::XlibSurfaceCreateFlagsKhr::empty(),
                dpy: *display,
                window: *window,
                chain: None,
            };

            Ok(instance.create_xlib_surface_khr(&xlib_surface_create_info, None).map_err(|e| {
                println!("Failed to create xlib surface ({})", e);
            })?)
        }
    }
}

fn check_device_suitability(physical_device: dacite::core::PhysicalDevice, surface: &dacite::khr_surface::SurfaceKhr) -> Result<DeviceSettings, ()> {
    let mut graphics_index = None;
    let mut present_index = None;

    for (index, queue_family_properties) in physical_device.queue_family_properties().enumerate() {
        if queue_family_properties.queue_count == 0 {
            continue;
        }

        if graphics_index.is_none() && queue_family_properties.queue_flags.contains(dacite::core::QUEUE_GRAPHICS_BIT) {
            graphics_index = Some(index);
        }

        if present_index.is_none() {
            if let Ok(true) = physical_device.get_surface_support_khr(index as u32, surface) {
                present_index = Some(index);
            }
        }

    }

    if graphics_index.is_none() || present_index.is_none() {
        return Err(());
    }

    let required_device_extensions = vec![dacite::core::DeviceExtensionProperties {
        extension: dacite::core::DeviceExtension::KhrSwapchain,
        spec_version: 67,
    }];

    let device_extensions = physical_device.check_device_extensions(required_device_extensions).map_err(|_| ())?;

    Ok(DeviceSettings {
        physical_device: physical_device,
        queue_family_indices: QueueFamilyIndices {
            graphics: graphics_index.unwrap() as u32,
            present: present_index.unwrap() as u32,
        },
        device_extensions: device_extensions,
    })
}

fn find_suitable_device(instance: &dacite::core::Instance, surface: &dacite::khr_surface::SurfaceKhr) -> Result<DeviceSettings, ()> {
    let physical_devices = instance.enumerate_physical_devices().map_err(|e| {
        println!("Failed to enumerate physical devices ({})", e);
    })?;

    for physical_device in physical_devices {
        if let Ok(device_settings) = check_device_suitability(physical_device, surface) {
            return Ok(device_settings);
        }
    }

    println!("Failed to find a suitable device");
    Err(())
}

fn create_device(physical_device: &dacite::core::PhysicalDevice, device_extensions: Vec<dacite::core::DeviceExtension>, queue_family_indices: &QueueFamilyIndices) -> Result<dacite::core::Device, ()> {
    let device_queue_create_infos = vec![
        dacite::core::DeviceQueueCreateInfo {
            flags: dacite::core::DeviceQueueCreateFlags::empty(),
            queue_family_index: queue_family_indices.graphics,
            queue_priorities: vec![1.0],
            chain: None,
        },
    ];

    let device_create_info = dacite::core::DeviceCreateInfo {
        flags: dacite::core::DeviceCreateFlags::empty(),
        queue_create_infos: device_queue_create_infos,
        enabled_layers: vec![],
        enabled_extensions: device_extensions,
        enabled_features: None,
        chain: None,
    };

    physical_device.create_device(&device_create_info, None).map_err(|e| {
        println!("Failed to create device ({})", e);
    })
}

fn real_main() -> Result<(), ()> {
    let Window {
        events_loop,
        window,
        backend: window_backend,
    } = create_window()?;

    let instance_extensions = compute_instance_extensions(&window_backend)?;
    let instance = create_instance(instance_extensions)?;
    let surface = create_surface(&instance, &window_backend)?;

    let DeviceSettings {
        physical_device,
        queue_family_indices,
        device_extensions,
    } = find_suitable_device(&instance, &surface)?;

    let device = create_device(&physical_device, device_extensions, &queue_family_indices)?;
    let graphics_queue = device.get_queue(queue_family_indices.graphics, 0);
    let present_queue = device.get_queue(queue_family_indices.present, 0);

    window.show();
    events_loop.run_forever(|event| {
        if let winit::Event::WindowEvent { event: winit::WindowEvent::Closed, .. } = event {
            events_loop.interrupt();
        }
    });

    Ok(())
}

fn main() {
    match real_main() {
        Ok(_) => process::exit(0),
        Err(_) => process::exit(1),
    }
}
