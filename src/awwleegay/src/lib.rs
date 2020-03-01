#![allow(unused_imports)]

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
