use assert_cmd::prelude::*;
use std::process::Command;
use predicates::prelude::*;

#[test]
fn run_with_defaults() {
    Command::cargo_bin("catsay")
        .expect("binary exists")
        .assert()
        .success()
        .stdout(predicate::str::contains("Meow!"));
}
