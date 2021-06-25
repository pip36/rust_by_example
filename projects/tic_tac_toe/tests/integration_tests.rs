use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::{Command, Stdio}; // Run programs

#[test]
fn verify_main_menu() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tic_tac_toe")?;

    let assert = cmd.assert();

    assert
        .success()
        .stdout(predicate::str::contains("Tic Tac Toe"))
        .stdout(predicate::str::contains("1) Play vs Human"));

    let assert = cmd.stdin(Stdio::from("1")).assert();
    Ok(())
}
