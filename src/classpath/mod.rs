use core::panic;
use std::{env, path::PathBuf};

use anyhow::Result;
mod composite_entry;
mod dir_entry;
mod wildcard_entry;
mod zip_entry;

use composite_entry::CompositeEntry;
use dir_entry::DirEntry;
use wildcard_entry::WildCardEntry;
use zip_entry::ZipEntry;

const PATH_SEPARATOR: &str = if cfg!(windows) { ";" } else { ":" };
const JAVA_HOME: &str = "JAVA_HOME";

pub(crate) trait Entry: std::fmt::Debug {
    fn read_class_by_path(&self, classpath: &str) -> Result<(Vec<u8>, &dyn Entry)>;

    fn read_class(&self, classname: &str) -> Result<(Vec<u8>, &dyn Entry)> {
        let classpath = classname.replace(".", "/") + ".class";
        self.read_class_by_path(&classpath)
    }
}

fn newEntry(path: &str) -> Box<dyn Entry> {
    if path.contains(PATH_SEPARATOR) {
        return Box::new(CompositeEntry::new(path));
    }
    if path.ends_with("/*") {
        return Box::new(WildCardEntry::new(path));
    }
    if path.ends_with(".jar")
        || path.ends_with(".JAR")
        || path.ends_with(".zip")
        || path.ends_with(".ZIP")
    {
        return Box::new(ZipEntry::new(path));
    }
    return Box::new(DirEntry::new(path));
}

#[derive(Debug)]
pub(crate) struct ClassPath {
    bootClassLoader: Box<dyn Entry>,
    extClassLoader: Box<dyn Entry>,
    userClassLoader: Box<dyn Entry>,
}

impl ClassPath {
    pub(crate) fn new(cp: Option<&str>, jre: Option<&str>) -> Self {
        let (boot_class_loader, ext_class_loader) = Self::parseBootAndExtClassLoader(jre);
        let user_class_loader = Self::parseUserClassLoader(cp);
        Self {
            bootClassLoader: boot_class_loader,
            extClassLoader: ext_class_loader,
            userClassLoader: user_class_loader,
        }
    }

    fn parseBootAndExtClassLoader(jre_dir: Option<&str>) -> (Box<dyn Entry>, Box<dyn Entry>) {
        let mut jre_dir = ClassPath::getJreDir(jre_dir);
        jre_dir.push("lib");
        jre_dir.push("*");
        let boot_class_loader = newEntry(&jre_dir.as_path().to_str().unwrap());
        jre_dir.pop();
        jre_dir.push("ext");
        jre_dir.push("*");
        let ext_class_loader = newEntry(&jre_dir.as_path().to_str().unwrap());
        return (boot_class_loader, ext_class_loader);
    }

    fn getJreDir(jre_dir: Option<&str>) -> PathBuf {
        if let Some(jar_dir) = jre_dir {
            let mut pb = PathBuf::from(jar_dir);
            pb.push("jre");
            if pb.exists() {
                return pb;
            }
        }
        if let Ok(dir) = env::current_dir() {
            let mut pb = dir;
            pb.push("jre");
            if pb.exists() {
                return pb;
            }
        }
        if let Ok(java_home) = env::var(JAVA_HOME) {
            let mut pb = PathBuf::from(java_home);
            pb.push("jre");
            if pb.exists() {
                return pb;
            }
        }
        panic!("Can not find jre folder")
    }

    fn parseUserClassLoader(cp: Option<&str>) -> Box<dyn Entry> {
        if let Some(cp) = cp {
            return newEntry(cp);
        }
        return newEntry(".");
    }
}

impl Entry for ClassPath {
    fn read_class_by_path(&self, classpath: &str) -> Result<(Vec<u8>, &dyn Entry)> {
        let result = self.bootClassLoader.read_class_by_path(classpath);
        if result.is_ok() {
            return result;
        }
        let result = self.extClassLoader.read_class_by_path(classpath);
        if result.is_ok() {
            return result;
        }
        return self.userClassLoader.read_class_by_path(classpath);
    }
}
