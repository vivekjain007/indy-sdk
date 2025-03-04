extern crate libc;

use self::libc::c_char;

use std::ffi::CStr;
use std::str::Utf8Error;
use std::ffi::CString;


pub fn c_str_to_string(cstr: *const c_char) -> Result<Option<String>, Utf8Error> {
    if cstr.is_null() {
        return Ok(None);
    }

    unsafe {
        match CStr::from_ptr(cstr).to_str() {
            Ok(str) => Ok(Some(str.to_string())),
            Err(err) => Err(err)
        }
    }
}

pub fn string_to_cstring(s: String) -> CString {
    CString::new(s).unwrap()
}

macro_rules! check_useful_c_str {
    ($x:ident, $e:expr) => {
        let $x = match cstring::c_str_to_string($x) {
            Ok(Some(val)) => val,
            _ => return $e,
        };

        if $x.is_empty() {
            return $e
        }
    }
}

macro_rules! check_useful_opt_c_str {
    ($x:ident, $e:expr) => {
        let $x = match cstring::c_str_to_string($x) {
            Ok(opt_val) => opt_val,
            Err(_) => return $e
        };
    }
}

macro_rules! check_useful_c_byte_array {
    ($ptr:ident, $len:expr, $err1:expr, $err2:expr) => {
        if $ptr.is_null() {
            return $err1;
        }

        if $len <= 0 {
            return $err2;
        }

        let $ptr = unsafe { std::slice::from_raw_parts($ptr, $len as usize) };
        let $ptr = $ptr.to_vec();
    }
}
