//! Rust FFI bindings for V4-front compiler
//!
//! This module provides safe Rust wrappers around V4-front C API for
//! compiling Forth source code to V4 bytecode.

use std::ffi::{CStr, CString, c_char, c_int};
use std::ptr;
use std::slice;

// ============================================================================
// V4-front Compiler API
// ============================================================================

#[repr(C)]
struct V4FrontContext {
    _private: [u8; 0],
}

#[repr(C)]
struct V4FrontWord {
    name: *mut c_char,
    code: *mut u8,
    code_len: u32,
}

#[repr(C)]
struct V4FrontBuf {
    words: *mut V4FrontWord,
    word_count: c_int,
    data: *mut u8,
    size: usize,
}

unsafe extern "C" {
    fn v4front_context_create() -> *mut V4FrontContext;
    fn v4front_context_destroy(ctx: *mut V4FrontContext);
    fn v4front_context_reset(ctx: *mut V4FrontContext);
    fn v4front_context_register_word(
        ctx: *mut V4FrontContext,
        name: *const c_char,
        vm_word_idx: c_int,
    ) -> c_int;
    fn v4front_compile_with_context(
        ctx: *mut V4FrontContext,
        source: *const c_char,
        out_buf: *mut V4FrontBuf,
        err: *mut c_char,
        err_cap: usize,
    ) -> c_int;
    fn v4front_free(buf: *mut V4FrontBuf);
}

// ============================================================================
// Safe Rust Wrapper
// ============================================================================

/// Compiled word definition
#[derive(Debug, Clone)]
pub struct WordDef {
    pub name: String,
    pub bytecode: Vec<u8>,
}

/// Compilation result
#[derive(Debug)]
pub struct CompileResult {
    pub words: Vec<WordDef>,
    pub bytecode: Vec<u8>,
}

/// Stateful Forth compiler for REPL
pub struct Compiler {
    ctx: *mut V4FrontContext,
    next_word_id: i32,
}

impl Compiler {
    /// Create a new compiler context
    pub fn new() -> Result<Self, String> {
        unsafe {
            let ctx = v4front_context_create();
            if ctx.is_null() {
                return Err("Failed to create compiler context".to_string());
            }

            Ok(Compiler {
                ctx,
                next_word_id: 0,
            })
        }
    }

    /// Compile Forth source code
    ///
    /// Returns compiled bytecode and any word definitions.
    /// Word definitions are automatically registered with the compiler context.
    pub fn compile(&mut self, source: &str) -> Result<CompileResult, String> {
        unsafe {
            let c_source = CString::new(source).map_err(|e| e.to_string())?;
            let mut out_buf = V4FrontBuf {
                words: ptr::null_mut(),
                word_count: 0,
                data: ptr::null_mut(),
                size: 0,
            };
            let mut err_buf = [0u8; 256];

            let result = v4front_compile_with_context(
                self.ctx,
                c_source.as_ptr(),
                &mut out_buf,
                err_buf.as_mut_ptr() as *mut c_char,
                err_buf.len(),
            );

            if result != 0 {
                let err_msg = CStr::from_ptr(err_buf.as_ptr() as *const c_char)
                    .to_string_lossy()
                    .into_owned();
                v4front_free(&mut out_buf);
                return Err(if err_msg.is_empty() {
                    format!("Compilation failed (error code: {})", result)
                } else {
                    err_msg
                });
            }

            // Extract word definitions
            let mut words = Vec::new();
            if !out_buf.words.is_null() && out_buf.word_count > 0 {
                let words_slice = slice::from_raw_parts(out_buf.words, out_buf.word_count as usize);

                for word in words_slice {
                    let name = CStr::from_ptr(word.name).to_string_lossy().into_owned();
                    let bytecode = slice::from_raw_parts(word.code, word.code_len as usize);

                    // Note: We don't register word index here.
                    // The device will register the word and return its index,
                    // which we'll then register via register_word_index()

                    words.push(WordDef {
                        name,
                        bytecode: bytecode.to_vec(),
                    });
                }
            }

            // Extract main bytecode
            let bytecode = if !out_buf.data.is_null() && out_buf.size > 0 {
                slice::from_raw_parts(out_buf.data, out_buf.size).to_vec()
            } else {
                Vec::new()
            };

            v4front_free(&mut out_buf);

            Ok(CompileResult { words, bytecode })
        }
    }

    /// Reset compiler context (clear all registered words)
    pub fn reset(&mut self) {
        unsafe {
            v4front_context_reset(self.ctx);
            self.next_word_id = 0;
        }
    }

    /// Register a word index from device
    ///
    /// Called after device executes bytecode and returns word index
    pub fn register_word_index(&mut self, name: &str, vm_word_idx: i32) -> Result<(), String> {
        unsafe {
            let c_name = CString::new(name).map_err(|e| e.to_string())?;
            let result = v4front_context_register_word(self.ctx, c_name.as_ptr(), vm_word_idx);
            if result < 0 {
                return Err(format!(
                    "Failed to register word '{}' with index {}",
                    name, vm_word_idx
                ));
            }
            Ok(())
        }
    }
}

impl Drop for Compiler {
    fn drop(&mut self) {
        unsafe {
            v4front_context_destroy(self.ctx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compiler_creation() {
        let compiler = Compiler::new();
        assert!(compiler.is_ok());
    }

    #[test]
    fn test_basic_compilation() {
        let mut compiler = Compiler::new().unwrap();
        let result = compiler.compile("1 2 +");
        assert!(result.is_ok());
        let compiled = result.unwrap();
        assert!(!compiled.bytecode.is_empty());
        assert!(compiled.words.is_empty());
    }

    #[test]
    fn test_word_definition() {
        let mut compiler = Compiler::new().unwrap();
        let result = compiler.compile(": DOUBLE 2 * ;");
        assert!(result.is_ok());
        let compiled = result.unwrap();
        assert_eq!(compiled.words.len(), 1);
        assert_eq!(compiled.words[0].name, "DOUBLE");
    }

    #[test]
    fn test_persistent_words() {
        let mut compiler = Compiler::new().unwrap();

        // Define word
        let result1 = compiler.compile(": SQUARE DUP * ;");
        assert!(result1.is_ok());
        let compiled1 = result1.unwrap();
        assert_eq!(compiled1.words.len(), 1);
        assert_eq!(compiled1.words[0].name, "SQUARE");

        // Simulate device registering the word at index 0
        compiler.register_word_index("SQUARE", 0).unwrap();

        // Now we can use the word
        let result2 = compiler.compile("5 SQUARE");
        assert!(result2.is_ok());
    }

    #[test]
    fn test_error_handling() {
        let mut compiler = Compiler::new().unwrap();
        let result = compiler.compile("UNKNOWN_WORD");
        assert!(result.is_err());
    }

    #[test]
    fn test_reset() {
        let mut compiler = Compiler::new().unwrap();

        compiler.compile(": TEST 42 ;").unwrap();
        compiler.reset();

        // After reset, TEST should be unknown
        let result = compiler.compile("TEST");
        assert!(result.is_err());
    }
}
