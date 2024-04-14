mod database;
pub mod object;
mod workspace;

use database::Database;
use object::{Author, Blob, Commit, Entry, Object, Tree};
use workspace::Workspace;

use anyhow::{anyhow, Context, Result};
use std::{
    env, fs,
    io::{stdin, BufRead, BufReader, Write},
    path::PathBuf,
};

// allows for testing w/o clobbering git history
const GIT_DIR: &str = ".mygit";

fn main() -> Result<()> {
    if env::args().len() < 2 {
        return Err(anyhow!("mygit requires a valid sub-command"));
    }

    return match env::args().nth(1).unwrap().as_str() {
        "init" => init(),
        "commit" => commit(),
        x => Err(anyhow!("\"{x}\" is not a valid mygit sub-command")),
    };
}

fn init() -> Result<()> {
    let mut git_path: PathBuf = match env::args().nth(2) {
        Some(dir) => PathBuf::from(dir),
        None => env::current_dir().with_context(|| "failed to get current dir")?,
    };

    git_path.push(GIT_DIR);

    for subdir in vec!["objects", "refs"] {
        fs::create_dir_all(git_path.join(subdir)).with_context(|| "failed to create dir")?;
    }

    println!("Initialized empty mygit dir at {:#?}", git_path);
    return Ok(());
}

fn commit() -> Result<()> {
    let root_path: PathBuf = env::current_dir().with_context(|| "failed to get current dir")?;
    let git_path: PathBuf = root_path.join(GIT_DIR);
    let refs_path: PathBuf = git_path.join("objects");

    let workspace: Workspace = Workspace::new(root_path);
    let db: Database = Database::new(refs_path);

    let mut entries: Vec<Entry> = vec![];

    for path in workspace.list_files(None)? {
        let byte_content: Vec<u8> = fs::read(&path)?;

        let mut blob: Blob = Blob::new(byte_content);
        db.store(&mut blob)?;

        entries.push(Entry {
            oid: blob.oid().to_owned(),
            path: path,
            mode: String::from("100644"),
        });
    }

    let mut tree = Tree::new(entries);
    db.store(&mut tree)?;

    // COMMIT

    let name = env::var("GIT_AUTHOR_NAME").unwrap_or_default();
    let email = env::var("GIT_AUTHOR_EMAIL").unwrap_or_default();
    let author = Author::new(name, email);

    println!("Enter commit message");
    let reader = BufReader::new(stdin());
    let message: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let mut commit = Commit::new((tree.oid(), author, message.join("\n")));
    db.store(&mut commit)?;

    // Write HEAD

    let mut head: fs::File = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(git_path.join("HEAD"))?;
    head.write(commit.oid().as_bytes())?;

    println!("[(root-commit) {}] {}", commit.oid(), message[0]);

    return Ok(());
}
