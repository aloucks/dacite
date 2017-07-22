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

//! See extension [`VK_EXT_validation_flags`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VK_EXT_validation_flags)

use std::ptr;
use vks;

/// See [`VkValidationCheckEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkValidationCheckEXT)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ValidationCheckExt {
    All,
    Unknown(vks::ext_validation_flags::VkValidationCheckEXT),
}

impl From<ValidationCheckExt> for vks::ext_validation_flags::VkValidationCheckEXT {
    fn from(check: ValidationCheckExt) -> Self {
        match check {
            ValidationCheckExt::All => vks::ext_validation_flags::VK_VALIDATION_CHECK_ALL_EXT,
            ValidationCheckExt::Unknown(check) => check,
        }
    }
}

gen_chain_struct! {
    name: ValidationFlagsChainExt [ValidationFlagsChainExtWrapper],
    query: ValidationFlagsChainQueryExt [ValidationFlagsChainQueryExtWrapper],
    vks: vks::ext_validation_flags::VkValidationFlagsEXT,
    input: true,
    output: false,
}

/// See [`VkValidationFlagsEXT`](https://www.khronos.org/registry/vulkan/specs/1.0-extensions/html/vkspec.html#VkValidationFlagsEXT)
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationFlagsExt {
    pub disabled_validation_checks: Vec<ValidationCheckExt>,
    pub chain: Option<ValidationFlagsChainExt>,
}

#[derive(Debug)]
pub(crate) struct VkValidationFlagsEXTWrapper {
    pub vks_struct: vks::ext_validation_flags::VkValidationFlagsEXT,
    vk_disabled_validation_checks: Vec<vks::ext_validation_flags::VkValidationCheckEXT>,
    chain: Option<ValidationFlagsChainExtWrapper>,
}

impl VkValidationFlagsEXTWrapper {
    pub fn new(flags: &ValidationFlagsExt, with_chain: bool) -> Self {
        let vk_disabled_validation_checks: Vec<_> = flags.disabled_validation_checks.iter().cloned().map(From::from).collect();
        let vk_disabled_validation_checks_ptr = if !vk_disabled_validation_checks.is_empty() {
            vk_disabled_validation_checks.as_ptr() as _
        }
        else {
            ptr::null_mut()
        };

        let (pnext, chain) = ValidationFlagsChainExtWrapper::new_optional(&flags.chain, with_chain);

        VkValidationFlagsEXTWrapper {
            vks_struct: vks::ext_validation_flags::VkValidationFlagsEXT {
                sType: vks::core::VK_STRUCTURE_TYPE_VALIDATION_FLAGS_EXT,
                pNext: pnext,
                disabledValidationCheckCount: vk_disabled_validation_checks.len() as u32,
                pDisabledValidationChecks: vk_disabled_validation_checks_ptr,
            },
            vk_disabled_validation_checks: vk_disabled_validation_checks,
            chain: chain,
        }
    }
}
