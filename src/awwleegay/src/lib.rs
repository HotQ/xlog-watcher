#![allow(unused_imports, dead_code)]
extern crate chrono;

#[macro_use]
pub(crate) mod utils;
pub(crate) mod log_filter;
pub(crate) mod xlog_decode;

use log_filter::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use utils::*;

#[no_mangle]
pub extern "C" fn rust_hello() -> *mut c_char {
    string_to_c_str("hello from Rust")
}

#[no_mangle]
pub extern "C" fn rust_capitalize(s: *mut c_char) {
    unsafe {
        let mut p = s as *mut u8;
        while *p != 0 {
            let ch = char::from(*p);
            if ch.is_ascii() {
                let upper = ch.to_ascii_uppercase();
                *p = upper as u8;
            }
            p = p.offset(1);
        }
    }
}

#[no_mangle]
pub extern "C" fn rust_add(a: c_int, b: c_int) -> c_int {
    a + b
}

#[no_mangle]
pub extern "C" fn rust_free_string(raw: *mut c_char) {
    unsafe {
        CString::from_raw(raw);
    }
}

#[no_mangle]
pub extern "C" fn rust_free_vec_string(ptr: *mut [*mut c_char; 2]) {
    let x: Box<[*mut c_char; 2]> = unsafe { Box::from_raw(ptr) };
    rust_free_string(x[0]);
    rust_free_string(x[1]);
}

#[no_mangle]
pub extern "C" fn awwleegay_parse_xlog_to_file(
    src_file: *const c_char,
    dst_file: *const c_char,
) -> *mut c_char {
    let src_file = unsafe { CStr::from_ptr(src_file).to_string_lossy() };
    let dst_file = unsafe { CStr::from_ptr(dst_file).to_string_lossy() };

    let info = match xlog_decode::parse_to_file(&src_file, &dst_file) {
        Ok(_) => String::from("everything is ok"),
        Err(e) => format! {"{:?}",e},
    };
    string_to_c_str(&info)
}

#[no_mangle]
pub extern "C" fn awwleegay_parse_xlog_to_file_tmp(src_file: *const c_char) -> *mut *mut c_char {
    let src_file = unsafe { CStr::from_ptr(src_file).to_string_lossy() };
    let (info, path): (String, String) = match xlog_decode::parse_to_file_tmp(&src_file) {
        Ok(dst_path) => (
            String::from("everything is ok"),
            dst_path.display().to_string(),
        ),
        Err(e) => (format! {"{:?}",e}, String::new()),
    };

    let vec: Box<[*mut c_char; 2]> = Box::new([string_to_c_str(&info), string_to_c_str(&path)]);
    let raw = Box::into_raw(vec) as *mut _;

    raw
}

#[no_mangle]
pub extern "C" fn awwleegay_parse_xlog_to_string(src_file: *const c_char) -> *mut c_char {
    let src_file = unsafe { CStr::from_ptr(src_file).to_string_lossy() };

    let log = match xlog_decode::parse_to_string(&src_file) {
        Ok(log) => log,
        Err(e) => format! {"shit happen when parse xlog to string: {:?}",e},
    };
    string_to_c_str(&log)
}

#[no_mangle]
pub extern "C" fn rust_filter_line(
    command: i64,
    pattern_type: i64,
    pattern: *const c_char,
    file_source: i64,
    source_str: *const c_char,
) -> *mut c_char  {
    let pattern = unsafe { CStr::from_ptr(pattern).to_string_lossy() };
    let source_str = unsafe { CStr::from_ptr(source_str).to_string_lossy() };
    let filter_str = match filter_line(command, pattern_type, &pattern, file_source, &source_str) {
        Ok(s) => s,
        Err(e) => format!("shit happen when excu rust_filter_line {:?}", e),
    };
    string_to_c_str(&filter_str)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
