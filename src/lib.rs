pub mod config;
mod gpg;

use rand::{rngs::OsRng, RngCore};
use rpassword;
use std::fs;
use std::path::{Path, PathBuf};
use std::str;

use self::config::Config;

pub enum Output {
    Get(PathBuf, Vec<u8>, bool),
    NoOutput,
}

pub enum Error {
    AlreadyInitialized,
    CannotChangeToDir(PathBuf),
    CannotInitRepo,
    FileDoesNotExist(PathBuf),
    InputsDidntMatch,
    NoRepo,
    TargetIsDirectory(PathBuf),
    CannotCreateDirectory(PathBuf),
    UnlockFailed,
    Usage,
}

use Error::*;
use Output::*;

const INIT_MESSAGE: &str = "\
Please pick a master passphrase. It is used to encrypt a very long
random key, which in turn is used to encrypt all the private data in
this directory.

Make sure you remember the master passphrase and that it's strong
enough for your privacy needs.
";

fn read_password(prompt: &str) -> String {
    let result = if atty::is(atty::Stream::Stdin) {
        rpassword::read_password_from_tty(Some(prompt))
    } else {
        rpassword::prompt_password_stderr(prompt)
    };
    match result {
        Ok(password) => password,

        // TODO: Error reading password, handle it somehow?
        Err(_) => String::from(""),
    }
}

fn read_secret(prompt1: &str, prompt2: &str) -> Result<String, Error> {
    let input1 = read_password(prompt1);
    let input2 = read_password(prompt2);
    if input1 == input2 {
        Ok(input1)
    } else {
        Err(InputsDidntMatch)
    }
}

fn unlock_repo() -> Result<Vec<u8>, Error> {
    if !Path::new(".sala/key").is_file() {
        Err(NoRepo)
    } else {
        let passphrase = read_password("Enter the master passphrase: ");
        gpg::decrypt(Path::new(".sala/key"), &passphrase.as_bytes()).map_err(|_| UnlockFailed)
    }
}

pub fn init(repo_path: &Path, config: &Config) -> Result<Output, Error> {
    let key_path = repo_path.join(".sala/key");
    if key_path.exists() {
        return Err(AlreadyInitialized);
    }

    let sala_path = repo_path.join(".sala");
    if !sala_path.exists() {
        fs::create_dir_all(&sala_path).map_err(|_| CannotInitRepo)?
    } else {
        return Err(AlreadyInitialized);
    }
    println!("{}", INIT_MESSAGE);

    let master_passphrase = read_secret("Enter a master passphrase: ", "Confirm: ")?;

    println!("");
    print!("Generating a master key (512 bits)...");
    let mut rng = OsRng::new().unwrap();
    let mut key: [u8; 32] = [0; 32];
    rng.fill_bytes(&mut key);
    let key_ascii: String = key
        .iter()
        .map(|&b| format!("{:x}", b))
        .collect::<Vec<String>>()
        .concat();
    println!(" done");

    gpg::encrypt(
        &key_ascii,
        &master_passphrase.as_bytes(),
        &key_path,
        &config.cipher,
    )
    .unwrap();
    Ok(NoOutput)
}

pub fn get(repo_path: &Path, path: &Path, raw: bool) -> Result<Output, Error> {
    let full_path = repo_path.join(path);

    if !full_path.is_file() {
        return Err(FileDoesNotExist(path.to_path_buf()));
    }
    let master_key = unlock_repo()?;
    let secret = gpg::decrypt(&full_path, &master_key).unwrap();
    Ok(Get(path.to_path_buf(), secret, raw))
}

pub fn set(repo_path: &Path, path: &Path, config: &Config) -> Result<Output, Error> {
    let full_path = repo_path.join(path);
    if let Some(path_parent) = path.parent() {
        fs::create_dir_all(full_path.parent().unwrap())
            .map_err(|_| CannotCreateDirectory(path_parent.to_path_buf()))?
    }

    if full_path.is_dir() {
        return Err(TargetIsDirectory(path.to_path_buf()));
    }
    let master_key = unlock_repo()?;
    let new_secret = read_secret(
        &format!("Type a new secret for {}: ", path.to_string_lossy()),
        "Confirm: ",
    )?;

    gpg::encrypt(&new_secret, &master_key, &full_path, &config.cipher).unwrap();
    Ok(NoOutput)
}

pub fn get_or_set(
    repo_path: &Path,
    path: &Path,
    config: &Config,
    raw: bool,
) -> Result<Output, Error> {
    if repo_path.join(path).exists() {
        get(repo_path, path, raw)
    } else {
        set(repo_path, path, config)
    }
}
