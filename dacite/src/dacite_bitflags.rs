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

macro_rules! dacite_bitflags {
    (
        $( #[$attr:meta] )*
        pub struct $flags:ident: $vks:ty;
        pub enum $flag_bits:ident: $vks_flag_bits:ty;
        max_enum: $max_enum:expr;

        flags {
            $(
                $( #[$flag_attr:meta] )*
                const $flag:ident [$flag_bit:ident] = $value:expr;
            )*
        }

        no_bits {
            $(
                $( #[$flag_no_bits_attr:meta] )*
                const $flag_no_bits:ident = $value_no_bits:expr;
            )*
        }
    ) => (
        bitflags! {
            $( #[$attr] )*
            #[derive(Default)]
            pub struct $flags: $vks {
                const MAX_ENUM = $max_enum;

                $(
                    $( #[$flag_attr] )*
                    const $flag = $value;
                )*

                $(
                    $( #[$flag_no_bits_attr] )*
                    const $flag_no_bits = $value_no_bits;
                )*
            }
        }

        impl From<$flag_bits> for $flags {
            fn from(bits: $flag_bits) -> Self {
                match bits {
                    $( $flag_bits::$flag_bit => $flags::$flag, )*
                    $flag_bits::Unknown(bit) => $flags::from_bits_truncate(bit),
                }
            }
        }

        $( #[$attr] )*
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum $flag_bits {
            $( $flag_bit, )*
            Unknown($vks_flag_bits),
        }

        impl $flag_bits {
            pub fn from_flags(flags: $flags) -> Option<Self> {
                match flags {
                    $(
                        $flags::$flag => Some($flag_bits::$flag_bit),
                    )*

                    _ => {
                        if flags.bits().count_ones() == 1 {
                            Some($flag_bits::Unknown(flags.bits()))
                        }
                        else {
                            None
                        }
                    }
                }
            }

            #[inline]
            pub fn from_bits(bits: $vks_flag_bits) -> Option<Self> {
                Self::from_flags($flags::from_bits_truncate(bits))
            }

            /// Convert the flag to the underlying integral type. The result will have only a
            /// single bit set to 1.
            #[inline]
            pub fn bit(&self) -> $vks_flag_bits {
                $flags::from(*self).bits()
            }
        }
    )
}
