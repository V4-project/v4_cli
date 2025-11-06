/*!
 * Hardware tests for v4_cli
 *
 * These tests require actual hardware (ESP32-C6) connected via serial port.
 * They are marked with `#[ignore]` so they won't run in CI.
 *
 * ## Running Hardware Tests
 *
 * ```bash
 * # Run all ignored (hardware) tests
 * cargo test -- --ignored
 *
 * # Run specific hardware test
 * cargo test hardware_led_control -- --ignored
 *
 * # With custom port
 * V4_TEST_PORT=/dev/ttyACM0 cargo test -- --ignored
 * ```
 *
 * ## Requirements
 *
 * - ESP32-C6 with V4-runtime flashed
 * - Serial port connection (default: /dev/ttyACM0)
 * - LED connected to GPIO 1
 *
 * ## Environment Variables
 *
 * - `V4_TEST_PORT`: Serial port path (default: /dev/ttyACM0)
 * - `V4_TEST_SKIP_RESET`: Skip VM reset before tests (default: false)
 */

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::thread;
use std::time::Duration;

/// Get serial port from environment or use default
fn get_test_port() -> String {
    env::var("V4_TEST_PORT").unwrap_or_else(|_| "/dev/ttyACM0".to_string())
}

/// Get v4 CLI binary path
fn get_v4_binary() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("target");
    path.push("release");
    path.push("v4");

    // If release binary doesn't exist, try debug
    if !path.exists() {
        path.pop();
        path.pop();
        path.push("debug");
        path.push("v4");
    }

    path
}

/// Create temporary test file
fn create_test_file(name: &str, content: &str) -> PathBuf {
    let mut path = PathBuf::from("/tmp");
    path.push(name);
    fs::write(&path, content).expect("Failed to write test file");
    path
}

/// Run v4 reset command
fn reset_vm(port: &str) -> Result<(), String> {
    if env::var("V4_TEST_SKIP_RESET").is_ok() {
        return Ok(());
    }

    let output = Command::new(get_v4_binary())
        .args(&["reset", "--port", port])
        .output()
        .map_err(|e| format!("Failed to execute reset: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "Reset failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    // Wait for VM to stabilize
    thread::sleep(Duration::from_millis(100));
    Ok(())
}

/// Compile and push bytecode
fn compile_and_push(port: &str, source: &str, name: &str) -> Result<(), String> {
    // Create source file
    let source_path = create_test_file(name, source);

    // Compile
    let v4_bin = get_v4_binary();
    let bytecode_path = source_path.with_extension("v4b");

    let compile_output = Command::new(&v4_bin)
        .args(&[
            "compile",
            source_path.to_str().unwrap(),
            "-o",
            bytecode_path.to_str().unwrap(),
        ])
        .output()
        .map_err(|e| format!("Failed to compile: {}", e))?;

    if !compile_output.status.success() {
        return Err(format!(
            "Compilation failed: {}",
            String::from_utf8_lossy(&compile_output.stderr)
        ));
    }

    // Push bytecode
    let push_output = Command::new(&v4_bin)
        .args(&["push", bytecode_path.to_str().unwrap(), "--port", port])
        .output()
        .map_err(|e| format!("Failed to push: {}", e))?;

    if !push_output.status.success() {
        return Err(format!(
            "Push failed: {}",
            String::from_utf8_lossy(&push_output.stderr)
        ));
    }

    Ok(())
}

/* ========================================================================= */
/* Hardware Tests                                                            */
/* ========================================================================= */

#[test]
#[ignore] // Requires hardware
fn hardware_led_control() {
    let port = get_test_port();

    // Ensure v4 binary exists
    assert!(
        get_v4_binary().exists(),
        "v4 binary not found. Run: cargo build --release"
    );

    println!("Testing LED control on port: {}", port);

    // Reset VM
    reset_vm(&port).expect("Failed to reset VM");

    // Test 1: LED ON
    println!("Test 1: LED ON");
    let led_on_source = r#"
\ LED ON test
: LED_ON  1 1 0 0x0100 SYS DROP ;
LED_ON
"#;

    compile_and_push(&port, led_on_source, "test_led_on.v4").expect("Failed to execute LED_ON");

    println!("✓ LED should be ON now");
    thread::sleep(Duration::from_secs(1));

    // Test 2: LED OFF
    println!("Test 2: LED OFF");
    let led_off_source = r#"
\ LED OFF test
: LED_OFF  1 1 0 0x0101 SYS DROP ;
LED_OFF
"#;

    compile_and_push(&port, led_off_source, "test_led_off.v4")
        .expect("Failed to execute LED_OFF");

    println!("✓ LED should be OFF now");
    thread::sleep(Duration::from_secs(1));
}

#[test]
#[ignore] // Requires hardware
fn hardware_word_shadowing() {
    let port = get_test_port();

    assert!(
        get_v4_binary().exists(),
        "v4 binary not found. Run: cargo build --release"
    );

    println!("Testing word shadowing on port: {}", port);

    // Reset VM
    reset_vm(&port).expect("Failed to reset VM");

    // Test: Define LED_ON to turn LED ON
    println!("Test 1: Initial LED_ON definition (turns LED ON)");
    let initial_source = r#"
: LED_ON  1 1 0 0x0100 SYS DROP ;
LED_ON
"#;

    compile_and_push(&port, initial_source, "test_shadow_1.v4")
        .expect("Failed to execute initial LED_ON");

    println!("✓ LED should be ON");
    thread::sleep(Duration::from_secs(1));

    // Test: Redefine LED_ON to turn LED OFF
    println!("Test 2: Redefine LED_ON (now turns LED OFF)");
    let shadow_source = r#"
\ Redefine LED_ON to turn LED OFF instead
: LED_ON  1 1 0 0x0101 SYS DROP ;
LED_ON
"#;

    compile_and_push(&port, shadow_source, "test_shadow_2.v4")
        .expect("Failed to execute shadowed LED_ON");

    println!("✓ LED should be OFF (due to word shadowing)");
    thread::sleep(Duration::from_secs(1));

    // Test: Third redefinition - turn ON again
    println!("Test 3: Redefine LED_ON again (back to ON)");
    let shadow_source_2 = r#"
: LED_ON  1 1 0 0x0100 SYS DROP ;
LED_ON
"#;

    compile_and_push(&port, shadow_source_2, "test_shadow_3.v4")
        .expect("Failed to execute re-shadowed LED_ON");

    println!("✓ LED should be ON again");
    thread::sleep(Duration::from_secs(1));
}

#[test]
#[ignore] // Requires hardware
fn hardware_multiple_file_loading() {
    let port = get_test_port();

    assert!(
        get_v4_binary().exists(),
        "v4 binary not found. Run: cargo build --release"
    );

    println!("Testing multiple file loading on port: {}", port);

    // Reset VM
    reset_vm(&port).expect("Failed to reset VM");

    // Load file 1: Define words at indices 0, 1
    println!("Loading file 1: Initial word definitions");
    let file1_source = r#"
: WORD1  1 1 0 0x0100 SYS DROP ;
: WORD2  1 1 0 0x0101 SYS DROP ;
WORD1
"#;

    compile_and_push(&port, file1_source, "test_multi_1.v4")
        .expect("Failed to load file 1");
    println!("✓ File 1 loaded (LED ON)");
    thread::sleep(Duration::from_millis(500));

    // Load file 2: Define more words at indices 2, 3, 4
    println!("Loading file 2: Additional words");
    let file2_source = r#"
: WORD3  1 1 0 0x0101 SYS DROP ;
: WORD4  1 1 0 0x0100 SYS DROP ;
: WORD5  1 1 0 0x0101 SYS DROP ;
WORD3
"#;

    compile_and_push(&port, file2_source, "test_multi_2.v4")
        .expect("Failed to load file 2");
    println!("✓ File 2 loaded (LED OFF)");
    thread::sleep(Duration::from_millis(500));

    // Load file 3: Call previously defined word
    println!("Loading file 3: Use existing words");
    let file3_source = r#"
: WORD6  1 1 0 0x0100 SYS DROP ;
WORD6
"#;

    compile_and_push(&port, file3_source, "test_multi_3.v4")
        .expect("Failed to load file 3");
    println!("✓ File 3 loaded (LED ON)");
    thread::sleep(Duration::from_millis(500));
}

#[test]
#[ignore] // Requires hardware
fn hardware_sequential_operations() {
    let port = get_test_port();

    assert!(
        get_v4_binary().exists(),
        "v4 binary not found. Run: cargo build --release"
    );

    println!("Testing sequential LED operations on port: {}", port);

    // Reset VM
    reset_vm(&port).expect("Failed to reset VM");

    // Define helper words
    let setup_source = r#"
: LED_ON  1 1 0 0x0100 SYS DROP ;
: LED_OFF 1 1 0 0x0101 SYS DROP ;
"#;

    compile_and_push(&port, setup_source, "test_seq_setup.v4")
        .expect("Failed to setup");
    println!("✓ Setup complete");

    // Rapid on/off sequence
    for i in 0..3 {
        println!("Cycle {}: ON", i + 1);
        let on_source = "LED_ON";
        compile_and_push(&port, on_source, &format!("test_seq_on_{}.v4", i))
            .expect("Failed ON");
        thread::sleep(Duration::from_millis(300));

        println!("Cycle {}: OFF", i + 1);
        let off_source = "LED_OFF";
        compile_and_push(&port, off_source, &format!("test_seq_off_{}.v4", i))
            .expect("Failed OFF");
        thread::sleep(Duration::from_millis(300));
    }

    println!("✓ Sequential operations complete");
}
