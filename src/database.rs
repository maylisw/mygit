use super::Object;

use anyhow::{Context, Result};
use miniz_oxide::deflate::{self, CompressionLevel};
use rand::{distributions::Alphanumeric, Rng};
use sha1::{Digest, Sha1};
use std::{
    fs,
    io::prelude::*,
    path::{Path, PathBuf},
};

pub struct Database {
    pathname: PathBuf,
}

impl Database {
    pub fn new(path: PathBuf) -> Database {
        return Database { pathname: path };
    }

    pub fn store<T: Object>(&self, object: &mut T) -> Result<()> {
        let mut obj_data: Vec<u8> = object.bytes();
        let mut content: Vec<u8> = format!("{} {}\0", object.datatype(), obj_data.len())
            .as_bytes()
            .to_vec();
        content.append(&mut obj_data);

        object.set_oid(format!("{:x}", Sha1::digest(&content)));

        self.write_object(object.oid(), &content)?;
        return Ok(());
    }

    pub fn write_object(&self, oid: &String, data: &Vec<u8>) -> Result<()> {
        let object_path: PathBuf = self.pathname.join(Path::new(&oid[0..2]).join(&oid[2..40]));
        let dirname: &Path = object_path.parent().unwrap();

        fs::create_dir_all(dirname).with_context(|| "failed to create dir")?;

        let temp_path: PathBuf = dirname.join(generate_temp_name());
        let mut temp_file: fs::File = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(&temp_path)?;

        let compressed = deflate::compress_to_vec_zlib(data, CompressionLevel::BestSpeed as u8);
        temp_file.write_all(&compressed)?;

        fs::rename(temp_path, object_path)?;

        return Ok(());
    }
}

fn generate_temp_name() -> String {
    let rand_str: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();
    return format!("tmp_obj_{rand_str}");
}
