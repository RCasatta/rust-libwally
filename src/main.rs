extern crate libc;

use std::ffi::CString;
use std::ffi::CStr;
use std::os::raw::c_char;

extern {
    fn wally_is_elements_build() -> libc::c_int;
    //fn bip39_get_languages(output : *const c_char) -> libc::c_int;
}

fn main() {
    let output = unsafe { wally_is_elements_build() };
    println!("is_elements_build:{}", output);

    /*
    let s = CString::new("data data data data").unwrap();

    let output = unsafe { bip39_get_languages(s.as_ref().as_ptr()) };

    println!("mnemonic:{:?}", s);
    */
}
