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

extern crate dacite;
extern crate vk_sys;

fn main() {
    let instance_extensions = dacite::core::Instance::enumerate_instance_extension_properties(None).unwrap();
    println!("Instance extension(s) ({}):", instance_extensions.len());
    if !instance_extensions.is_empty() {
        for extension in instance_extensions {
            println!("    {} (revision {})", String::from(extension.extension), extension.spec_version);
        }
    }
    else {
        println!("    None");
    }

    println!();
    let instance_layers = dacite::core::Instance::enumerate_instance_layer_properties().unwrap();
    println!("Instance layer(s) ({}):", instance_layers.len());
    if !instance_layers.is_empty() {
        for layer in instance_layers {
            println!("    {}", layer.layer_name);
            println!("        Specification version: {}", layer.spec_version);
            println!("        Implementation version: {}", layer.implementation_version);
            println!("        Description: {}", layer.description);

            let extensions = dacite::core::Instance::enumerate_instance_extension_properties(Some(&layer.layer_name)).unwrap();
            println!("        Extension(s) ({}):", extensions.len());
            for extension in extensions {
                println!("            {} (revision {})", String::from(extension.extension), extension.spec_version);
            }
        }
    }
    else {
        println!("    None");
        println!();
    }

    let instance_create_info = dacite::core::InstanceCreateInfo {
        flags: vk_sys::VK_INSTANCE_CREATE_DUMMY,
        application_info: Some(dacite::core::ApplicationInfo {
            application_name: Some("dacite info example".to_owned()),
            application_version: 0,
            engine_name: None,
            engine_version: 0,
            api_version: None,
        }),
        enabled_layers: vec![],
        enabled_extensions: vec![],
    };

    let instance = dacite::core::Instance::create(&instance_create_info, None).unwrap();;
    let physical_devices = instance.enumerate_physical_devices().unwrap();

    println!();
    println!("Found {} physical device(s)", physical_devices.len());
}
