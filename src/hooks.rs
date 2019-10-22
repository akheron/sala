use dirs;

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};
use std::str;

pub enum Hook {
    PostGet(String),
    PostSet,
}

fn hook_name(hook: &Hook) -> &'static str {
    match hook {
        Hook::PostGet(_) => "post-get",
        Hook::PostSet => "post-set",
    }
}

fn hook_args(hook: &Hook, path: &Path) -> Vec<OsString> {
    match hook {
        Hook::PostGet(secret) => vec![OsString::from(path), OsString::from(secret)],
        Hook::PostSet => vec![OsString::from(path)],
    }
}

fn hook_warning(hook_path: &Path, exit_status: &ExitStatus) -> String {
    let status = match exit_status.code() {
        Some(s) => s.to_string(),
        None => "(signaled)".to_string(),
    };
    return format!(
        "Hook {hook_path:?} failed with status {status}",
        hook_path = hook_path,
        status = status
    );
}

fn run_hook_script(repo_path: &Path, hook_dir: &Path, hook: &Hook, path: &Path) -> Option<String> {
    let hook_path = hook_dir.join(hook_name(hook));

    let result = Command::new(&hook_path)
        .current_dir(repo_path)
        .args(hook_args(hook, path))
        .spawn()
        .and_then(|mut c| c.wait());

    match result {
        Err(_) => {
            // Command not found or not executable, this is fine!
            None
        }
        Ok(exit_status) => {
            if exit_status.success() {
                None
            } else {
                Some(hook_warning(&hook_path, &exit_status))
            }
        }
    }
}

fn get_hook_dirs(repo_path: &Path) -> Vec<PathBuf> {
    let mut result: Vec<PathBuf> = Vec::new();

    result.push(repo_path.join(".sala"));
    dirs::config_dir().map(|dir| result.push(dir.join("sala")));

    result
}

pub fn run_hook(repo_path: &Path, path: &Path, hook: Hook) -> Vec<String> {
    let dirs = get_hook_dirs(repo_path);

    let mut warnings = Vec::new();
    for dir in dirs.iter() {
        run_hook_script(repo_path, &dir, &hook, path).map(|w| warnings.push(w));
    }

    warnings
}
