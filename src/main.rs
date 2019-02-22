use rpassword;
use sala;
use std::path::{Path, PathBuf};
use std::process;
use std::str;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "sala")]
enum Cli {
    #[structopt(name = "init")]
    Init,

    #[structopt(name = "get")]
    Get {
        #[structopt(parse(from_os_str))]
        path: PathBuf,
    },

    #[structopt(name = "set")]
    Set {
        #[structopt(parse(from_os_str))]
        path: PathBuf,
    },
}

fn read_password(prompt: &str) -> String {
    let result = if atty::is(atty::Stream::Stdin) {
        rpassword::read_password_from_tty(Some(prompt))
    } else {
        rpassword::prompt_password_stderr(prompt)
    };
    match result {
        Ok(password) => password,
        Err(err) => {
            eprintln!("failed reading password! {:?}", err);
            String::from("")
        }
    }
}

enum UnlockError {
    NoRepo,
    UnlockFailed,
}

fn unlock_repo() -> Result<Vec<u8>, UnlockError> {
    if !Path::new(".sala/key").is_file() {
        Err(UnlockError::NoRepo)
    } else {
        let passphrase = read_password("Enter the master passphrase: ");
        sala::gpg_decrypt(Path::new(".sala/key"), &passphrase.as_bytes())
            .map_err(|_| UnlockError::UnlockFailed)
    }
}

fn main() {
    let args = Cli::from_args();

    match args {
        Cli::Init => {
            println!("Init");
            return;
        }

        _ => {}
    }

    match args {
        Cli::Get { path } => {
            if !path.is_file() {
                eprintln!(
                    "Error: File does not exist: {}",
                    path.to_str().unwrap_or("<invalid utf8>")
                );
                process::exit(1);
            }
            let master_key = match unlock_repo() {
                Err(UnlockError::NoRepo) => {
                    eprintln!("Run `sala init' first");
                    process::exit(1);
                }

                Err(UnlockError::UnlockFailed) => {
                    eprintln!("");
                    eprintln!("Error: Unable to unlock the encryption key");
                    process::exit(1);
                }

                Ok(key) => key,
            };
            let secret = sala::gpg_decrypt(&path, &master_key).unwrap();
            println!("");
            println!(
                "{}: {}",
                path.to_str().unwrap_or("<invalid utf8>"),
                str::from_utf8(&secret).unwrap_or("<invalid utf8>")
            );
            println!("");
        }

        _ => {}
    }
}
