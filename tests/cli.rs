use assert_cmd::prelude::*;
use copy_dir::copy_dir;
use predicates::prelude::*;
use predicates::str::similar;
use std::error::Error;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command;
use tempfile::{tempdir, TempDir};

struct TempRepo {
    dir: TempDir,
}

impl TempRepo {
    fn new() -> io::Result<TempRepo> {
        let dir = tempdir()?;
        copy_dir("tests/repo", dir.path().join("repo"))?;
        Ok(TempRepo { dir })
    }

    fn path(&self) -> PathBuf {
        self.dir.path().join("repo")
    }
}

const DIRECTORY: &str = "foo";
const EXISTING_SECRET: &str = "foo/@bar";
const NON_EXISTING_SECRET: &str = "foo/@new";

#[test]
fn test_no_args() -> Result<(), Box<Error>> {
    let dir = tempdir()?;
    Command::cargo_bin("sala")?
        .current_dir(&dir)
        .assert()
        .failure()
        .stderr(similar("Try `sala --help'\n"));

    Ok(())
}

#[test]
fn test_init_already_initialized() -> Result<(), Box<Error>> {
    let repo = TempRepo::new()?;
    Command::cargo_bin("sala")?
        .current_dir(repo.path())
        .arg("init")
        .output()?
        .assert()
        .failure()
        .stderr(similar("Error: The master key already exists\n"));

    Ok(())
}

#[test]
fn test_init_passphrases_dont_match() -> Result<(), Box<Error>> {
    let dir = tempdir()?;
    Command::cargo_bin("sala")?
        .current_dir(dir.path())
        .arg("init")
        .with_stdin()
        .buffer("foobar\nquux\n")
        .output()?
        .assert()
        .failure()
        .stderr(similar(
            "\
Enter a master passphrase: Confirm: 
Inputs did not match.
",
        ));

    assert_eq!(dir.path().join(".sala/key").exists(), false);
    Ok(())
}

#[test]
fn test_init_success() -> Result<(), Box<Error>> {
    let dir = tempdir()?;
    Command::cargo_bin("sala")?
        .current_dir(dir.path())
        .arg("init")
        .with_stdin()
        .buffer("foobar\nfoobar\n")
        .output()?
        .assert()
        .success()
        .stdout(similar(
            "\
Please pick a master passphrase. It is used to encrypt a very long
random key, which in turn is used to encrypt all the private data in
this directory.

Make sure you remember the master passphrase and that it's strong
enough for your privacy needs.


Generating a master key (512 bits)... done
",
        ))
        .stderr(similar("Enter a master passphrase: Confirm: "));

    assert_eq!(dir.path().join(".sala/key").metadata()?.len() > 0, true);
    Ok(())
}

#[test]
fn test_get_no_repo() -> Result<(), Box<Error>> {
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
fn test_get_wrong_passphrase() -> Result<(), Box<Error>> {
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
fn test_get_not_found() -> Result<(), Box<Error>> {
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
fn test_get_directory() -> Result<(), Box<Error>> {
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
fn test_get_success() -> Result<(), Box<Error>> {
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
fn test_set_no_repo() -> Result<(), Box<Error>> {
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
fn test_set_wrong_passphrase() -> Result<(), Box<Error>> {
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
fn test_set_target_is_directory() -> Result<(), Box<Error>> {
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
fn test_set_secrets_dont_match() -> Result<(), Box<Error>> {
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
fn test_set_new_success() -> Result<(), Box<Error>> {
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
fn test_set_replace_existing_success() -> Result<(), Box<Error>> {
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
fn test_implicit_get() -> Result<(), Box<Error>> {
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
fn test_implicit_set() -> Result<(), Box<Error>> {
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
