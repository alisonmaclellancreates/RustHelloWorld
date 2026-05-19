use std::process::Command;

#[test]
fn binary_prints_hello_world() {
    let binary_path = env!("CARGO_BIN_EXE_rust_hello_world");

    let output = Command::new(binary_path)
        .output()
        .expect("binary should execute successfully");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "Hello, world!\n");
}
