use std::fs;

use assert_cmd::Command;

#[test]
fn test_n_options() {
    run_test(
        &["-n", "src/main.rs"],
        &"tests/expected/test_n_options.stdout",
    )
}

#[test]
fn test_b_options() {
    run_test(
        &["-b", "src/main.rs"],
        &"tests/expected/test_b_options.stdout",
    )
}

#[test]
fn simple_test() {
    run_test(&["src/main.rs"], &"tests/expected/test_simple.stdout")
}

fn run_test(args: &[&str], path: &str) {
    let mut cmd = Command::cargo_bin("catr").unwrap();
    let op = cmd.args(args).output().unwrap();
    let stdout = String::from_utf8(op.stdout).expect("Invalid Utf8 String");
    assert_eq!(
        stdout,
        fs::read_to_string(path).expect("Impossible to read expected Result file")
    )
}
