use assert_cmd::prelude::*;
use predicates::prelude::*;
use predicates::str::similar;
use std::error::Error;
use std::process::Command;
use tempfile::tempdir;

mod common;
use common::TempRepo;

const DIRECTORY: &str = "foo";
const EXISTING_SECRET: &str = "foo/@bar";
const NON_EXISTING_SECRET: &str = "foo/@new";
const INVALID_SECRET_PATH_DEEP: &str = "foo/@bar/@baz";
const NON_EXISTING_SECRET_DEEP: &str = "foo/bar/baz/@new";

#[test]
fn set_no_repo() -> Result<(), Box<Error>> {
    let dir = tempdir()?;

    Command::cargo_bin("sala")?
        .current_dir(&dir)
        .args(&["set", "foobar"])
        .assert()
        .failure()
        .stderr(similar("Run `sala init' first").trim());

    Ok(())
}

#[test]
fn set_wrong_passphrase() -> Result<(), Box<Error>> {
    let repo = TempRepo::new()?;
    Command::cargo_bin("sala")?
        .current_dir(repo.path())
        .args(&["set", NON_EXISTING_SECRET])
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
fn set_target_is_directory() -> Result<(), Box<Error>> {
    let repo = TempRepo::new()?;
    Command::cargo_bin("sala")?
        .current_dir(repo.path())
        .args(&["set", DIRECTORY])
        .assert()
        .failure()
        .stderr(similar(
            "\
Error: Target is a directory: foo
",
        ));

    Ok(())
}

#[test]
fn set_secrets_dont_match() -> Result<(), Box<Error>> {
    let repo = TempRepo::new()?;
    Command::cargo_bin("sala")?
        .current_dir(repo.path())
        .args(&["set", NON_EXISTING_SECRET])
        .with_stdin()
        .buffer("qwerty\nfoo\nother\n")
        .output()?
        .assert()
        .failure()
        .stderr(similar(
            "\
Enter the master passphrase: Type a new secret for foo/@new: Confirm: 
Inputs did not match.
",
        ));

    assert_eq!(repo.path().join(NON_EXISTING_SECRET).exists(), false);
    Ok(())
}

#[test]
fn set_new_success() -> Result<(), Box<Error>> {
    let repo = TempRepo::new()?;
    Command::cargo_bin("sala")?
        .current_dir(repo.path())
        .args(&["set", NON_EXISTING_SECRET])
        .with_stdin()
        .buffer("qwerty\nfoo\nfoo\n")
        .output()?
        .assert()
        .success()
        .stderr(similar(
            "Enter the master passphrase: Type a new secret for foo/@new: Confirm: ",
        ));

    let secret_path = repo.path().join(NON_EXISTING_SECRET);
    assert_eq!(secret_path.is_file(), true);
    assert_eq!(secret_path.metadata()?.len() > 0, true);
    Ok(())
}

#[test]
fn set_new_cannot_create_parent_dirs() -> Result<(), Box<Error>> {
    let repo = TempRepo::new()?;
    Command::cargo_bin("sala")?
        .current_dir(repo.path())
        .args(&["set", INVALID_SECRET_PATH_DEEP])
        .with_stdin()
        .buffer("qwerty\nfoo\nfoo\n")
        .output()?
        .assert()
        .failure()
        .stderr(similar("Error: Cannot create directory: foo/@bar\n"));

    Ok(())
}

#[test]
fn set_new_creates_parent_dirs() -> Result<(), Box<Error>> {
    let repo = TempRepo::new()?;
    Command::cargo_bin("sala")?
        .current_dir(repo.path())
        .args(&["set", NON_EXISTING_SECRET_DEEP])
        .with_stdin()
        .buffer("qwerty\nfoo\nfoo\n")
        .output()?
        .assert()
        .success()
        .stderr(similar(
            "Enter the master passphrase: Type a new secret for foo/bar/baz/@new: Confirm: ",
        ));

    let secret_path = repo.path().join(NON_EXISTING_SECRET_DEEP);
    assert_eq!(secret_path.is_file(), true);
    assert_eq!(secret_path.metadata()?.len() > 0, true);
    Ok(())
}

#[test]
fn set_replace_existing_success() -> Result<(), Box<Error>> {
    let repo = TempRepo::new()?;
    Command::cargo_bin("sala")?
        .current_dir(repo.path())
        .args(&["set", EXISTING_SECRET])
        .with_stdin()
        .buffer("qwerty\nquux\nquux\n")
        .output()?
        .assert()
        .success()
        .stderr(similar(
            "Enter the master passphrase: Type a new secret for foo/@bar: Confirm: ",
        ));

    let secret_path = repo.path().join(EXISTING_SECRET);
    assert_eq!(secret_path.is_file(), true);
    assert_eq!(secret_path.metadata()?.len() > 0, true);
    Ok(())
}

#[test]
fn implicit_set() -> Result<(), Box<Error>> {
    let repo = TempRepo::new()?;
    Command::cargo_bin("sala")?
        .current_dir(repo.path())
        .arg(NON_EXISTING_SECRET)
        .with_stdin()
        .buffer("qwerty\nfoo\nfoo\n")
        .output()?
        .assert()
        .success()
        .stderr(similar(
            "Enter the master passphrase: Type a new secret for foo/@new: Confirm: ",
        ));

    let secret_path = repo.path().join(NON_EXISTING_SECRET);
    assert_eq!(secret_path.is_file(), true);
    assert_eq!(secret_path.metadata()?.len() > 0, true);
    Ok(())
}
