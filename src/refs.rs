use anyhow::{Context, Result};
use std::{
    fs,
    io::{Read, Write},
    path::PathBuf,
};

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
        let mut head: fs::File = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(&self.head_path)
            .with_context(|| {
                format!("failed to open head file for writing: {:?}", self.head_path)
            })?;
        head.write(oid.as_bytes())?;
        head.write(&[b'\n'])?;
        return Ok(());
    }

    pub fn read_head(&self) -> Result<String> {
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
