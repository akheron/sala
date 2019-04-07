use assert_cmd::prelude::*;
use copy_dir::copy_dir;
use predicates::prelude::*;
use predicates::str::similar;
use std::error::Error;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::{tempdir, TempDir};

// helpers

pub struct TempRepo {
    dir: TempDir,
}

impl TempRepo {
    pub fn new() -> io::Result<TempRepo> {
        let dir = tempdir()?;
        copy_dir("tests/repo", dir.path().join("repo"))?;
        Ok(TempRepo { dir })
    }

    pub fn path(&self) -> PathBuf {
        self.dir.path().join("repo")
    }

    pub fn path_string(&self) -> String {
        self.path().to_string_lossy().to_string()
    }
}

fn run_test<T>(test_fn: T) -> Result<(), Box<Error>>
where
    T: FnOnce(&mut Command, &Path, &TempRepo) -> Result<(), Box<Error>>,
{
    let empty_dir = tempdir()?;
    let repo = TempRepo::new()?;
    let mut cmd = Command::cargo_bin("sala")?;
    cmd.env("HOME", empty_dir.path());
    test_fn(&mut cmd, &empty_dir.path(), &repo)
}

const DIRECTORY: &str = "foo";
const EXISTING_SECRET: &str = "foo/@bar";
const NON_EXISTING_SECRET: &str = "foo/@new";
const INVALID_SECRET_PATH_DEEP: &str = "foo/@bar/@baz";
const NON_EXISTING_SECRET_DEEP: &str = "foo/bar/baz/@new";

// get

#[test]
fn get_no_repo() -> Result<(), Box<Error>> {
    run_test(|cmd, dir, _| {
        fs::write(dir.join("foo"), "".as_bytes())?;

        cmd.current_dir(&dir)
            .args(&["get", "foo"])
            .assert()
            .failure()
            .stderr(similar("No repository. Run `sala init' first\n"));

        Ok(())
    })
}

#[test]
fn get_wrong_passphrase() -> Result<(), Box<Error>> {
    run_test(|cmd, _, repo| {
        cmd.current_dir(repo.path())
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
    })
}

#[test]
fn get_not_found() -> Result<(), Box<Error>> {
    run_test(|cmd, _, repo| {
        cmd.current_dir(repo.path())
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
    })
}

#[test]
fn get_directory() -> Result<(), Box<Error>> {
    run_test(|cmd, _, repo| {
        cmd.current_dir(repo.path())
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
    })
}

#[test]
fn get_success() -> Result<(), Box<Error>> {
    run_test(|cmd, _, repo| {
        cmd.current_dir(repo.path())
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
    })
}

#[test]
fn get_raw() -> Result<(), Box<Error>> {
    run_test(|cmd, _, repo| {
        cmd.current_dir(repo.path())
            .args(&["-r", "get", EXISTING_SECRET])
            .with_stdin()
            .buffer("qwerty\n")
            .output()?
            .assert()
            .success()
            .stderr(similar("Enter the master passphrase: "))
            .stdout(similar("baz\n"));

        Ok(())
    })
}

#[test]
fn implicit_get() -> Result<(), Box<Error>> {
    run_test(|cmd, _, repo| {
        cmd.current_dir(repo.path())
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
    })
}

#[test]
fn get_in_dir_does_not_exist() -> Result<(), Box<Error>> {
    run_test(|cmd, tmpdir, _| {
        let dir = tmpdir.join("foo");
        cmd.args(&["-C", &dir.to_string_lossy(), "get", EXISTING_SECRET])
            .with_stdin()
            .buffer("qwerty\n")
            .output()?
            .assert()
            .failure()
            .stderr(similar(format!(
                "\
Error: File does not exist or invalid: {}
",
                EXISTING_SECRET
            )));

        Ok(())
    })
}

#[test]
fn get_in_dir_success() -> Result<(), Box<Error>> {
    run_test(|cmd, _, repo| {
        cmd.args(&["-C", &repo.path_string(), "get", EXISTING_SECRET])
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
    })
}

#[test]
fn get_saladir_env() -> Result<(), Box<Error>> {
    run_test(|cmd, _, repo| {
        cmd.env("SALADIR", repo.path_string())
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
    })
}

// set

#[test]
fn set_no_repo() -> Result<(), Box<Error>> {
    run_test(|cmd, dir, _| {
        cmd.current_dir(&dir)
            .args(&["set", "foobar"])
            .assert()
            .failure()
            .stderr(similar("No repository. Run `sala init' first").trim());

        Ok(())
    })
}

#[test]
fn set_wrong_passphrase() -> Result<(), Box<Error>> {
    run_test(|cmd, _, repo| {
        cmd.current_dir(repo.path())
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
    })
}

#[test]
fn set_target_is_directory() -> Result<(), Box<Error>> {
    run_test(|cmd, _, repo| {
        cmd.current_dir(repo.path())
            .args(&["set", DIRECTORY])
            .assert()
            .failure()
            .stderr(similar(
                "\
Error: Target is a directory: foo
",
            ));

        Ok(())
    })
}

#[test]
fn set_secrets_dont_match() -> Result<(), Box<Error>> {
    run_test(|cmd, _, repo| {
        cmd.current_dir(repo.path())
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
    })
}

#[test]
fn set_new_success() -> Result<(), Box<Error>> {
    run_test(|cmd, _, repo| {
        cmd.current_dir(repo.path())
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
    })
}

#[test]
fn set_new_cannot_create_parent_dirs() -> Result<(), Box<Error>> {
    run_test(|cmd, _, repo| {
        cmd.current_dir(repo.path())
            .args(&["set", INVALID_SECRET_PATH_DEEP])
            .with_stdin()
            .buffer("qwerty\nfoo\nfoo\n")
            .output()?
            .assert()
            .failure()
            .stderr(similar("Error: Cannot create directory: foo/@bar\n"));

        Ok(())
    })
}

#[test]
fn set_new_creates_parent_dirs() -> Result<(), Box<Error>> {
    run_test(|cmd, _, repo| {
        cmd.current_dir(repo.path())
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
    })
}

#[test]
fn set_replace_existing_success() -> Result<(), Box<Error>> {
    run_test(|cmd, _, repo| {
        cmd.current_dir(repo.path())
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
    })
}

#[test]
fn implicit_set() -> Result<(), Box<Error>> {
    run_test(|cmd, _, repo| {
        cmd.current_dir(repo.path())
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
    })
}

// init

#[test]
fn init_already_initialized() -> Result<(), Box<Error>> {
    run_test(|cmd, _, repo| {
        cmd.current_dir(repo.path())
            .arg("init")
            .output()?
            .assert()
            .failure()
            .stderr(similar("Error: The master key already exists\n"));

        Ok(())
    })
}

#[test]
fn init_passphrases_dont_match() -> Result<(), Box<Error>> {
    run_test(|cmd, dir, _| {
        cmd.current_dir(dir)
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

        assert_eq!(dir.join(".sala/key").exists(), false);
        Ok(())
    })
}

#[test]
fn init_success() -> Result<(), Box<Error>> {
    run_test(|cmd, dir, _| {
        cmd.current_dir(dir)
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

        assert_eq!(dir.join(".sala/key").metadata()?.len() > 0, true);
        Ok(())
    })
}

// misc

#[test]
fn no_args() -> Result<(), Box<Error>> {
    run_test(|cmd, dir, _| {
        cmd.current_dir(dir)
            .assert()
            .failure()
            .stderr(similar("Try `sala --help'\n"));

        Ok(())
    })
}
