use assert_cmd::prelude::*;
use copy_dir::copy_dir;
use predicates::prelude::*;
use predicates::str::{contains, similar};
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

const EXISTING_SECRET: &str = "foo/@bar";
const NON_EXISTING_SECRET: &str = "foo/@new";
const EXISTING_DIRECTORY: &str = "foo";

#[test]
fn test_no_args() -> Result<(), Box<Error>> {
    let dir = tempdir()?;
    Command::cargo_bin("sala")?
        .current_dir(&dir)
        .assert()
        .failure()
        .stderr(contains("USAGE"));

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
        .stderr(similar("Run `sala init' first").trim());

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
Error: File does not exist: {}
",
            NON_EXISTING_SECRET
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
