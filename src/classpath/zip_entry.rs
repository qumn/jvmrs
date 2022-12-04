use core::panic;
use std::{
    cell::RefCell,
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
};

use anyhow::Ok;
use zip::ZipArchive;

use super::Entry;

#[derive(Debug)]
pub(super) struct ZipEntry {
    zip_archive: RefCell<Option<ZipArchive<File>>>,
    abs_path: Box<Path>,
}

impl ZipEntry {
    pub(super) fn new(path: &str) -> Self {
        let path = PathBuf::from(path);
        if !path.exists() {
            panic!("path {} not exists", path.display());
        }
        ZipEntry {
            zip_archive: RefCell::new(None),
            abs_path: path.into_boxed_path(),
        }
    }
}

impl Entry for ZipEntry {
    fn read_class_by_path(&self, classpath: &str) -> anyhow::Result<(Vec<u8>, &dyn Entry)> {
        let mut zip_archive = self.zip_archive.borrow_mut();
        let zip = zip_archive.get_or_insert_with(|| {
            let file = fs::File::open(self.abs_path.clone()).unwrap();
            ZipArchive::new(file).expect("请输入正确的路径")
        });

        let mut file = zip.by_name(classpath).expect("can't find class file");
        let mut buf = Vec::with_capacity(file.size() as usize);
        file.read_to_end(&mut buf)?;
        Ok((buf, self))
    }
}
