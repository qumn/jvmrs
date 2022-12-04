use std::{
    fs::{self},
    path::PathBuf,
};

use super::Entry;
use anyhow::{Ok, Result};

#[derive(Debug)]
pub(super) struct DirEntry {
    abs_dir: PathBuf,
}

impl DirEntry {
    pub(super) fn new(path: &str) -> Self {
        let mut pb = PathBuf::from(path);
        pb = fs::canonicalize(pb).expect("请输入一个正确的路径");
        DirEntry { abs_dir: pb }
    }
}

impl Entry for DirEntry {
    fn read_class_by_path(&self, classpath: &str) -> Result<(Vec<u8>, &dyn Entry)> {
        let mut path = self.abs_dir.clone();
        path.push(classpath);
        Ok((fs::read(path).expect("can't find class"), self))
    }
}
