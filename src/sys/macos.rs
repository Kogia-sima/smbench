// Copyright (c) 2015 Guillaume Gomez

use std::os::raw::c_char;

fn get_sysctl_str(s: &[u8]) -> Option<String> {
    let mut len = 0;

    unsafe {
        ffi::sysctlbyname(
            s.as_ptr() as *const c_char,
            std::ptr::null_mut(),
            &mut len,
            std::ptr::null_mut(),
            0,
        );
    }

    if len < 1 {
        return None;
    }

    let mut buf = Vec::with_capacity(len);
    unsafe {
        ffi::sysctlbyname(
            s.as_ptr() as *const c_char,
            buf.as_mut_ptr() as _,
            &mut len,
            std::ptr::null_mut(),
            0,
        );
    }

    if len > 0 {
        unsafe {
            buf.set_len(len);
        }
        String::from_utf8(buf).ok()
    } else {
        None
    }
}

pub fn cpu_brand() -> Option<String> {
    get_sysctl_str(b"machdep.cpu.brand_string\0")
}

mod ffi {
    use std::os::raw::{c_char, c_int, c_void};

    extern "C" {
        pub fn sysctlbyname(
            name: *const c_char,
            oldp: *mut c_void,
            oldlenp: *mut usize,
            newp: *mut c_void,
            newlen: usize,
        ) -> c_int;
    }
}
