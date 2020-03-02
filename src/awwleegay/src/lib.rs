#[macro_use]
pub(crate) mod utils;
pub(crate) mod xlog_decode;

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

#[no_mangle]
pub extern "C" fn rust_hello() -> *mut c_char {
    let cstr: CString = CString::new("hello from Rust").expect("CString::new failed");
    let c_char = cstr.into_raw() as *mut c_char;
    c_char
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
    let cstr: CString = CString::new(info).expect("CString::new failed");
    let c_char = cstr.into_raw() as *mut c_char;
    c_char
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
