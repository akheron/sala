use nix::{
    self,
    fcntl::{self, FcntlArg, FdFlag},
    unistd,
};
use std::io;
use std::path::Path;
use std::process::{Command, Stdio};

#[derive(Debug)]
pub enum GpgError {
    IOError(io::Error),
    DecryptFailed,
}

fn nix_err(err: nix::Error) -> GpgError {
    if let nix::Error::Sys(err_no) = err {
        GpgError::IOError(io::Error::from(err_no))
    } else {
        panic!("unexpected nix error type: {:?}", err)
    }
}

pub fn gpg_decrypt(path: &Path, key: &[u8]) -> Result<Vec<u8>, GpgError> {
    let (passphrase_read_fd, passphrase_write_fd) = unistd::pipe().map_err(nix_err)?;

    fcntl::fcntl(passphrase_write_fd, FcntlArg::F_SETFD(FdFlag::FD_CLOEXEC)).map_err(nix_err)?;

    let gpg = Command::new("gpg")
        .arg("--batch")
        .arg("--no-tty")
        .arg("--armor")
        .arg("--decrypt")
        .arg("--passphrase-fd")
        .arg(passphrase_read_fd.to_string())
        .arg("--")
        .arg(path.as_os_str())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(GpgError::IOError)?;

    unistd::close(passphrase_read_fd).map_err(nix_err)?;
    unistd::write(passphrase_write_fd, key).map_err(nix_err)?;
    unistd::close(passphrase_write_fd).map_err(nix_err)?;

    let output = gpg.wait_with_output().map_err(GpgError::IOError)?;
    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(GpgError::DecryptFailed)
    }
}
