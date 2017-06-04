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

macro_rules! chain_struct {
    (
        $( #[$struct_attrs:meta] )*
        pub struct $struct_name:ident {
            $(
                $field_name:ident: $field_ty:ident {
                    fn: $field_setter:ident,
                    wrapper: $field_wrapper_ty:ident,
                }
            )*
        }

        $( #[$struct_wrapper_attrs:meta] )*
        struct $struct_wrapper_name:ident;
    ) => (
        $( #[$struct_attrs] )*
        pub struct $struct_name {
            $( $field_name: Option<$field_ty>, )*
        }

        impl $struct_name {
            #[inline]
            pub fn new() -> Self {
                Default::default()
            }

            $(
                #[inline]
                pub fn $field_setter(&mut self, $field_name: $field_ty) -> &mut Self {
                    self.$field_name = Some($field_name);
                    self
                }
            )*
        }

        $( #[$struct_wrapper_attrs] )*
        struct $struct_wrapper_name {
            pub pnext: *const ::libc::c_void,
            $( $field_name: Option<Box<$field_wrapper_ty>>, )*
        }

        impl $struct_wrapper_name {
            pub fn new(from: &$struct_name) -> Self {
                let mut pnext_first: *const ::libc::c_void = ::std::ptr::null();
                let mut pnext: *mut *const ::libc::c_void = &mut pnext_first;

                $(
                    let $field_name = from.$field_name.as_ref().map(|$field_name| {
                        let mut $field_name: Box<_> = Box::new($field_wrapper_ty::new($field_name, false));
                        unsafe {
                            *pnext = &$field_name.vks_struct as *const _ as *const ::libc::c_void;
                            pnext = &mut $field_name.vks_struct.pNext;
                        }

                        $field_name
                    });
                )*

                $struct_wrapper_name {
                    pnext: pnext_first,
                    $( $field_name: $field_name, )*
                }
            }

            #[inline]
            pub fn new_optional(chain: &Option<$struct_name>, with_chain: bool) -> (*const ::libc::c_void, Option<Self>) {
                match *chain {
                    Some(ref chain) if with_chain => {
                        let chain = Self::new(chain);
                        (chain.pnext, Some(chain))
                    }

                    _ => (::std::ptr::null_mut(), None),
                }
            }
        }
    )
}

macro_rules! gen_chain_struct {
    (
        name: $name:ident [$wrapper_name:ident],
        query: $query_name:ident [$query_wrapper_name:ident],
        vks: $vks_ty:ident,
        input: $is_input:ident,
        output: $is_output:ident,

        $(
            $field_name:ident: $field_ty:ident {
                mod: $field_mod:ident,
                fn_add: $field_fn_add:ident,
                fn_has: $field_fn_has:ident,
                fn_get: $field_fn_get:ident,
                wrapper: $field_wrapper_ty:ident,
                vks: $field_vks_ty:ident,
                stype: $field_stype:pat,
            }
        )*
    ) => {
        #[derive(Debug, Clone, Default, PartialEq)]
        pub struct $name {
            $( $field_name: Option<::$field_mod::$field_ty>, )*
            guard: (),
        }

        impl $name {
            #[inline]
            pub fn new() -> Self {
                Default::default()
            }

            $(
                #[inline]
                pub fn $field_fn_add(&mut self, $field_name: ::$field_mod::$field_ty) -> &mut Self {
                    self.$field_name = Some($field_name);
                    self
                }

                #[inline]
                pub fn $field_fn_has(&self) -> bool {
                    self.$field_name.is_some()
                }

                #[inline]
                pub fn $field_fn_get(&self) -> Option<&::$field_mod::$field_ty> {
                    self.$field_name.as_ref()
                }
            )*
        }

        gen_chain_struct! {
            chain_from_pnext

            name: $name [$wrapper_name],
            query: $query_name [$query_wrapper_name],
            vks: $vks_ty,
            input: $is_input,
            output: $is_output,

            $(
                $field_name: $field_ty {
                    mod: $field_mod,
                    fn_add: $field_fn_add,
                    fn_has: $field_fn_has,
                    fn_get: $field_fn_get,
                    wrapper: $field_wrapper_ty,
                    vks: $field_vks_ty,
                    stype: $field_stype,
                }
            )*
        }

        gen_chain_struct! {
            chain_wrapper

            name: $name [$wrapper_name],
            query: $query_name [$query_wrapper_name],
            vks: $vks_ty,
            input: $is_input,
            output: $is_output,

            $(
                $field_name: $field_ty {
                    mod: $field_mod,
                    fn_add: $field_fn_add,
                    fn_has: $field_fn_has,
                    fn_get: $field_fn_get,
                    wrapper: $field_wrapper_ty,
                    vks: $field_vks_ty,
                    stype: $field_stype,
                }
            )*
        }

        gen_chain_struct! {
            query

            name: $name [$wrapper_name],
            query: $query_name [$query_wrapper_name],
            vks: $vks_ty,
            input: $is_input,
            output: $is_output,

            $(
                $field_name: $field_ty {
                    mod: $field_mod,
                    fn_add: $field_fn_add,
                    fn_has: $field_fn_has,
                    fn_get: $field_fn_get,
                    wrapper: $field_wrapper_ty,
                    vks: $field_vks_ty,
                    stype: $field_stype,
                }
            )*
        }
    };

    (
        chain_from_pnext

        name: $name:ident [$wrapper_name:ident],
        query: $query_name:ident [$query_wrapper_name:ident],
        vks: $vks_ty:ident,
        input: $is_input:ident,
        output: true,

        $(
            $field_name:ident: $field_ty:ident {
                mod: $field_mod:ident,
                fn_add: $field_fn_add:ident,
                fn_has: $field_fn_has:ident,
                fn_get: $field_fn_get:ident,
                wrapper: $field_wrapper_ty:ident,
                vks: $field_vks_ty:ident,
                stype: $field_stype:pat,
            }
        )*
    ) => {
        impl $name {
            #[allow(unused_mut)]
            pub(crate) unsafe fn from_pnext(mut pnext: *mut ::libc::c_void) -> Self {
                let mut res = $name::new();

                #[allow(non_snake_case)]
                #[repr(C)]
                pub struct VkStructure {
                    pub sType: vks::VkStructureType,
                    pub pNext: *mut ::libc::c_void,
                }

                while !pnext.is_null() {
                    let tmp = &*(pnext as *const VkStructure);
                    match tmp.sType {
                        $( $field_stype => res.$field_name = Some(::$field_mod::$field_ty::from_vks(&*(pnext as *const vks::$field_vks_ty), false)), )*
                        _ => { }
                    }

                    pnext = tmp.pNext;
                }

                res
            }

            #[inline]
            pub(crate) unsafe fn from_optional_pnext(pnext: *mut ::libc::c_void, with_chain: bool) -> Option<Self> {
                if with_chain && !pnext.is_null() {
                    Some($name::from_pnext(pnext))
                }
                else {
                    None
                }
            }
        }
    };

    (
        chain_from_pnext

        name: $name:ident [$wrapper_name:ident],
        query: $query_name:ident [$query_wrapper_name:ident],
        vks: $vks_ty:ident,
        input: $is_input:ident,
        output: false,

        $(
            $field_name:ident: $field_ty:ident {
                mod: $field_mod:ident,
                fn_add: $field_fn_add:ident,
                fn_has: $field_fn_has:ident,
                fn_get: $field_fn_get:ident,
                wrapper: $field_wrapper_ty:ident,
                vks: $field_vks_ty:ident,
                stype: $field_stype:pat,
            }
        )*
    ) => { };

    (
        chain_wrapper

        name: $name:ident [$wrapper_name:ident],
        query: $query_name:ident [$query_wrapper_name:ident],
        vks: $vks_ty:ident,
        input: true,
        output: $is_output:ident,

        $(
            $field_name:ident: $field_ty:ident {
                mod: $field_mod:ident,
                fn_add: $field_fn_add:ident,
                fn_has: $field_fn_has:ident,
                fn_get: $field_fn_get:ident,
                wrapper: $field_wrapper_ty:ident,
                vks: $field_vks_ty:ident,
                stype: $field_stype:pat,
            }
        )*
    ) => {
        #[derive(Debug)]
        pub(crate) struct $wrapper_name {
            pub pnext: *mut ::libc::c_void,
            $( $field_name: Option<Box<::$field_mod::$field_wrapper_ty>>, )*
        }

        impl $wrapper_name {
            #[allow(unused_mut)]
            #[allow(unused_variables)]
            pub fn new(from: &$name) -> Self {
                let mut pnext_first: *mut ::libc::c_void = ::std::ptr::null_mut();
                let mut pnext: *mut *mut ::libc::c_void = &mut pnext_first;

                $(
                    let $field_name = from.$field_name.as_ref().map(|$field_name| {
                        let mut $field_name: Box<_> = Box::new(::$field_mod::$field_wrapper_ty::new($field_name, false));
                        unsafe {
                            *pnext = &$field_name.vks_struct as *const _ as *mut ::libc::c_void;
                            pnext = &mut ($field_name.vks_struct.pNext as *mut _);
                        }

                        $field_name
                    });
                )*

                $wrapper_name {
                    pnext: pnext_first,
                    $( $field_name: $field_name, )*
                }
            }

            #[inline]
            pub fn new_optional(chain: &Option<$name>, with_chain: bool) -> (*mut ::libc::c_void, Option<Self>) {
                match *chain {
                    Some(ref chain) if with_chain => {
                        let chain = Self::new(chain);
                        (chain.pnext, Some(chain))
                    }

                    _ => (::std::ptr::null_mut(), None),
                }
            }
        }
    };

    (
        chain_wrapper

        name: $name:ident [$wrapper_name:ident],
        query: $query_name:ident [$query_wrapper_name:ident],
        vks: $vks_ty:ident,
        input: false,
        output: $is_output:ident,

        $(
            $field_name:ident: $field_ty:ident {
                mod: $field_mod:ident,
                fn_add: $field_fn_add:ident,
                fn_has: $field_fn_has:ident,
                fn_get: $field_fn_get:ident,
                wrapper: $field_wrapper_ty:ident,
                vks: $field_vks_ty:ident,
                stype: $field_stype:pat,
            }
        )*
    ) => { };

    (
        query

        name: $name:ident [$wrapper_name:ident],
        query: $query_name:ident [$query_wrapper_name:ident],
        vks: $vks_ty:ident,
        input: $is_input:ident,
        output: true,

        $(
            $field_name:ident: $field_ty:ident {
                mod: $field_mod:ident,
                fn_add: $field_fn_add:ident,
                fn_has: $field_fn_has:ident,
                fn_get: $field_fn_get:ident,
                wrapper: $field_wrapper_ty:ident,
                vks: $field_vks_ty:ident,
                stype: $field_stype:pat,
            }
        )*
    ) => {
        #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
        pub struct $query_name {
            $( $field_name: bool, )*
            guard: (),
        }

        impl $query_name {
            #[inline]
            pub fn new() -> Self {
                Default::default()
            }

            $(
                #[inline]
                pub fn $field_fn_add(&mut self) -> &mut Self {
                    self.$field_name = true;
                    self
                }
            )*
        }

        #[derive(Debug, Default)]
        pub(crate) struct $query_wrapper_name {
            pub vks_struct: vks::$vks_ty,
            $( $field_name: Option<Box<vks::$field_vks_ty>>, )*
        }

        impl $query_wrapper_name {
            #[allow(unused_mut)]
            #[allow(unused_variables)]
            pub fn new(query: &$query_name) -> Self {
                let mut vks_struct: vks::$vks_ty = Default::default();
                let mut pnext: *mut *mut ::libc::c_void = &mut (vks_struct.pNext as *mut _);

                $(
                    let $field_name = if query.$field_name {
                        let mut $field_name: Box<vks::$field_vks_ty> = Default::default();
                        unsafe {
                            let $field_name: &mut vks::$field_vks_ty = &mut $field_name;
                            *pnext = $field_name as *mut _ as *mut ::libc::c_void;
                        }
                        pnext = &mut ($field_name.pNext as *mut _);
                        Some($field_name)
                    }
                    else {
                        None
                    };
                )*

                $query_wrapper_name {
                    vks_struct: vks_struct,
                    $( $field_name: $field_name, )*
                }
            }

            #[inline]
            pub fn new_optional(query: Option<&$query_name>) -> Self {
                match query {
                    Some(query) => $query_wrapper_name::new(query),
                    None => Default::default(),
                }
            }
        }
    };

    (
        query

        name: $name:ident [$wrapper_name:ident],
        query: $query_name:ident [$query_wrapper_name:ident],
        vks: $vks_ty:ident,
        input: $is_input:ident,
        output: false,

        $(
            $field_name:ident: $field_ty:ident {
                mod: $field_mod:ident,
                fn_add: $field_fn_add:ident,
                fn_has: $field_fn_has:ident,
                fn_get: $field_fn_get:ident,
                wrapper: $field_wrapper_ty:ident,
                vks: $field_vks_ty:ident,
                stype: $field_stype:pat,
            }
        )*
    ) => { };
}
