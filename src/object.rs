pub mod author;

pub use author::Author;
use hex;
use std::{fmt, path::PathBuf};

pub enum Datatype {
    Blob,
    Tree,
    Commit,
}

impl fmt::Display for Datatype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Datatype::Blob => write!(f, "blob"),
            Datatype::Tree => write!(f, "tree"),
            Datatype::Commit => write!(f, "commit"),
        }
    }
}

pub trait Object {
    type Input;

    fn new(data: Self::Input) -> Self;
    fn bytes(&self) -> Vec<u8>;
    fn datatype(&self) -> &Datatype;
    fn oid(&self) -> &String;
    fn set_oid(&mut self, oid: String);
}

pub struct Blob {
    data: Vec<u8>,
    datatype: Datatype,
    oid: String,
}

impl Object for Blob {
    type Input = Vec<u8>;

    fn new(data: Vec<u8>) -> Blob {
        return Blob {
            data: data,
            datatype: Datatype::Blob,
            oid: String::new(),
        };
    }

    fn bytes(&self) -> Vec<u8> {
        return self.data.clone();
    }

    fn datatype(&self) -> &Datatype {
        return &self.datatype;
    }

    fn oid(&self) -> &String {
        return &self.oid;
    }

    fn set_oid(&mut self, oid: String) {
        self.oid = oid;
    }
}

pub struct Entry {
    pub path: PathBuf,
    pub oid: String,
    pub mode: String,
}

pub struct Tree {
    entries: Vec<Entry>,
    datatype: Datatype,
    oid: String,
}

impl Object for Tree {
    type Input = Vec<Entry>;

    fn new(entries: Vec<Entry>) -> Tree {
        let mut e: Vec<Entry> = entries;
        e.sort_by(|a, b| a.path.cmp(&b.path));

        return Tree {
            entries: e,
            datatype: Datatype::Tree,
            oid: String::new(),
        };
    }

    fn bytes(&self) -> Vec<u8> {
        let mut content: Vec<u8> = vec![];
        for e in &self.entries {
            let hex_oid = hex::decode(&e.oid).unwrap();
            content.extend(
                format!("{} {}\0", e.mode, e.path.to_str().unwrap())
                    .as_bytes()
                    .to_vec(),
            );
            content.extend(hex_oid);
        }
        return content;
    }

    fn datatype(&self) -> &Datatype {
        return &self.datatype;
    }

    fn oid(&self) -> &String {
        return &self.oid;
    }

    fn set_oid(&mut self, oid: String) {
        self.oid = oid;
    }
}

pub struct Commit<'a> {
    parent_oid: &'a String,
    tree_oid: &'a String,
    author: Author,
    message: String,
    datatype: Datatype,
    oid: String,
}

impl<'a> Object for Commit<'a> {
    type Input = (&'a String, &'a String, Author, String);

    fn new(
        (parent_oid, tree_oid, author, message): (&'a String, &'a String, Author, String),
    ) -> Commit {
        return Commit {
            parent_oid,
            tree_oid,
            author,
            message,
            datatype: Datatype::Commit,
            oid: String::new(),
        };
    }

    fn bytes(&self) -> Vec<u8> {
        let mut commit: String = format!("tree {}\n", self.tree_oid);
        if !self.parent_oid.is_empty() {
            commit += &format!("parent {}\n", self.parent_oid);
        }
        commit += &format!("author {}\n", self.author);
        commit += &format!("committer {}\n", self.author);
        commit += &"\n";
        commit += &self.message;

        return commit.as_bytes().to_vec();
    }

    fn datatype(&self) -> &Datatype {
        return &self.datatype;
    }

    fn oid(&self) -> &String {
        return &self.oid;
    }

    fn set_oid(&mut self, oid: String) {
        self.oid = oid;
    }
}
