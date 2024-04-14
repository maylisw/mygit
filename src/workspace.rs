use anyhow::Result;
use std::{
    fs,
    io::{prelude::*, BufReader},
    path::{Path, PathBuf},
};

pub struct Workspace {
    pathname: PathBuf,
    ignores: Vec<PathBuf>,
}

impl Workspace {
    pub fn new(path: PathBuf) -> Self {
        let mut ignores = vec![PathBuf::from(".git")];
        let gitignores = match read_gitignore(&path.join(".gitignore")) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("error reading .gitignore {e}");
                vec![]
            }
        };
        ignores.extend(gitignores);

        return Self {
            pathname: path,
            ignores: ignores,
        };
    }

    pub fn list_files(&self, root_path: Option<&PathBuf>) -> Result<Vec<PathBuf>> {
        let root_path = match root_path {
            Some(path) => path,
            None => &self.pathname,
        };
        let entries = fs::read_dir(root_path)?;
        let mut files = vec![];

        for e in entries {
            let entry = e?;
            let path = PathBuf::from(entry.path().strip_prefix(root_path)?);
            if self.ignores.contains(&path) {
                continue;
            } else if path.is_dir() {
                eprintln!("{:?} is a directory, not recursing", path);
                // let res = self.list_files(Some(&path))?;
                // files.extend(res);
                continue;
            } else {
                files.push(path);
            }
        }
        return Ok(files);
    }
}

fn read_gitignore(gitignore: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut paths = vec![];

    if Path::exists(&gitignore) {
        let file = fs::File::open(gitignore)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let raw_line = line?;
            paths.push(PathBuf::from(raw_line));
        }
    }

    return Ok(paths);
}
