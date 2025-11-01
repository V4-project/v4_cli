//! Rust FFI bindings for V4 REPL C API
//!
//! This module provides safe Rust wrappers around the libv4repl C API.

use std::ffi::{CStr, CString, c_char, c_int, c_void};
use std::ptr;

// ============================================================================
// V4 VM API
// ============================================================================

#[repr(C)]
pub struct Vm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct VmConfig {
    pub mem: *mut u8,
    pub mem_size: u32,
    pub mmio: *const c_void,
    pub mmio_count: c_int,
    pub arena: *mut c_void,
}

extern "C" {
    fn vm_create(cfg: *const VmConfig) -> *mut Vm;
    fn vm_destroy(vm: *mut Vm);
    fn vm_reset(vm: *mut Vm);
}

// ============================================================================
// V4-front Compiler API
// ============================================================================

#[repr(C)]
pub struct V4FrontContext {
    _private: [u8; 0],
}

extern "C" {
    fn v4front_context_create() -> *mut V4FrontContext;
    fn v4front_context_destroy(ctx: *mut V4FrontContext);
    fn v4front_context_reset(ctx: *mut V4FrontContext);
}

// ============================================================================
// V4-repl API
// ============================================================================

#[repr(C)]
pub struct V4ReplContext {
    _private: [u8; 0],
}

#[repr(C)]
pub struct V4ReplConfig {
    pub vm: *mut Vm,
    pub front_ctx: *mut V4FrontContext,
    pub line_buffer_size: usize,
}

extern "C" {
    fn v4_repl_create(config: *const V4ReplConfig) -> *mut V4ReplContext;
    fn v4_repl_destroy(ctx: *mut V4ReplContext);
    fn v4_repl_process_line(ctx: *mut V4ReplContext, line: *const c_char) -> c_int;
    fn v4_repl_print_stack(ctx: *const V4ReplContext);
    fn v4_repl_get_error(ctx: *const V4ReplContext) -> *const c_char;
    fn v4_repl_reset(ctx: *mut V4ReplContext);
    fn v4_repl_stack_depth(ctx: *const V4ReplContext) -> c_int;
}

// ============================================================================
// Safe Rust Wrapper
// ============================================================================

/// Safe Rust wrapper for V4 REPL
pub struct Repl {
    repl_ctx: *mut V4ReplContext,
    vm: *mut Vm,
    compiler_ctx: *mut V4FrontContext,
    vm_memory: Vec<u8>,
}

impl Repl {
    /// Create a new REPL instance with default configuration
    pub fn new() -> Result<Self, String> {
        Self::with_memory_size(16384) // 16KB default
    }

    /// Create a new REPL instance with specified VM memory size
    pub fn with_memory_size(mem_size: usize) -> Result<Self, String> {
        unsafe {
            // Allocate VM memory
            let mut vm_memory = vec![0u8; mem_size];

            // Create VM
            let vm_config = VmConfig {
                mem: vm_memory.as_mut_ptr(),
                mem_size: mem_size as u32,
                mmio: ptr::null(),
                mmio_count: 0,
                arena: ptr::null_mut(),
            };

            let vm = vm_create(&vm_config);
            if vm.is_null() {
                return Err("Failed to create VM".to_string());
            }

            // Create compiler context
            let compiler_ctx = v4front_context_create();
            if compiler_ctx.is_null() {
                vm_destroy(vm);
                return Err("Failed to create compiler context".to_string());
            }

            // Create REPL context
            let repl_config = V4ReplConfig {
                vm,
                front_ctx: compiler_ctx,
                line_buffer_size: 512,
            };

            let repl_ctx = v4_repl_create(&repl_config);
            if repl_ctx.is_null() {
                v4front_context_destroy(compiler_ctx);
                vm_destroy(vm);
                return Err("Failed to create REPL context".to_string());
            }

            Ok(Repl {
                repl_ctx,
                vm,
                compiler_ctx,
                vm_memory,
            })
        }
    }

    /// Process a single line of Forth code
    pub fn process_line(&mut self, line: &str) -> Result<(), String> {
        unsafe {
            let c_line = CString::new(line).map_err(|e| e.to_string())?;
            let result = v4_repl_process_line(self.repl_ctx, c_line.as_ptr());

            if result != 0 {
                // Get error message
                let err_ptr = v4_repl_get_error(self.repl_ctx);
                if !err_ptr.is_null() {
                    let err_msg = CStr::from_ptr(err_ptr).to_string_lossy().into_owned();
                    return Err(err_msg);
                }
                return Err(format!(
                    "Compilation or execution failed (code: {})",
                    result
                ));
            }

            Ok(())
        }
    }

    /// Print current stack contents to stdout
    pub fn print_stack(&self) {
        unsafe {
            v4_repl_print_stack(self.repl_ctx);
        }
    }

    /// Get current stack depth
    pub fn stack_depth(&self) -> i32 {
        unsafe { v4_repl_stack_depth(self.repl_ctx) }
    }

    /// Reset VM and compiler context
    pub fn reset(&mut self) {
        unsafe {
            v4_repl_reset(self.repl_ctx);
        }
    }
}

impl Drop for Repl {
    fn drop(&mut self) {
        unsafe {
            v4_repl_destroy(self.repl_ctx);
            v4front_context_destroy(self.compiler_ctx);
            vm_destroy(self.vm);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repl_creation() {
        let repl = Repl::new();
        assert!(repl.is_ok());
    }

    #[test]
    fn test_basic_arithmetic() {
        let mut repl = Repl::new().unwrap();
        assert!(repl.process_line("1 2 +").is_ok());
        assert_eq!(repl.stack_depth(), 1);
    }

    #[test]
    fn test_word_definition() {
        let mut repl = Repl::new().unwrap();
        assert!(repl.process_line(": DOUBLE 2 * ;").is_ok());
        assert!(repl.process_line("5 DOUBLE").is_ok());
        assert_eq!(repl.stack_depth(), 1);
    }

    #[test]
    fn test_error_handling() {
        let mut repl = Repl::new().unwrap();
        let result = repl.process_line("UNKNOWN_WORD");
        assert!(result.is_err());
    }
}
