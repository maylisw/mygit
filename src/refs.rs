use super::lockfile::Lockfile;
use anyhow::{anyhow, Context, Result};

use std::{error::Error, fmt, fs, io::Read, path::PathBuf};

#[derive(Debug, Clone)]
struct LockDenied;
impl Error for LockDenied {}

impl fmt::Display for LockDenied {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "LockDenied");
    }
}

pub struct Refs {
    pathname: PathBuf,
    head_path: PathBuf,
}

impl Refs {
    pub fn new(path: PathBuf) -> Refs {
        return Refs {
            head_path: path.join("HEAD"),
            pathname: path.clone(),
        };
    }

    pub fn update_head(&self, oid: &String) -> Result<()> {
        let mut lockfile = Lockfile::new(&self.head_path);
        if !lockfile.hold_for_update()? {
            return Err(anyhow!(LockDenied).context(format!(
                "Could not acquire lock on file: {:?}",
                &self.head_path
            )));
        }

        lockfile.write(oid)?;
        lockfile.write(&"\n".to_string())?;
        lockfile.commit()?;
        return Ok(());
    }

    pub fn read_head(&self) -> Result<String, anyhow::Error> {
        if self.head_path.exists() {
            let mut head: fs::File = fs::OpenOptions::new()
                .read(true)
                .open(&self.head_path)
                .with_context(|| {
                    format!("failed to open head file for reading: {:?}", self.head_path)
                })?;

            let mut buf: String = String::new();
            head.read_to_string(&mut buf)?;
            return Ok(buf);
        }
        return Ok(String::new());
    }
}
