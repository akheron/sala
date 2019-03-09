use assert_cmd::prelude::*;
use predicates::str::similar;
use std::error::Error;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn no_args() -> Result<(), Box<Error>> {
    let dir = tempdir()?;
    Command::cargo_bin("sala")?
        .current_dir(&dir)
        .assert()
        .failure()
        .stderr(similar("Try `sala --help'\n"));

    Ok(())
}
