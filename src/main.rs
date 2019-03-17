use clap::{App, Arg, SubCommand};
use sala::{
    config,
    Error::{self, *},
    Output::{self, *},
};
use std::env;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

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

    let repo_path = PathBuf::from(
        app_m
            .value_of_os("directory")
            .as_ref()
            .map(|x| x.to_os_string())
            .or_else(|| env::var_os("SALADIR"))
            .unwrap_or(OsString::from(".")),
    );

    let config = config::load(&repo_path);

    if let Err(_) = env::set_current_dir(&repo_path) {
        print_error(&sala::Error::CannotChangeToDir(PathBuf::from(&repo_path)));
        std::process::exit(1);
    }

    let raw = app_m.is_present("raw");
    let result = match (app_m.subcommand(), app_m.value_of_os("path")) {
        (("init", Some(_)), _) => sala::init(&repo_path, &config),
        (("get", Some(sub_m)), _) => sala::get(
            &repo_path,
            Path::new(sub_m.value_of_os("path").unwrap()),
            raw,
        ),
        (("set", Some(sub_m)), _) => sala::set(
            &repo_path,
            Path::new(sub_m.value_of_os("path").unwrap()),
            &config,
        ),
        (_, Some(path)) => sala::get_or_set(&repo_path, Path::new(path), &config, raw),
        _ => Err(sala::Error::Usage),
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
