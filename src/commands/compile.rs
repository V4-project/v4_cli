use crate::Result;
use crate::v4front_ffi;
use std::fs;
use std::path::Path;

/// Compile Forth source to V4 bytecode
pub fn compile(input: &str, output: Option<&str>) -> Result<()> {
    // Read source file
    let input_path = Path::new(input);
    if !input_path.exists() {
        return Err(crate::V4Error::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Source file not found: {}", input),
        )));
    }

    let source = fs::read_to_string(input_path)?;
    println!("Compiling {} ({} bytes)...", input, source.len());

    // Determine output filename
    let output_path = if let Some(out) = output {
        Path::new(out).to_path_buf()
    } else {
        // Default: replace .v4 extension with .v4b
        let mut out = input_path.to_path_buf();
        out.set_extension("v4b");
        out
    };

    // Compile source code
    let buf = v4front_ffi::compile_source(&source).map_err(|e| crate::V4Error::Protocol(e))?;

    println!("✓ Compilation successful");

    // Save bytecode to file
    v4front_ffi::save_bytecode(&buf, &output_path).map_err(|e| crate::V4Error::Protocol(e))?;

    // Free the buffer
    v4front_ffi::free_bytecode(buf);

    let output_size = fs::metadata(&output_path)?.len();
    println!(
        "✓ Bytecode saved to {} ({} bytes)",
        output_path.display(),
        output_size
    );

    Ok(())
}
