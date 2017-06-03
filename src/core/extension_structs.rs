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

macro_rules! gen_extension_structs {
    (
        pub struct $name:ident;
        pub struct $props_name:ident;

        $(
            $ext:ident {
                name: $ext_name:expr,
                fn_add: $ext_fn_add:ident,
                fn_has: $ext_fn_has:ident,
                fn_get: $ext_fn_get:ident,
                $( load_instance: $ext_load_instance:ident, )*
                $( load_device: $ext_load_device:ident, )*
            }
        )*
    ) => (
        #[derive(Debug, Clone, PartialEq, Eq, Default)]
        pub struct $name {
            named: HashSet<String>,
            $( $ext: bool, )*
        }

        impl $name {
            pub fn new() -> Self {
                Default::default()
            }

            #[inline]
            pub fn len(&self) -> usize {
                let mut res = self.named.len();
                $( if self.$ext { res += 1; } )*
                res
            }

            #[inline]
            pub fn is_empty(&self) -> bool {
                if !self.named.is_empty() {
                    return false;
                }

                $(
                    if self.$ext {
                        return false;
                    }
                )*

                true
            }

            pub fn difference(&self, other: &Self) -> Self {
                $name {
                    named: self.named.difference(&other.named).cloned().collect(),
                    $( $ext: self.$ext && !other.$ext, )*
                }
            }

            pub fn intersection(&self, other: &Self) -> Self {
                $name {
                    named: self.named.intersection(&other.named).cloned().collect(),
                    $( $ext: self.$ext && other.$ext, )*
                }
            }

            pub fn union(&self, other: &Self) -> Self {
                $name {
                    named: self.named.union(&other.named).cloned().collect(),
                    $( $ext: self.$ext || other.$ext, )*
                }
            }

            pub fn names(&self) -> Vec<&str> {
                let mut res: Vec<_> = self.named.iter().map(String::as_str).collect();
                $( if self.$ext { res.push($ext_name); } )*
                res
            }

            pub fn add_named(&mut self, name: &str) -> &mut Self {
                $(
                    if name == $ext_name {
                        return self.$ext_fn_add();
                    }
                )*

                self.named.insert(name.to_owned());
                self
            }

            pub fn has_named(&self, name: &str) -> bool {
                $(
                    if name == $ext_name {
                        return self.$ext_fn_has();
                    }
                )*

                self.named.contains(name)
            }

            $(
                pub fn $ext_fn_add(&mut self) -> &mut Self {
                    self.$ext = true;
                    self
                }

                pub fn $ext_fn_has(&self) -> bool {
                    self.$ext
                }
            )*

            #[allow(dead_code)]
            #[allow(unused_variables)]
            unsafe fn load_instance(&self, loader: &mut vks::InstanceProcAddrLoader, instance: vks::VkInstance) {
                $(
                    $( loader.$ext_load_instance(instance); )*
                )*
            }

            #[allow(dead_code)]
            #[allow(unused_variables)]
            unsafe fn load_device(&self, loader: &mut vks::DeviceProcAddrLoader, device: vks::VkDevice) {
                $(
                    $( loader.$ext_load_device(device); )*
                )*
            }

            fn to_cstring_vec(&self) -> Vec<CString> {
                let mut res: Vec<_> = self.named
                    .iter()
                    .cloned()
                    .map(CString::new)
                    .map(Result::unwrap)
                    .collect();

                $(
                    if self.$ext {
                        res.push(CString::new($ext_name.to_owned()).unwrap());
                    }
                )*

                res
            }
        }

        #[derive(Debug, Clone, PartialEq, Eq, Default)]
        pub struct $props_name {
            named: HashMap<String, u32>,
            $( $ext: Option<u32>, )*
        }

        impl $props_name {
            pub fn new() -> Self {
                Default::default()
            }

            #[inline]
            pub fn len(&self) -> usize {
                let mut res = self.named.len();
                $( if self.$ext.is_some() { res += 1; } )*
                res
            }

            #[inline]
            pub fn is_empty(&self) -> bool {
                if !self.named.is_empty() {
                    return false;
                }

                $(
                    if self.$ext.is_some() {
                        return false;
                    }
                )*

                true
            }

            pub fn difference(&self, other: &Self) -> Self {
                let named = self.named
                    .iter()
                    .filter(|&(name, spec_version)| {
                        match other.named.get(name) {
                            Some(other_spec_version) => spec_version > other_spec_version,
                            None => true,
                        }
                    })
                    .map(|(n, v)| (n.clone(), v.clone()))
                    .collect();

                $(
                    let $ext = match (self.$ext, other.$ext) {
                        (Some(spec_version), Some(other_spec_version)) if spec_version > other_spec_version => Some(spec_version),
                        (Some(spec_version), None) => Some(spec_version),
                        _ => None,
                    };
                )*

                $props_name {
                    named: named,
                    $( $ext: $ext, )*
                }
            }

            pub fn intersection(&self, other: &Self) -> Self {
                let named = self.named
                    .iter()
                    .filter_map(|(name, &spec_version)| {
                        match other.named.get(name) {
                            Some(&other_spec_version) => Some((name.clone(), cmp::min(spec_version, other_spec_version))),
                            None => None,
                        }
                    })
                    .collect();

                $(
                    let $ext = match (self.$ext, other.$ext) {
                        (Some(spec_version), Some(other_spec_version)) => Some(cmp::min(spec_version, other_spec_version)),
                        _ => None,
                    };
                )*

                $props_name {
                    named: named,
                    $( $ext: $ext, )*
                }
            }

            pub fn union(&self, other: &Self) -> Self {
                let mut res = self.clone();
                for (name, &spec_version) in &other.named {
                    res.add_named(name, spec_version);
                }

                $(
                    if let Some(spec_version) = other.$ext {
                        res.$ext_fn_add(spec_version);
                    }
                )*

                res
            }

            pub fn to_extensions(&self) -> $name {
                $name {
                    named: self.named.keys().cloned().collect(),
                    $( $ext: self.$ext.is_some(), )*
                }
            }

            pub fn names(&self) -> Vec<&str> {
                let mut res: Vec<_> = self.named.keys().map(String::as_str).collect();
                $( if self.$ext.is_some() { res.push($ext_name); } )*
                res
            }

            pub fn properties(&self) -> Vec<(&str, u32)> {
                let mut res: Vec<_> = self.named.iter().map(|(n, v)| (n.as_str(), *v)).collect();
                $( if let Some(v) = self.$ext { res.push(($ext_name, v)); } )*
                res
            }

            pub fn add_named(&mut self, name: &str, spec_version: u32) -> &mut Self {
                $(
                    if name == $ext_name {
                        return self.$ext_fn_add(spec_version);
                    }
                )*

                {
                    let v = self.named.entry(name.to_owned()).or_insert(spec_version);
                    if *v < spec_version {
                        *v = spec_version;
                    }
                }

                self
            }

            pub fn has_named(&self, name: &str) -> bool {
                self.get_named(name).is_some()
            }

            pub fn get_named(&self, name: &str) -> Option<u32> {
                $(
                    if name == $ext_name {
                        return self.$ext_fn_get();
                    }
                )*

                self.named.get(name).cloned()
            }

            $(
                pub fn $ext_fn_add(&mut self, spec_version: u32) -> &mut Self {
                    if let Some(ref mut old_spec_version) = self.$ext {
                        if *old_spec_version < spec_version {
                            *old_spec_version = spec_version;
                        }
                    }
                    else {
                        self.$ext = Some(spec_version);
                    }

                    self
                }

                pub fn $ext_fn_has(&self) -> bool {
                    self.$ext.is_some()
                }

                pub fn $ext_fn_get(&self) -> Option<u32> {
                    self.$ext
                }
            )*
        }
    )
}
