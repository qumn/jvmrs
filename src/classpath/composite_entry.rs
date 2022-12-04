use super::{newEntry, Entry, PATH_SEPARATOR};
use anyhow::Result;

#[derive(Debug)]
pub(super) struct CompositeEntry(Vec<Box<dyn Entry>>);

impl CompositeEntry {
    pub(super) fn new(paths: &str) -> Self {
        let entrys: Vec<Box<dyn Entry>> = paths
            .split(PATH_SEPARATOR)
            .map(|path| newEntry(path))
            .collect();
        Self(entrys)
    }
}

impl Entry for CompositeEntry {
    fn read_class_by_path(&self, classpath: &str) -> Result<(Vec<u8>, &dyn Entry)> {
        for entry in &self.0 {
            let res = entry.read_class_by_path(classpath);
            if res.is_ok() {
                return res;
            }
        }
        return Err(anyhow::Error::msg("Can't find class"));
    }
}
