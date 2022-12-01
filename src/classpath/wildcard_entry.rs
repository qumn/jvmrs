use std::{fs, path::PathBuf};

use tracing::trace;

use crate::classpath::newEntry;

use super::Entry;

#[derive(Debug)]
pub(super) struct WildCardEntry(Vec<Box<dyn Entry>>);

impl WildCardEntry {
    pub(super) fn new(path: &str) -> Self {
        trace!("create a wildcard entry: {path}");
        let dir_path = path.strip_suffix("/*").expect("expect path end with /*");
        let jars = getJarFile(dir_path);
        let entrys = jars
            .into_iter()
            .map(|path| newEntry(path.to_str().unwrap()))
            .collect();
        let wild_card = WildCardEntry(entrys);

        wild_card
    }
}

impl Entry for WildCardEntry {
    fn read_class_by_path(&self, classpath: &str) -> anyhow::Result<(Vec<u8>, &dyn Entry)> {
        for entry in &self.0 {
            let res = entry.read_class_by_path(classpath);
            if res.is_ok() {
                return res;
            }
        }
        return Err(anyhow::Error::msg("Can't find class"));
    }
}

fn getJarFile(dir: &str) -> Vec<PathBuf> {
    let dr = fs::read_dir(dir).expect("read dir error");
    dr.into_iter()
        .filter_map(|de| de.ok())
        .filter(|de| de.file_type().map(|t| t.is_file()).unwrap_or(false))
        .filter(|de| {
            de.path()
                .extension()
                .map(|ext| ext == "jar")
                .unwrap_or(false)
            ||
            de.path()
                .extension()
                .map(|ext| ext == "JAR")
                .unwrap_or(false)
        })
        .map(|de| de.path())
        .collect()
}

#[cfg(test)]
mod test {
    use super::getJarFile;

    #[test]
    fn ends_with_should_work() {
        let jars = getJarFile(
            "/Users/qumn/Library/Java/JavaVirtualMachines/corretto-1.8.0_342/Contents/Home/jre/lib",
        );
        //info!("{:?}", jars);
        println!("{:?}", jars);
        assert!(jars.len() > 0);
    }
}
