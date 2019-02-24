use clap::{App, Arg, ArgMatches, SubCommand};
use rpassword;
use sala;
use std::path::{Path, PathBuf};
use std::process;
use std::str;

enum SalaResult {
    AllesGut,
    FileDoesNotExist(PathBuf),
    InputsDontMatch,
    NoRepo,
    Get(PathBuf, Vec<u8>),
    TargetIsDirectory(PathBuf),
    UnlockFailed,
    Usage,
}

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

fn read_secret(prompt1: &str, prompt2: &str) -> Result<String, SalaResult> {
    let input1 = read_password(prompt1);
    let input2 = read_password(prompt2);
    if input1 == input2 {
        Ok(input1)
    } else {
        Err(SalaResult::InputsDontMatch)
    }
}

fn unlock_repo() -> Result<Vec<u8>, SalaResult> {
    if !Path::new(".sala/key").is_file() {
        Err(SalaResult::NoRepo)
    } else {
        let passphrase = read_password("Enter the master passphrase: ");
        sala::gpg_decrypt(Path::new(".sala/key"), &passphrase.as_bytes())
            .map_err(|_| SalaResult::UnlockFailed)
    }
}

fn main() {
    let app_m = App::new("sala")
        .version("1.4")
        .about("Store passwords and other sensitive information to plain text files")
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
        .get_matches();

    exit_with_output(&run_subcommand(app_m.subcommand()));
}

fn run_subcommand(sub: (&str, Option<&ArgMatches>)) -> SalaResult {
    use SalaResult::*;

    match sub {
        ("init", Some(_)) => {
            println!("init");
            return AllesGut;
        }

        ("get", Some(sub_m)) => {
            let path_arg = sub_m.value_of_os("path").unwrap();
            let path = Path::new(path_arg).to_path_buf();

            if !path.is_file() {
                return FileDoesNotExist(path);
            }
            let master_key = match unlock_repo() {
                Err(res) => return res,
                Ok(key) => key,
            };
            let secret = sala::gpg_decrypt(&path, &master_key).unwrap();

            Get(path, secret)
        }

        ("set", Some(sub_m)) => {
            let path_arg = sub_m.value_of_os("path").unwrap();
            let path = Path::new(path_arg).to_path_buf();

            if path.is_dir() {
                return TargetIsDirectory(path);
            }
            let master_key = match unlock_repo() {
                Err(err) => return err,
                Ok(key) => key,
            };
            let new_secret = match read_secret(
                &format!("Type a new secret for {}: ", path.to_string_lossy()),
                "Confirm: ",
            ) {
                Err(res) => return res,
                Ok(secret) => secret,
            };

            sala::gpg_encrypt(&new_secret, &master_key, &path).unwrap();
            AllesGut
        }

        _ => Usage,
    }
}

fn exit_with_output(result: &SalaResult) -> ! {
    use SalaResult::*;

    process::exit(match result {
        FileDoesNotExist(path) => {
            eprintln!(
                "Error: File does not exist or invalid: {}",
                path.to_string_lossy()
            );
            1
        }
        InputsDontMatch => {
            eprintln!("");
            eprintln!("Inputs did not match.");
            1
        }
        NoRepo => {
            eprintln!("Run `sala init' first");
            1
        }
        UnlockFailed => {
            eprintln!("");
            eprintln!("Error: Unable to unlock the encryption key");
            1
        }
        Usage => {
            eprintln!("Try `sala --help'");
            1
        }
        Get(path, secret) => {
            println!("");
            println!(
                "{}: {}",
                path.to_string_lossy(),
                String::from_utf8_lossy(&secret)
            );
            println!("");
            0
        }
        TargetIsDirectory(path) => {
            eprintln!("Error: Target is a directory: {}", path.to_string_lossy());
            1
        }
        AllesGut => 0,
    })
}
