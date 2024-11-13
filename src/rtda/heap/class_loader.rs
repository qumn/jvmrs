use std::{
    collections::HashMap,
    iter::Map,
    rc::{Rc, Weak},
};

use crate::{
    classfile::{self, ClassFile, ClassReader},
    classpath::{self, ClassPath, Entry},
};

use super::class::{Class, SharedClass};

#[derive(Clone)]
pub struct ClassLoader {
    cp: ClassPath,
    classCache: Rc<HashMap<&'static str, SharedClass>>,
}

impl ClassLoader {
    pub(crate) fn new(cp: &ClassPath) -> ClassLoader {
        ClassLoader {
            cp: cp.clone(),
            classCache: Rc::new(HashMap::new()),
        }
    }

    pub(crate) fn load_class(&self, name: &str) -> SharedClass {
        if self.classCache.contains_key(name) {
            return self.classCache.get(name).unwrap().clone();
        }
        self.loadNonArray(name)
    }

    fn loadNonArray(&self, name: &str) -> SharedClass {
        let (data, entry) = self.read_class(name);
        let class = self.parse_class(data);
        class
    }

    fn read_class(&self, name: &str) -> (Vec<u8>, &dyn Entry) {
        match self.cp.read_class(name) {
            Ok((bytes, entry)) => (bytes, entry),
            Err(msg) => {
                panic!("{}", msg)
            }
        }
    }

    fn parse_class(&self, bytes: Vec<u8>) -> SharedClass {
        let cr = ClassReader::new(bytes);
        let cf = ClassFile::new(cr);
        Class::new(&cf, self)
    }

}
