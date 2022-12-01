use std::{fs::{File, self}, io::Read, cell::RefCell};

use anyhow::Ok;
use zip::ZipArchive;

use super::Entry;

#[derive(Debug)]
pub(super) struct ZipEntry {
    zip_archive: RefCell<ZipArchive<File>>,
}

impl ZipEntry {
    pub(super) fn new(path: &str) -> Self {
        let file = fs::File::open(path).expect("请输入正确的路径");
        let zip_archive = ZipArchive::new(file).expect("请输入正确的路径");
        ZipEntry { zip_archive: RefCell::new(zip_archive) }
    }
}

impl Entry for ZipEntry {
    fn read_class_by_path(&self, classpath: &str) -> anyhow::Result<(Vec<u8>, &dyn Entry)> {
        let mut zip = self.zip_archive.borrow_mut();
        let mut file =  zip.by_name(classpath)?;
        let mut buf =  Vec::with_capacity(file.size() as usize);
        file.read_to_end(&mut buf)?;
        Ok((buf, self))
    }
}
