use nix::{
    self,
    fcntl::{self, FcntlArg, FdFlag},
    unistd,
};
use std::fs::{self, File};
use std::io;
use std::io::Write;
use std::os::unix::io::FromRawFd;
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::process::{Command, Stdio};

#[derive(Debug)]
pub enum GpgError {
    IOError(io::Error),
    OperationFailed(String),
}

fn nix_err(err: nix::Error) -> GpgError {
    if let nix::Error::Sys(err_no) = err {
        GpgError::IOError(io::Error::from(err_no))
    } else {
        panic!("unexpected nix error type: {:?}", err)
    }
}

fn gpg_err(msg: &[u8]) -> GpgError {
    GpgError::OperationFailed(String::from_utf8_lossy(msg).to_string())
}

pub fn decrypt(path: &Path, key: &[u8]) -> Result<Vec<u8>, GpgError> {
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
    unsafe {
        let mut stream = UnixStream::from_raw_fd(passphrase_write_fd);
        stream.write_all(key).map_err(GpgError::IOError)?;
    }

    let output = gpg.wait_with_output().map_err(GpgError::IOError)?;
    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(gpg_err(&output.stderr))
    }
}

pub fn encrypt(data: &str, key: &[u8], target: &Path) -> Result<(), GpgError> {
    let mut target_tmp = target.as_os_str().to_os_string();
    target_tmp.push(".tmp");

    let target_file = File::create(&target_tmp).map_err(GpgError::IOError)?;
    match gpg_encrypt_impl(data, key, target_file) {
        Ok(_) => {
            fs::rename(&target_tmp, target).map_err(GpgError::IOError)?;
            Ok(())
        }
        Err(err) => {
            fs::remove_file(&target_tmp).map_err(GpgError::IOError)?;
            Err(err)
        }
    }
}

fn gpg_encrypt_impl<T: Into<Stdio>>(data: &str, key: &[u8], outfile: T) -> Result<(), GpgError> {
    let (passphrase_read_fd, passphrase_write_fd) = unistd::pipe().map_err(nix_err)?;
    fcntl::fcntl(passphrase_write_fd, FcntlArg::F_SETFD(FdFlag::FD_CLOEXEC)).map_err(nix_err)?;

    let mut gpg = Command::new("gpg")
        .arg("--batch")
        .arg("--no-tty")
        .arg("--armor")
        .arg("--symmetric")
        .arg("--passphrase-fd")
        .arg(passphrase_read_fd.to_string())
        .stdin(Stdio::piped())
        .stdout(outfile)
        .stderr(Stdio::piped())
        .spawn()
        .map_err(GpgError::IOError)?;

    unistd::close(passphrase_read_fd).map_err(nix_err)?;
    unsafe {
        let mut stream = UnixStream::from_raw_fd(passphrase_write_fd);
        stream.write_all(key).map_err(GpgError::IOError)?;
    }
    {
        gpg.stdin
            .as_mut()
            .unwrap()
            .write_all(data.as_bytes())
            .map_err(GpgError::IOError)?;
    }

    let output = gpg.wait_with_output().map_err(GpgError::IOError)?;
    if output.status.success() {
        Ok(())
    } else {
        Err(gpg_err(&output.stderr))
    }
}
