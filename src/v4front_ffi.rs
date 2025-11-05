// FFI bindings for V4-front compiler library
#![allow(non_camel_case_types)]

use std::os::raw::{c_char, c_int};

// V4-front error codes (from v4front/errors.h)
pub type v4front_err = c_int;

// V4FrontWord - represents a compiled word definition
#[repr(C)]
#[derive(Debug)]
pub struct V4FrontWord {
    pub name: *mut c_char,    // Word name (dynamically allocated)
    pub code: *mut u8,        // Bytecode (dynamically allocated)
    pub code_len: u32,        // Length of bytecode
}

// V4FrontBuf - holds dynamically allocated bytecode output
#[repr(C)]
#[derive(Debug)]
pub struct V4FrontBuf {
    pub words: *mut V4FrontWord,  // Array of compiled words
    pub word_count: c_int,         // Number of words
    pub data: *mut u8,             // Main bytecode
    pub size: usize,               // Size of main bytecode
}

// V4BytecodeHeader - .v4b file format header
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct V4BytecodeHeader {
    pub magic: [u8; 4],      // "V4BC"
    pub version_major: u8,
    pub version_minor: u8,
    pub flags: u16,
    pub code_size: u32,
    pub reserved: u32,
}

unsafe extern "C" {
    // Compile Forth source code
    // Returns 0 on success, negative on error
    pub fn v4front_compile(
        source: *const c_char,
        out_buf: *mut V4FrontBuf,
        err: *mut c_char,
        err_cap: usize,
    ) -> v4front_err;

    // Save compiled bytecode to file
    pub fn v4front_save_bytecode(
        buf: *const V4FrontBuf,
        filename: *const c_char,
    ) -> v4front_err;

    // Free buffer returned by v4front_compile
    pub fn v4front_free(buf: *mut V4FrontBuf);
}

// Safe Rust wrapper for V4-front compiler
pub fn compile_source(source: &str) -> Result<V4FrontBuf, String> {
    use std::ffi::CString;

    let c_source = CString::new(source).map_err(|_| "Invalid source string")?;
    let mut buf = V4FrontBuf {
        words: std::ptr::null_mut(),
        word_count: 0,
        data: std::ptr::null_mut(),
        size: 0,
    };
    let mut err_buf = vec![0u8; 256];

    let result = unsafe {
        v4front_compile(
            c_source.as_ptr(),
            &mut buf as *mut V4FrontBuf,
            err_buf.as_mut_ptr() as *mut c_char,
            err_buf.len(),
        )
    };

    if result != 0 {
        // Find null terminator and convert to String
        let err_len = err_buf.iter().position(|&b| b == 0).unwrap_or(err_buf.len());
        let err_msg = String::from_utf8_lossy(&err_buf[..err_len]).to_string();
        Err(if err_msg.is_empty() {
            format!("Compilation failed with error code {}", result)
        } else {
            err_msg
        })
    } else {
        Ok(buf)
    }
}

pub fn save_bytecode(buf: &V4FrontBuf, path: &std::path::Path) -> Result<(), String> {
    use std::ffi::CString;

    let path_str = path.to_str().ok_or("Invalid path")?;
    let c_path = CString::new(path_str).map_err(|_| "Invalid path string")?;

    let result = unsafe {
        v4front_save_bytecode(buf as *const V4FrontBuf, c_path.as_ptr())
    };

    if result != 0 {
        Err(format!("Failed to save bytecode (error code {})", result))
    } else {
        Ok(())
    }
}

pub fn free_bytecode(mut buf: V4FrontBuf) {
    unsafe {
        v4front_free(&mut buf as *mut V4FrontBuf);
    }
}
