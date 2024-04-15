use anyhow::Result;
use std::{
    error::Error,
    fmt, fs,
    io::{self, Write},
    path::PathBuf,
};

pub struct Lockfile<'a> {
    file_path: &'a PathBuf,
    lock_path: PathBuf,
    lock: Option<fs::File>,
}

#[derive(Debug, Clone)]
pub enum LockfileError {
    MissingParent,
    NoPermission,
    StaleLock,
    Unknown { err: io::ErrorKind },
}
impl Error for LockfileError {}

impl fmt::Display for LockfileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LockfileError::MissingParent => write!(f, "MissingParent"),
            LockfileError::NoPermission => write!(f, "NoPermission"),
            LockfileError::StaleLock => write!(f, "StaleLock"),
            LockfileError::Unknown { err } => write!(f, "Unknown: {:?}", err),
        }
    }
}

impl<'a> Lockfile<'a> {
    pub fn new(file_path: &'a PathBuf) -> Lockfile {
        let mut lock_path = file_path.clone();
        lock_path.set_extension(".lock");
        return Lockfile {
            file_path,
            lock_path,
            lock: None,
        };
    }

    pub fn hold_for_update(&mut self) -> Result<bool, LockfileError> {
        if self.lock.is_some() {
            return Ok(true);
        }

        return match fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(&self.lock_path)
        {
            Ok(file) => {
                self.lock = Some(file);
                Ok(true)
            }
            Err(err) => match err.kind() {
                io::ErrorKind::AlreadyExists => Ok(false),
                io::ErrorKind::NotFound => Err(LockfileError::MissingParent),
                io::ErrorKind::PermissionDenied => Err(LockfileError::NoPermission),
                _ => Err(LockfileError::Unknown { err: err.kind() }),
            },
        };
    }

    pub fn write(&self, data: &String) -> Result<()> {
        let mut lock = self.get_lock()?;
        lock.write_all(data.as_bytes())?;

        return Ok(());
    }

    pub fn commit(&mut self) -> Result<()> {
        self.get_lock()?;

        fs::rename(&self.lock_path, &self.file_path)?;
        self.lock = None;
        return Ok(());
    }

    fn get_lock(&self) -> Result<&fs::File, LockfileError> {
        return match &self.lock {
            Some(file) => Ok(file),
            None => Err(LockfileError::StaleLock),
        };
    }
}
