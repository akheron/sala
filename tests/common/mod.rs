use copy_dir::copy_dir;
use std::io;
use std::path::PathBuf;
use tempfile::{tempdir, TempDir};

pub struct TempRepo {
    dir: TempDir,
}

impl TempRepo {
    pub fn new() -> io::Result<TempRepo> {
        let dir = tempdir()?;
        copy_dir("tests/repo", dir.path().join("repo"))?;
        Ok(TempRepo { dir })
    }

    pub fn path(&self) -> PathBuf {
        self.dir.path().join("repo")
    }

    pub fn path_string(&self) -> String {
        self.path().to_string_lossy().to_string()
    }
}
