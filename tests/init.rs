use assert_cmd::prelude::*;
use predicates::str::similar;
use std::error::Error;
use std::process::Command;
use tempfile::tempdir;

mod common;
use common::TempRepo;

#[test]
fn init_already_initialized() -> Result<(), Box<Error>> {
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
fn init_passphrases_dont_match() -> Result<(), Box<Error>> {
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
fn init_success() -> Result<(), Box<Error>> {
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
