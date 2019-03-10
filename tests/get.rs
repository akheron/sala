use assert_cmd::prelude::*;
use predicates::str::{contains, similar};
use std::error::Error;
use std::fs;
use std::process::Command;
use tempfile::tempdir;

mod common;
use common::TempRepo;

const DIRECTORY: &str = "foo";
const EXISTING_SECRET: &str = "foo/@bar";
const NON_EXISTING_SECRET: &str = "foo/@new";

#[test]
fn get_no_repo() -> Result<(), Box<Error>> {
    let dir = tempdir()?;
    fs::write(dir.path().join("foo"), "".as_bytes())?;

    Command::cargo_bin("sala")?
        .current_dir(&dir)
        .args(&["get", "foo"])
        .assert()
        .failure()
        .stderr(similar("Run `sala init' first\n"));

    Ok(())
}

#[test]
fn get_wrong_passphrase() -> Result<(), Box<Error>> {
    let repo = TempRepo::new()?;
    Command::cargo_bin("sala")?
        .current_dir(repo.path())
        .args(&["get", EXISTING_SECRET])
        .with_stdin()
        .buffer("this is wrong\n")
        .output()?
        .assert()
        .failure()
        .stderr(similar(
            "\
Enter the master passphrase: 
Error: Unable to unlock the encryption key
",
        ));

    Ok(())
}

#[test]
fn get_not_found() -> Result<(), Box<Error>> {
    let repo = TempRepo::new()?;
    Command::cargo_bin("sala")?
        .current_dir(repo.path())
        .args(&["get", NON_EXISTING_SECRET])
        .assert()
        .failure()
        .stderr(similar(format!(
            "\
Error: File does not exist or invalid: {}
",
            NON_EXISTING_SECRET
        )));

    Ok(())
}

#[test]
fn get_directory() -> Result<(), Box<Error>> {
    let repo = TempRepo::new()?;
    Command::cargo_bin("sala")?
        .current_dir(repo.path())
        .args(&["get", DIRECTORY])
        .assert()
        .failure()
        .stderr(similar(format!(
            "\
Error: File does not exist or invalid: {}
",
            DIRECTORY
        )));

    Ok(())
}

#[test]
fn get_success() -> Result<(), Box<Error>> {
    let repo = TempRepo::new()?;
    Command::cargo_bin("sala")?
        .current_dir(repo.path())
        .args(&["get", EXISTING_SECRET])
        .with_stdin()
        .buffer("qwerty\n")
        .output()?
        .assert()
        .success()
        .stderr(similar("Enter the master passphrase: "))
        .stdout(similar(
            "
foo/@bar: baz

",
        ));

    Ok(())
}

#[test]
fn get_raw() -> Result<(), Box<Error>> {
    let repo = TempRepo::new()?;
    Command::cargo_bin("sala")?
        .current_dir(repo.path())
        .args(&["-r", "get", EXISTING_SECRET])
        .with_stdin()
        .buffer("qwerty\n")
        .output()?
        .assert()
        .success()
        .stderr(similar("Enter the master passphrase: "))
        .stdout(similar("baz\n"));

    Ok(())
}

#[test]
fn implicit_get() -> Result<(), Box<Error>> {
    let repo = TempRepo::new()?;
    Command::cargo_bin("sala")?
        .current_dir(repo.path())
        .arg(EXISTING_SECRET)
        .with_stdin()
        .buffer("qwerty\n")
        .output()?
        .assert()
        .success()
        .stderr(similar("Enter the master passphrase: "))
        .stdout(similar(
            "
foo/@bar: baz

",
        ));

    Ok(())
}

#[test]
fn get_in_dir_does_not_exist() -> Result<(), Box<Error>> {
    let dir = tempdir()?.path().join("foo");
    Command::cargo_bin("sala")?
        .args(&["-C", &dir.to_string_lossy(), "get", EXISTING_SECRET])
        .with_stdin()
        .buffer("qwerty\n")
        .output()?
        .assert()
        .failure()
        .stderr(contains("Error: Cannot change to directory: "));

    Ok(())
}

#[test]
fn get_in_dir_success() -> Result<(), Box<Error>> {
    let repo = TempRepo::new()?;
    Command::cargo_bin("sala")?
        .args(&["-C", &repo.path_string(), "get", EXISTING_SECRET])
        .with_stdin()
        .buffer("qwerty\n")
        .output()?
        .assert()
        .success()
        .stderr(similar("Enter the master passphrase: "))
        .stdout(similar(
            "
foo/@bar: baz

",
        ));

    Ok(())
}

#[test]
fn get_saladir_env() -> Result<(), Box<Error>> {
    let repo = TempRepo::new()?;
    Command::cargo_bin("sala")?
        .env("SALADIR", repo.path_string())
        .args(&["get", EXISTING_SECRET])
        .with_stdin()
        .buffer("qwerty\n")
        .output()?
        .assert()
        .success()
        .stderr(similar("Enter the master passphrase: "))
        .stdout(similar(
            "
foo/@bar: baz

",
        ));

    Ok(())
}
