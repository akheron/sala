use dirs;
use serde::Deserialize;
use std::fs;
use std::path::Path;

pub struct Config {
    pub cipher: String,
    pub key_length: usize,
    pub password_generator: String,
}

impl Config {
    fn merge(self, other: ConfigFromFile) -> Config {
        Config {
            cipher: other.cipher.unwrap_or(self.cipher),
            key_length: other.key_length.unwrap_or(self.key_length),
            password_generator: other.password_generator.unwrap_or(self.password_generator),
        }
    }
}

pub fn default_config() -> Config {
    Config {
        cipher: "AES-128".to_string(),
        key_length: 64,
        password_generator: "pwgen -nc 12 10".to_string(),
    }
}

#[derive(Deserialize)]
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

fn from_file(path: &Path) -> ConfigFromFile {
    match fs::read(path) {
        Ok(contents) => toml::from_slice(&contents).unwrap_or(EMPTY_CONFIG),
        Err(_) => EMPTY_CONFIG,
    }
}

pub fn load(repo_path: &Path) -> Config {
    let home_config = dirs::home_dir()
        .map(|h| from_file(&h.join(".sala.toml")))
        .unwrap_or(EMPTY_CONFIG);
    let xdg_config = dirs::config_dir()
        .map(|h| from_file(&h.join("sala.toml")))
        .unwrap_or(EMPTY_CONFIG);
    let repo_config = from_file(&repo_path.join(".sala/config"));

    default_config()
        .merge(home_config)
        .merge(xdg_config)
        .merge(repo_config)
}
