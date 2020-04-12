#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::{CString, c_void};

mod ffi;

pub use ffi::*;

pub fn test_main() {
    unsafe {
        let argv: [CString; 0] = [];
        let hlfile = CString::new("").expect("could not convert file contents to CString");

        hl_global_init();
        hl_sys_init(argv.as_ptr() as *mut *mut c_void, argv.len() as i32, hlfile.as_ptr() as *mut c_void);


        let fdata = CString::new("").unwrap();
        let error_msg = CString::new("").unwrap();
        
        // let code = hl_code_read(fdata.as_ptr() as *const u8, 0, error_msg.as_ptr() as *mut *mut i8);
        // dbg!(code);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        unsafe {
            floorf(5.5);
        }
    }
}
