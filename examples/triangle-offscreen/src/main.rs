extern crate dacite;
extern crate png;
#[macro_use] extern crate glsl_to_spirv_macros;
#[macro_use] extern crate glsl_to_spirv_macros_impl;

use png::HasParameters;
use std::fs::File;
use std::process;
use std::slice;

struct DeviceSettings {
    physical_device: dacite::core::PhysicalDevice,
    queue_family_index: u32,
    memory_types: Vec<dacite::core::MemoryType>,
}

struct FramebufferSettings {
    image: dacite::core::Image,
    memory: dacite::core::DeviceMemory,
    view: dacite::core::ImageView,
    framebuffer: dacite::core::Framebuffer,
}

struct BufferSettings {
    buffer: dacite::core::Buffer,
    memory: dacite::core::DeviceMemory,
}

fn create_instance() -> Result<dacite::core::Instance, ()> {
    let application_info = dacite::core::ApplicationInfo {
        application_name: Some("dacite triangle-offscreen example".to_owned()),
        application_version: 0,
        engine_name: None,
        engine_version: 0,
        api_version: Some(dacite::DACITE_API_VERSION_1_0),
        chain: None,
    };

    let create_info = dacite::core::InstanceCreateInfo {
        flags: dacite::core::InstanceCreateFlags::empty(),
        application_info: Some(application_info),
        enabled_layers: vec![],
        enabled_extensions: dacite::core::InstanceExtensions::new(),
        chain: None,
    };

    dacite::core::Instance::create(&create_info, None).map_err(|e| {
        println!("Failed to create instance ({})", e);
    })
}

fn find_suitable_device(instance: &dacite::core::Instance) -> Result<DeviceSettings, ()> {
    let physical_devices = instance.enumerate_physical_devices().map_err(|e| {
        println!("Failed to enumerate physical devices ({})", e);
    })?;

    for physical_device in physical_devices {
        let queue_family_properties: Vec<_> = physical_device.get_queue_family_properties();
        for (index, queue_family_properties) in queue_family_properties.into_iter().enumerate() {
            if queue_family_properties.queue_count == 0 {
                continue;
            }

            if queue_family_properties.queue_flags.contains(dacite::core::QUEUE_GRAPHICS_BIT) {
                return Ok(DeviceSettings {
                    physical_device: physical_device.clone(),
                    queue_family_index: index as u32,
                    memory_types: physical_device.get_memory_properties().memory_types,
                });
            }
        }
    }

    println!("Failed to find a suitable device");
    Err(())
}

fn create_device(physical_device: &dacite::core::PhysicalDevice, queue_family_index: u32) -> Result<dacite::core::Device, ()> {
    let device_queue_create_infos = vec![
        dacite::core::DeviceQueueCreateInfo {
            flags: dacite::core::DeviceQueueCreateFlags::empty(),
            queue_family_index: queue_family_index,
            queue_priorities: vec![1.0],
            chain: None,
        },
    ];

    let device_create_info = dacite::core::DeviceCreateInfo {
        flags: dacite::core::DeviceCreateFlags::empty(),
        queue_create_infos: device_queue_create_infos,
        enabled_layers: vec![],
        enabled_extensions: dacite::core::DeviceExtensions::new(),
        enabled_features: None,
        chain: None,
    };

    physical_device.create_device(&device_create_info, None).map_err(|e| {
        println!("Failed to create device ({})", e);
    })
}

fn create_render_pass(device: &dacite::core::Device, format: dacite::core::Format) -> Result<dacite::core::RenderPass, ()> {
    let create_info = dacite::core::RenderPassCreateInfo {
        flags: dacite::core::RenderPassCreateFlags::empty(),
        attachments: Some(vec![dacite::core::AttachmentDescription {
            flags: dacite::core::AttachmentDescriptionFlags::empty(),
            format: format,
            samples: dacite::core::SAMPLE_COUNT_1_BIT,
            load_op: dacite::core::AttachmentLoadOp::Clear,
            store_op: dacite::core::AttachmentStoreOp::Store,
            stencil_load_op: dacite::core::AttachmentLoadOp::DontCare,
            stencil_store_op: dacite::core::AttachmentStoreOp::DontCare,
            initial_layout: dacite::core::ImageLayout::Undefined,
            final_layout: dacite::core::ImageLayout::TransferSrcOptimal,
        }]),
        subpasses: vec![dacite::core::SubpassDescription {
            flags: dacite::core::SubpassDescriptionFlags::empty(),
            pipeline_bind_point: dacite::core::PipelineBindPoint::Graphics,
            input_attachments: None,
            color_attachments: Some(vec![dacite::core::AttachmentReference {
                attachment: dacite::core::AttachmentIndex::Index(0),
                layout: dacite::core::ImageLayout::ColorAttachmentOptimal,
            }]),
            resolve_attachments: None,
            depth_stencil_attachment: None,
            preserve_attachments: None,
        }],
        dependencies: None,
        chain: None,
    };

    device.create_render_pass(&create_info, None).map_err(|e| {
        println!("Failed to create renderpass ({})", e);
    })
}

fn find_memory_type(types: &[dacite::core::MemoryType], type_bits: u32, properties: dacite::core::MemoryPropertyFlags) -> Option<u32> {
    types.iter().enumerate()
        .find(|&(i, m)| {
            ((type_bits & (1 << i)) != 0) && m.property_flags.contains(properties)
        })
    .map(|(i, _)| i as u32)
}

fn create_framebuffer(device: &dacite::core::Device, render_pass: &dacite::core::RenderPass, format: dacite::core::Format, extent: &dacite::core::Extent2D, memory_types: &[dacite::core::MemoryType]) -> Result<FramebufferSettings, ()> {
    let image_create_info = dacite::core::ImageCreateInfo {
        flags: dacite::core::ImageCreateFlags::empty(),
        image_type: dacite::core::ImageType::Type2D,
        format: format,
        extent: dacite::core::Extent3D::from_2d(extent, 1),
        mip_levels: 1,
        array_layers: 1,
        samples: dacite::core::SAMPLE_COUNT_1_BIT,
        tiling: dacite::core::ImageTiling::Optimal,
        usage: dacite::core::IMAGE_USAGE_COLOR_ATTACHMENT_BIT | dacite::core::IMAGE_USAGE_TRANSFER_SRC_BIT,
        sharing_mode: dacite::core::SharingMode::Exclusive,
        queue_family_indices: None,
        initial_layout: dacite::core::ImageLayout::Undefined,
        chain: None,
    };

    let image = device.create_image(&image_create_info, None).map_err(|e| {
        println!("Failed to create image ({})", e);
    })?;

    let mem_reqs = image.get_memory_requirements();

    let mem_type = find_memory_type(memory_types, mem_reqs.memory_type_bits, dacite::core::MEMORY_PROPERTY_DEVICE_LOCAL_BIT).ok_or_else(|| {
        println!("Failed to find image memory type");
    })?;

    let image_allocate_info = dacite::core::MemoryAllocateInfo {
        allocation_size: mem_reqs.size,
        memory_type_index: mem_type,
        chain: None,
    };

    let memory = device.allocate_memory(&image_allocate_info, None).map_err(|e| {
        println!("Failed to allocate image memory ({})", e);
    })?;

    image.bind_memory(memory.clone(), 0).map_err(|e| {
        println!("Failed to bind image memory ({})", e);
    })?;

    let view_create_info = dacite::core::ImageViewCreateInfo {
        flags: dacite::core::ImageViewCreateFlags::empty(),
        image: image.clone(),
        view_type: dacite::core::ImageViewType::Type2D,
        format: format,
        components: dacite::core::ComponentMapping::identity(),
        subresource_range: dacite::core::ImageSubresourceRange {
            aspect_mask: dacite::core::IMAGE_ASPECT_COLOR_BIT,
            base_mip_level: 0,
            level_count: dacite::core::OptionalMipLevels::MipLevels(1),
            base_array_layer: 0,
            layer_count: dacite::core::OptionalArrayLayers::ArrayLayers(1),
        },
        chain: None,
    };

    let view = device.create_image_view(&view_create_info, None).map_err(|e| {
        println!("Failed to create image view ({})", e);
    })?;

    let create_info = dacite::core::FramebufferCreateInfo {
        flags: dacite::core::FramebufferCreateFlags::empty(),
        render_pass: render_pass.clone(),
        attachments: Some(vec![view.clone()]),
        width: extent.width,
        height: extent.height,
        layers: 1,
        chain: None,
    };

    let framebuffer = device.create_framebuffer(&create_info, None).map_err(|e| {
        println!("Failed to create framebuffer ({})", e);
    })?;

    Ok(FramebufferSettings {
        image: image,
        memory: memory,
        view: view,
        framebuffer: framebuffer,
    })
}

fn create_buffer(device: &dacite::core::Device, extent: &dacite::core::Extent2D, memory_types: &[dacite::core::MemoryType]) -> Result<BufferSettings, ()> {
    let create_info = dacite::core::BufferCreateInfo {
        flags: dacite::core::BufferCreateFlags::empty(),
        size: 4 * extent.width as u64 * extent.height as u64,
        usage: dacite::core::BUFFER_USAGE_TRANSFER_DST_BIT,
        sharing_mode: dacite::core::SharingMode::Exclusive,
        queue_family_indices: vec![],
        chain: None,
    };

    let buffer = device.create_buffer(&create_info, None).map_err(|e| {
        println!("Failed to create buffer ({})", e);
    })?;

    let mem_reqs = buffer.get_memory_requirements();

    let mem_type = find_memory_type(memory_types, mem_reqs.memory_type_bits, dacite::core::MEMORY_PROPERTY_HOST_VISIBLE_BIT).ok_or_else(|| {
        println!("Failed to find buffer memory type");
    })?;

    let allocate_info = dacite::core::MemoryAllocateInfo {
        allocation_size: mem_reqs.size,
        memory_type_index: mem_type,
        chain: None,
    };

    let memory = device.allocate_memory(&allocate_info, None).map_err(|e| {
        println!("Failed to allocate buffer memory ({})", e);
    })?;

    buffer.bind_memory(memory.clone(), 0).map_err(|e| {
        println!("Failed to bind buffer memory ({})", e);
    })?;

    Ok(BufferSettings {
        buffer: buffer,
        memory: memory,
    })
}

fn create_vertex_shader(device: &dacite::core::Device) -> Result<dacite::core::ShaderModule, ()> {
    let vertex_shader_bytes = glsl_vs!{r#"
        #version 450

        out gl_PerVertex {
            vec4 gl_Position;
        };

        layout(location = 0) out vec3 fragColor;

        vec2 positions[3] = vec2[](
            vec2(0.0, -0.5),
            vec2(0.5, 0.5),
            vec2(-0.5, 0.5)
        );

        vec3 colors[3] = vec3[](
            vec3(1.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
            vec3(0.0, 0.0, 1.0)
        );

        void main() {
            gl_Position = vec4(positions[gl_VertexIndex], 0.0, 1.0);
            fragColor = colors[gl_VertexIndex];
        }
    "#};

    let create_info = dacite::core::ShaderModuleCreateInfo {
        flags: dacite::core::ShaderModuleCreateFlags::empty(),
        code: vertex_shader_bytes.to_vec(),
        chain: None,
    };

    device.create_shader_module(&create_info, None).map_err(|e| {
        println!("Failed to create vertex shader module ({})", e);
    })
}

fn create_fragment_shader(device: &dacite::core::Device) -> Result<dacite::core::ShaderModule, ()> {
    let fragment_shader_bytes = glsl_fs!{r#"
        #version 450

        layout(location = 0) in vec3 fragColor;

        layout(location = 0) out vec4 outColor;

        void main() {
            outColor = vec4(fragColor, 1.0);
        }
    "#};

    let create_info = dacite::core::ShaderModuleCreateInfo {
        flags: dacite::core::ShaderModuleCreateFlags::empty(),
        code: fragment_shader_bytes.to_vec(),
        chain: None,
    };

    device.create_shader_module(&create_info, None).map_err(|e| {
        println!("Failed to create fragment shader module ({})", e);
    })
}

fn create_pipeline_layout(device: &dacite::core::Device) -> Result<dacite::core::PipelineLayout, ()> {
    let create_info = dacite::core::PipelineLayoutCreateInfo {
        flags: dacite::core::PipelineLayoutCreateFlags::empty(),
        set_layouts: None,
        push_constant_ranges: None,
        chain: None,
    };

    device.create_pipeline_layout(&create_info, None).map_err(|e| {
        println!("Failed to create pipeline layout ({})", e);
    })
}

fn create_pipeline(device: &dacite::core::Device, render_pass: &dacite::core::RenderPass, extent: &dacite::core::Extent2D) -> Result<dacite::core::Pipeline, ()> {
    let vertex_shader = create_vertex_shader(device)?;
    let fragment_shader = create_fragment_shader(device)?;
    let layout = create_pipeline_layout(device)?;

    let create_infos = vec![dacite::core::GraphicsPipelineCreateInfo {
        flags: dacite::core::PipelineCreateFlags::empty(),
        stages: vec![
            dacite::core::PipelineShaderStageCreateInfo {
                flags: dacite::core::PipelineShaderStageCreateFlags::empty(),
                stage: dacite::core::SHADER_STAGE_VERTEX_BIT,
                module: vertex_shader.clone(),
                name: "main".to_owned(),
                specialization_info: None,
                chain: None,
            },
            dacite::core::PipelineShaderStageCreateInfo {
                flags: dacite::core::PipelineShaderStageCreateFlags::empty(),
                stage: dacite::core::SHADER_STAGE_FRAGMENT_BIT,
                module: fragment_shader.clone(),
                name: "main".to_owned(),
                specialization_info: None,
                chain: None,
            },
        ],
        vertex_input_state: dacite::core::PipelineVertexInputStateCreateInfo {
            flags: dacite::core::PipelineVertexInputStateCreateFlags::empty(),
            vertex_binding_descriptions: None,
            vertex_attribute_descriptions: None,
            chain: None,
        },
        input_assembly_state: dacite::core::PipelineInputAssemblyStateCreateInfo {
            flags: dacite::core::PipelineInputAssemblyStateCreateFlags::empty(),
            topology: dacite::core::PrimitiveTopology::TriangleList,
            primitive_restart_enable: false,
            chain: None,
        },
        tessellation_state: None,
        viewport_state: Some(dacite::core::PipelineViewportStateCreateInfo {
            flags: dacite::core::PipelineViewportStateCreateFlags::empty(),
            viewports: vec![dacite::core::Viewport {
                x: 0.0,
                y: 0.0,
                width: extent.width as f32,
                height: extent.height as f32,
                min_depth: 0.0,
                max_depth: 1.0,
            }],
            scissors: vec![dacite::core::Rect2D::new(dacite::core::Offset2D::zero(), *extent)],
            chain: None,
        }),
        rasterization_state: dacite::core::PipelineRasterizationStateCreateInfo {
            flags: dacite::core::PipelineRasterizationStateCreateFlags::empty(),
            depth_clamp_enable: false,
            rasterizer_discard_enable: false,
            polygon_mode: dacite::core::PolygonMode::Fill,
            cull_mode: dacite::core::CULL_MODE_NONE,
            front_face: dacite::core::FrontFace::Clockwise,
            depth_bias_enable: false,
            depth_bias_constant_factor: 0.0,
            depth_bias_clamp: 0.0,
            depth_bias_slope_factor: 0.0,
            line_width: 1.0,
            chain: None,
        },
        multisample_state: Some(dacite::core::PipelineMultisampleStateCreateInfo {
            flags: dacite::core::PipelineMultisampleStateCreateFlags::empty(),
            rasterization_samples: dacite::core::SAMPLE_COUNT_1_BIT,
            sample_shading_enable: false,
            min_sample_shading: 1.0,
            sample_mask: None,
            alpha_to_coverage_enable: false,
            alpha_to_one_enable: false,
            chain: None,
        }),
        depth_stencil_state: None,
        color_blend_state: Some(dacite::core::PipelineColorBlendStateCreateInfo {
            flags: dacite::core::PipelineColorBlendStateCreateFlags::empty(),
            logic_op_enable: false,
            logic_op: dacite::core::LogicOp::Copy,
            attachments: Some(vec![dacite::core::PipelineColorBlendAttachmentState {
                blend_enable: false,
                src_color_blend_factor: dacite::core::BlendFactor::One,
                dst_color_blend_factor: dacite::core::BlendFactor::Zero,
                color_blend_op: dacite::core::BlendOp::Add,
                src_alpha_blend_factor: dacite::core::BlendFactor::One,
                dst_alpha_blend_factor: dacite::core::BlendFactor::Zero,
                alpha_blend_op: dacite::core::BlendOp::Add,
                color_write_mask: dacite::core::COLOR_COMPONENT_R_BIT | dacite::core::COLOR_COMPONENT_G_BIT | dacite::core::COLOR_COMPONENT_B_BIT,
            }]),
            blend_constants: [0.0, 0.0, 0.0, 0.0],
            chain: None,
        }),
        dynamic_state: None,
        layout: layout.clone(),
        render_pass: render_pass.clone(),
        subpass: 0,
        base_pipeline: None,
        base_pipeline_index: None,
        chain: None,
    }];

    let pipelines = device.create_graphics_pipelines(None, &create_infos, None).map_err(|(e, _)| {
        println!("Failed to create pipeline ({})", e);
    })?;

    Ok(pipelines[0].clone())
}

fn create_command_pool(device: &dacite::core::Device, queue_family_index: u32) -> Result<dacite::core::CommandPool, ()> {
    let create_info = dacite::core::CommandPoolCreateInfo {
        flags: dacite::core::CommandPoolCreateFlags::empty(),
        queue_family_index: queue_family_index,
        chain: None,
    };

    device.create_command_pool(&create_info, None).map_err(|e| {
        println!("Failed to create command pool ({})", e);
    })
}

fn record_command_buffer(command_pool: &dacite::core::CommandPool, pipeline: &dacite::core::Pipeline, framebuffer: &dacite::core::Framebuffer, render_pass: &dacite::core::RenderPass, image: &dacite::core::Image, buffer: &dacite::core::Buffer, extent: &dacite::core::Extent2D) -> Result<dacite::core::CommandBuffer, ()> {
    let allocate_info = dacite::core::CommandBufferAllocateInfo {
        command_pool: command_pool.clone(),
        level: dacite::core::CommandBufferLevel::Primary,
        command_buffer_count: 1,
        chain: None,
    };

    let command_buffer = dacite::core::CommandPool::allocate_command_buffers(&allocate_info).map_err(|e| {
        println!("Failed to allocate command buffers ({})", e);
    })?[0].clone();

    let begin_info = dacite::core::CommandBufferBeginInfo {
        flags: dacite::core::CommandBufferUsageFlags::empty(),
        inheritance_info: None,
        chain: None,
    };

    command_buffer.begin(&begin_info).map_err(|e| {
        println!("Failed to begin command buffer ({})", e);
    })?;

    let begin_info = dacite::core::RenderPassBeginInfo {
        render_pass: render_pass.clone(),
        framebuffer: framebuffer.clone(),
        render_area: dacite::core::Rect2D::new(dacite::core::Offset2D::zero(), *extent),
        clear_values: Some(vec![dacite::core::ClearValue::Color(dacite::core::ClearColorValue::Float32([0.0, 0.0, 0.0, 1.0]))]),
        chain: None,
    };

    command_buffer.begin_render_pass(&begin_info, dacite::core::SubpassContents::Inline);
    command_buffer.bind_pipeline(dacite::core::PipelineBindPoint::Graphics, pipeline);
    command_buffer.draw(3, 1, 0, 0);
    command_buffer.end_render_pass();

    command_buffer.pipeline_barrier(dacite::core::PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT, dacite::core::PIPELINE_STAGE_TRANSFER_BIT, dacite::core::DependencyFlags::empty(), None, None, None);

    let regions = vec![dacite::core::BufferImageCopy {
        buffer_offset: 0,
        buffer_row_length: 0,
        buffer_image_height: 0,
        image_subresource: dacite::core::ImageSubresourceLayers {
            aspect_mask: dacite::core::IMAGE_ASPECT_COLOR_BIT,
            mip_level: 0,
            base_array_layer: 0,
            layer_count: 1,
        },
        image_offset: dacite::core::Offset3D::zero(),
        image_extent: dacite::core::Extent3D::from_2d(extent, 1),
    }];

    command_buffer.copy_image_to_buffer(image, dacite::core::ImageLayout::TransferSrcOptimal, buffer, &regions);

    command_buffer.end().map_err(|e| {
        println!("Failed to record command buffer ({})", e);
    })?;

    Ok(command_buffer)
}

fn draw(queue: &dacite::core::Queue, command_buffer: &dacite::core::CommandBuffer) -> Result<(), ()> {
    let submit_infos = vec![dacite::core::SubmitInfo {
        wait_semaphores: vec![],
        wait_dst_stage_mask: vec![],
        command_buffers: vec![command_buffer.clone()],
        signal_semaphores: vec![],
        chain: None,
    }];

    queue.submit(Some(&submit_infos), None).map_err(|e| {
        println!("Failed to submit command buffer ({})", e);
    })?;

    Ok(())
}

fn write_image(memory: &dacite::core::DeviceMemory, extent: &dacite::core::Extent2D) -> Result<(), ()> {
    let file = File::create("triangle-offscreen.png").map_err(|e| {
        println!("Failed to create file triangle-offscreen.png ({})", e);
    })?;

    let mut encoder = png::Encoder::new(file, extent.width, extent.height);
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().map_err(|e| {
        println!("Failed to write png header ({})", e);
    })?;

    let size = extent.width as usize * extent.height as usize;

    let mapped = memory.map(0, dacite::core::OptionalDeviceSize::Size((4 * size) as u64), dacite::core::MemoryMapFlags::empty()).map_err(|e| {
        println!("Failed to map memory ({})", e);
    })?;

    let slice: &[[u8; 4]] = unsafe { slice::from_raw_parts(mapped.as_ptr() as _, size) };
    let mut image_data = Vec::with_capacity(mapped.size() as usize);

    for pixel in slice {
        image_data.push(pixel[2]);
        image_data.push(pixel[1]);
        image_data.push(pixel[0]);
        image_data.push(pixel[3]);
    }

    writer.write_image_data(&image_data).map_err(|e| {
        println!("Failed to write png ({})", e);
    })
}

fn real_main() -> Result<(), ()> {
    let format = dacite::core::Format::B8G8R8A8_UNorm;
    let extent = dacite::core::Extent2D::new(800, 600);

    let instance = create_instance()?;

    let DeviceSettings {
        physical_device,
        queue_family_index,
        memory_types,
    } = find_suitable_device(&instance)?;

    let device = create_device(&physical_device, queue_family_index)?;
    let queue = device.get_queue(queue_family_index, 0);
    let render_pass = create_render_pass(&device, format)?;

    #[allow(unused_variables)]
    let FramebufferSettings {
        image,
        memory: image_memory,
        view: image_view,
        framebuffer,
    } = create_framebuffer(&device, &render_pass, format, &extent, &memory_types)?;

    let BufferSettings {
        buffer,
        memory: buffer_memory,
    } = create_buffer(&device, &extent, &memory_types)?;

    let pipeline = create_pipeline(&device, &render_pass, &extent)?;
    let command_pool = create_command_pool(&device, queue_family_index)?;
    let command_buffer = record_command_buffer(&command_pool, &pipeline, &framebuffer, &render_pass, &image, &buffer, &extent)?;

    draw(&queue, &command_buffer)?;

    device.wait_idle().map_err(|e| {
        println!("Failed to wait for device becoming idle ({})", e);
    })?;

    write_image(&buffer_memory, &extent)?;
    println!("Image was written to triangle-offscreen.png");

    Ok(())
}

fn main() {
    match real_main() {
        Ok(_) => process::exit(0),
        Err(_) => process::exit(1),
    }
}
