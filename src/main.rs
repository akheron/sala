use clap::{App, Arg, SubCommand};
use rand::{rngs::OsRng, RngCore};
use rpassword;
use sala;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::str;

enum Output {
    Get(PathBuf, Vec<u8>, bool),
    NoOutput,
}
enum Error {
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
        sala::gpg_decrypt(Path::new(".sala/key"), &passphrase.as_bytes()).map_err(|_| UnlockFailed)
    }
}

fn main() {
    let app_m = App::new("sala")
        .version("1.4")
        .about("Store passwords and other sensitive information to plain text files")
        .arg(
            Arg::with_name("raw")
                .short("r")
                .long("raw")
                .global(true)
                .help("Use a simple output format for machine processing"),
        )
        .arg(
            Arg::with_name("directory")
                .short("C")
                .long("directory")
                .takes_value(true)
                .value_name("DIR")
                .global(true)
                .help("Use the password repository in DIR instead of current directory"),
        )
        .subcommand(
            SubCommand::with_name("init")
                .display_order(0)
                .about("Initialize a repository"),
        )
        .subcommand(
            SubCommand::with_name("get")
                .display_order(1)
                .about("Read a secret")
                .arg(
                    Arg::with_name("path")
                        .required(true)
                        .help("Path of the file to read"),
                ),
        )
        .subcommand(
            SubCommand::with_name("set")
                .display_order(2)
                .about("Create or update a secret")
                .arg(
                    Arg::with_name("path")
                        .required(true)
                        .help("Path of the file to write"),
                ),
        )
        .arg(Arg::with_name("path").hidden(true))
        .get_matches();

    if let Some(cwd) = app_m
        .value_of_os("directory")
        .as_ref()
        .map(|x| x.to_os_string())
        .or_else(|| env::var_os("SALADIR"))
    {
        if let Err(_) = env::set_current_dir(&cwd) {
            print_error(&CannotChangeToDir(PathBuf::from(&cwd)));
            std::process::exit(1);
        }
    }

    let raw = app_m.is_present("raw");
    let result = match (app_m.subcommand(), app_m.value_of_os("path")) {
        (("init", Some(_)), _) => command_init(),
        (("get", Some(sub_m)), _) => command_get(sub_m.value_of_os("path").unwrap(), raw),
        (("set", Some(sub_m)), _) => command_set(sub_m.value_of_os("path").unwrap()),
        (_, Some(path)) => command_get_or_set(path, raw),
        _ => Err(Usage),
    };

    match result {
        Ok(output) => {
            print_output(&output);
        }
        Err(error) => {
            print_error(&error);
            std::process::exit(1);
        }
    };
}

fn command_init() -> Result<Output, Error> {
    let key_path = Path::new(".sala/key");
    if key_path.exists() {
        return Err(AlreadyInitialized);
    }

    let sala_path = Path::new(".sala");
    if !sala_path.exists() {
        fs::create_dir(&sala_path).map_err(|_| CannotInitRepo)?
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

    sala::gpg_encrypt(&key_ascii, &master_passphrase.as_bytes(), &key_path).unwrap();
    Ok(NoOutput)
}

fn command_get(path_arg: &OsStr, raw: bool) -> Result<Output, Error> {
    let path = Path::new(path_arg).to_path_buf();

    if !path.is_file() {
        return Err(FileDoesNotExist(path));
    }
    let master_key = unlock_repo()?;
    let secret = sala::gpg_decrypt(&path, &master_key).unwrap();
    Ok(Get(path, secret, raw))
}

fn command_set(path_arg: &OsStr) -> Result<Output, Error> {
    let path = Path::new(path_arg).to_path_buf();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|_| CannotCreateDirectory(parent.to_path_buf()))?
    }

    if path.is_dir() {
        return Err(TargetIsDirectory(path));
    }
    let master_key = unlock_repo()?;
    let new_secret = read_secret(
        &format!("Type a new secret for {}: ", path.to_string_lossy()),
        "Confirm: ",
    )?;

    sala::gpg_encrypt(&new_secret, &master_key, &path).unwrap();
    Ok(NoOutput)
}

fn command_get_or_set(path_arg: &OsStr, raw: bool) -> Result<Output, Error> {
    let path = Path::new(path_arg).to_path_buf();
    if path.exists() {
        command_get(path_arg, raw)
    } else {
        command_set(path_arg)
    }
}

fn print_output(output: &Output) {
    match output {
        Get(path, secret, raw) => {
            let secret_utf8 = String::from_utf8_lossy(&secret);
            if *raw {
                println!("{}", secret_utf8);
            } else {
                println!("");
                println!("{}: {}", path.to_string_lossy(), secret_utf8,);
                println!("");
            }
        }
        NoOutput => {}
    }
}

fn print_error(error: &Error) {
    match error {
        AlreadyInitialized => {
            eprintln!("Error: The master key already exists");
        }
        CannotInitRepo => {
            eprintln!("Error: Failed to initialize a new repository");
        }
        FileDoesNotExist(path) => {
            eprintln!(
                "Error: File does not exist or invalid: {}",
                path.to_string_lossy()
            );
        }
        CannotChangeToDir(path) => {
            eprintln!(
                "Error: Cannot change to directory: {}",
                path.to_string_lossy()
            );
        }
        InputsDidntMatch => {
            eprintln!("");
            eprintln!("Inputs did not match.");
        }
        NoRepo => {
            eprintln!("Run `sala init' first");
        }
        UnlockFailed => {
            eprintln!("");
            eprintln!("Error: Unable to unlock the encryption key");
        }
        Usage => {
            eprintln!("Try `sala --help'");
        }
        TargetIsDirectory(path) => {
            eprintln!("Error: Target is a directory: {}", path.to_string_lossy());
        }
        CannotCreateDirectory(path) => {
            eprintln!("Error: Cannot create directory: {}", path.to_string_lossy());
        }
    }
}
