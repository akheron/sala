use dirs;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Error {
    pub path: PathBuf,
    pub error: toml::de::Error,
}

pub struct Config {
    pub cipher: String,
    pub key_length: usize,
    pub password_generator: Option<String>,
}

impl Config {
    fn merge(self, other: ConfigFromFile) -> Config {
        Config {
            cipher: other.cipher.unwrap_or(self.cipher),
            key_length: other.key_length.unwrap_or(self.key_length),
            password_generator: other.password_generator.or(self.password_generator),
        }
    }
}

pub fn default_config() -> Config {
    Config {
        cipher: "AES-128".into(),
        key_length: 64,
        password_generator: None,
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
struct ConfigFromFile {
    pub cipher: Option<String>,
    pub key_length: Option<usize>,
    pub password_generator: Option<String>,
}

const EMPTY_CONFIG: ConfigFromFile = ConfigFromFile {
    cipher: None,
    key_length: None,
    password_generator: None,
};

fn from_file(path: &Path) -> Result<ConfigFromFile, Error> {
    match fs::read(path) {
        Ok(contents) => toml::from_slice(&contents).map_err(|e| Error {
            path: path.to_path_buf(),
            error: e,
        }),
        Err(_) => Ok(EMPTY_CONFIG),
    }
}

pub fn load(repo_path: &Path) -> Result<Config, Error> {
    let home_config = dirs::home_dir()
        .map(|h| from_file(&h.join(".sala.toml")))
        .unwrap_or(Ok(EMPTY_CONFIG))?;
    let xdg_config = dirs::config_dir()
        .map(|h| from_file(&h.join("sala.toml")))
        .unwrap_or(Ok(EMPTY_CONFIG))?;
    let repo_config = from_file(&repo_path.join(".sala/config"))?;

    let result = default_config()
        .merge(home_config)
        .merge(xdg_config)
        .merge(repo_config);

    Ok(result)
}
