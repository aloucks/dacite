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
                $( #[$field_guard:meta] )*
                field $field_name:ident: $field_ty:ident {
                    fn: $field_setter:ident,
                    stype: $field_stype:pat,
                    wrapper: $field_wrapper_ty:ident,
                },
            )*
        }

        $( #[$struct_wrapper_attrs:meta] )*
        struct $struct_wrapper_name:ident;
    ) => (
        $( #[$struct_attrs] )*
        pub struct $struct_name {
            $(
                $( #[$field_guard] )*
                pub $field_name: ::std::option::Option<$field_ty>,
            )*
        }

        impl $struct_name {
            #[allow(unused_mut)]
            pub fn new() -> Self {
                unsafe {
                    let mut result: Self = ::std::mem::uninitialized();

                    $(
                        $( #[$field_guard] )*
                        ::std::ptr::write(&mut result.$field_name, None);
                    )*

                    result
                }
            }

            $(
                $( #[$field_guard] )*
                pub fn $field_setter(&mut self, $field_name: $field_ty) -> &mut Self {
                    self.$field_name = Some($field_name);
                    self
                }
            )*
        }

        $( #[$struct_wrapper_attrs] )*
        struct $struct_wrapper_name {
            pub pnext: *const ::libc::c_void,

            $(
                $( #[$field_guard] )*
                $field_name: ::std::option::Option<::std::boxed::Box<$field_wrapper_ty>>,
            )*
        }

        impl $struct_wrapper_name {
            #[allow(unused_variables)]
            #[allow(unused_mut)]
            pub fn new(from: &$struct_name) -> Self {
                let mut pnext_first: *const ::libc::c_void = ::std::ptr::null();
                let mut pnext: *mut *const ::libc::c_void = &mut pnext_first;

                $(
                    $( #[$field_guard] )*
                    let $field_name = from.$field_name.as_ref().map(|$field_name| {
                        let mut $field_name: ::std::boxed::Box<_> = ::std::boxed::Box::new($field_wrapper_ty::new($field_name, false));
                        unsafe {
                            *pnext = &$field_name.vks_struct as *const _ as *const ::libc::c_void;
                            pnext = &mut $field_name.vks_struct.pNext;
                        }

                        $field_name
                    });
                )*

                unsafe {
                    let mut _result: $struct_wrapper_name = ::std::mem::uninitialized();
                    ::std::ptr::write(&mut _result.pnext, pnext_first);

                    $(
                        $( #[$field_guard] )*
                        ::std::ptr::write(&mut _result.$field_name, $field_name);
                    )*

                    _result
                }
            }

            #[inline]
            pub fn new_optional(chain: &::std::option::Option<$struct_name>, with_chain: bool) -> (*const ::libc::c_void, ::std::option::Option<Self>) {
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

// macro_rules! mut_chain_struct {
//     (
//         $( #[$struct_attrs:meta] )*
//         pub struct $struct_name:ident {
//             $(
//                 $( #[$field_guard:meta] )*
//                 field $field_name:ident: $field_ty:ident {
//                     fn: $field_setter:ident,
//                     vk: $field_vk_type:ty,
//                     stype: $field_stype:pat,
//                     wrapper: $field_wrapper_ty:ident,
//                 },
//             )*
//         }

//         $( #[$struct_wrapper_attrs:meta] )*
//         struct $struct_wrapper_name:ident;

//         $( #[$struct_query_attrs:meta] )*
//         pub struct $struct_query_name:ident;

//         $( #[$struct_query_wrapper_attrs:meta] )*
//         struct $struct_query_wrapper_name:ident;
//     ) => (
//         $( #[$struct_attrs] )*
//         pub struct $struct_name {
//             $(
//                 $( #[$field_guard] )*
//                 pub $field_name: ::std::option::Option<$field_ty>,
//             )*
//         }

//         impl $struct_name {
//             #[allow(unused_mut)]
//             pub fn new() -> Self {
//                 unsafe {
//                     let mut result: Self = ::std::mem::uninitialized();

//                     $(
//                         $( #[$field_guard] )*
//                         ::std::ptr::write(&mut result.$field_name, None);
//                     )*

//                     result
//                 }
//             }

//             $(
//                 $( #[$field_guard] )*
//                 pub fn $field_setter(&mut self, $field_name: $field_ty) -> &mut Self {
//                     self.$field_name = Some($field_name);
//                     self
//                 }
//             )*

//             pub unsafe fn from_vks(mut pnext: *mut ::libc::c_void, enable: bool) -> ::std::option::Option<Self> {
//                 if !enable || pnext.is_null() {
//                     return ::std::option::Option::None;
//                 }

//                 $(
//                     $( #[$field_guard] )*
//                     let mut $field_name = ::std::option::Option::None;
//                 )*

//                 loop {
//                     match *(pnext as *const ::vks::VkStructureType) {
//                         $(
//                             $( #[$field_guard] )*
//                             $field_stype => {
//                                 debug_assert!($field_name.is_none());
//                                 $field_name = ::std::option::Option::Some($field_ty::from_vks(&*(pnext as *const _), false));
//                             }
//                         )*

//                         _ => { }
//                     }

//                     pnext = (*(pnext as *const ::chain::VkStructureMut)).pnext;
//                     if pnext.is_null() {
//                         break;
//                     }
//                 }

//                 let mut _result: $struct_name = ::std::mem::uninitialized();

//                 $(
//                     $( #[$field_guard] )*
//                     ::std::ptr::write(&mut _result.$field_name, $field_name);
//                 )*

//                 ::std::option::Option::Some(_result)
//             }
//         }

//         $( #[$struct_wrapper_attrs] )*
//         struct $struct_wrapper_name {
//             pub pnext: *mut ::libc::c_void,

//             $(
//                 $( #[$field_guard] )*
//                 $field_name: ::std::option::Option<::std::boxed::Box<$field_wrapper_ty>>,
//             )*
//         }

//         impl $struct_wrapper_name {
//             #[allow(unused_variables)]
//             #[allow(unused_mut)]
//             pub fn new(from: &$struct_name) -> Self {
//                 let mut pnext_first: *mut ::libc::c_void = ::std::ptr::null_mut();
//                 let mut pnext: *mut *mut ::libc::c_void = &mut pnext_first;

//                 $(
//                     $( #[$field_guard] )*
//                     let $field_name = from.$field_name.as_ref().map(|$field_name| {
//                         let mut $field_name: ::std::boxed::Box<_> = ::std::boxed::Box::new($field_wrapper_ty::new($field_name, false));
//                         unsafe {
//                             *pnext = &$field_name.vks_struct as *const _ as *mut ::libc::c_void;
//                             pnext = &mut $field_name.vks_struct.pNext;
//                         }

//                         $field_name
//                     });
//                 )*

//                 unsafe {
//                     let mut _result: $struct_wrapper_name = ::std::mem::uninitialized();
//                     ::std::ptr::write(&mut _result.pnext, pnext_first);

//                     $(
//                         $( #[$field_guard] )*
//                         ::std::ptr::write(&mut _result.$field_name, $field_name);
//                     )*

//                     _result
//                 }
//             }

//             #[inline]
//             pub fn new_optional(chain: &::std::option::Option<$struct_name>, with_chain: bool) -> (*mut ::libc::c_void, ::std::option::Option<Self>) {
//                 match *chain {
//                     Some(ref chain) if with_chain => {
//                         let chain = Self::new(chain);
//                         (chain.pnext, Some(chain))
//                     }

//                     _ => (::std::ptr::null_mut(), None),
//                 }
//             }
//         }

//         $( #[$struct_query_attrs] )*
//         pub struct $struct_query_name {
//             $(
//                 $( #[$field_guard] )*
//                 pub $field_name: bool,
//             )*
//         }

//         impl $struct_query_name {
//             #[allow(unused_mut)]
//             pub fn new() -> Self {
//                 unsafe {
//                     let mut result: Self = ::std::mem::uninitialized();

//                     $(
//                         $( #[$field_guard] )*
//                         ::std::ptr::write(&mut result.$field_name, false);
//                     )*

//                     result
//                 }
//             }

//             $(
//                 $( #[$field_guard] )*
//                 #[inline]
//                 pub fn $field_setter(&mut self) -> &mut Self {
//                     self.$field_name = true;
//                     self
//                 }
//             )*
//         }

//         $( #[$struct_query_wrapper_attrs] )*
//         struct $struct_query_wrapper_name {
//             pub pnext: *mut ::libc::c_void,

//             $(
//                 $( #[$field_guard] )*
//                 pub $field_name: ::std::option::Option<::std::boxed::Box<$field_vk_type>>,
//             )*
//         }

//         impl $struct_query_wrapper_name {
//             #[allow(unused_variables)]
//             #[allow(unused_mut)]
//             #[allow(unused_assignments)]
//             pub fn new(query: &$struct_query_name) -> Self {
//                 let mut pnext_first: *mut ::libc::c_void = ::std::ptr::null_mut();
//                 let mut pnext: *mut *mut ::libc::c_void = &mut pnext_first;

//                 $(
//                     $( #[$field_guard] )*
//                     let $field_name = if query.$field_name {
//                         unsafe {
//                             let mut $field_name: ::std::boxed::Box<$field_vk_type> = ::std::default::Default::default();
//                             *pnext = &mut *$field_name as *mut $field_vk_type as *mut ::libc::c_void;
//                             pnext = &mut $field_name.pNext;
//                             Some($field_name)
//                         }
//                     }
//                     else {
//                         None
//                     };
//                 )*

//                 unsafe {
//                     let mut _result: Self = ::std::mem::uninitialized();

//                     $(
//                         $( #[$field_guard] )*
//                         ::std::ptr::write(&mut _result.$field_name, $field_name);
//                     )*

//                     _result
//                 }
//             }

//             pub fn new_optional(query: &::std::option::Option<$struct_query_name>) -> (*mut ::libc::c_void, ::std::option::Option<Self>) {
//                 match *query {
//                     ::std::option::Option::Some(ref query) => {
//                         let result = Self::new(query);
//                         (result.pnext, Some(result))
//                     }

//                     ::std::option::Option::None => (::std::ptr::null_mut(), None),
//                 }
//             }
//         }
//     )
// }
