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

use ext_debug_report;
use libc::{c_char, c_void};
use std::ffi::CStr;
use std::fmt;
use std::sync::Arc;
use utils;
use vks;

unsafe extern "system" fn debug_report_callback(flags: vks::VkDebugReportFlagsEXT, object_type: vks::VkDebugReportObjectTypeEXT, object: u64, location: usize, message_code: i32, layer_prefix: *const c_char, message: *const c_char, user_data: *mut c_void) -> vks::VkBool32 {
    let callback = user_data as *const Arc<ext_debug_report::DebugReportCallbacksExt>;
    // (*callback).alloc(size, alignment, allocation_scope.into())

    let layer_prefix = if !layer_prefix.is_null() {
        Some(CStr::from_ptr(layer_prefix).to_str().unwrap())
    }
    else {
        None
    };

    let message = if !message.is_null() {
        Some(CStr::from_ptr(message).to_str().unwrap())
    }
    else {
        None
    };

    let res = (*callback).callback(flags, object_type.into(), object, location, message_code, layer_prefix, message);
    utils::to_vk_bool(res)
}

pub struct CallbackHelper {
    pub vks_callback: vks::PFN_vkDebugReportCallbackEXT,
    pub user_data: *mut c_void,
    callback: Arc<Arc<ext_debug_report::DebugReportCallbacksExt>>,
}

impl Clone for CallbackHelper {
    fn clone(&self) -> Self {
        CallbackHelper {
            vks_callback: self.vks_callback,
            user_data: self.user_data,
            callback: self.callback.clone(),
        }
    }
}

impl fmt::Debug for CallbackHelper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("CallbackHelper")
            .field("vks_callback", &(self.vks_callback as *mut c_void))
            .field("user_data", &self.user_data)
            .field("callback", &self.callback)
            .finish()
    }
}

impl CallbackHelper {
    pub fn new(callback: Arc<ext_debug_report::DebugReportCallbacksExt>) -> Self {
        let callback = Arc::new(callback);
        let callback_ptr = Arc::into_raw(callback);
        let callback = unsafe { Arc::from_raw(callback_ptr) };

        CallbackHelper {
            vks_callback: debug_report_callback,
            user_data: callback_ptr as *mut c_void,
            callback: callback,
        }
    }
}
