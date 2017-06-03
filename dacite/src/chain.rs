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
            #[allow(unused_variables)]
            #[allow(unused_mut)]
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
