use std::{collections::HashMap, iter::Map};

use crate::classpath::{self, ClassPath, Entry};

use super::class::Class;

struct ClassLoader<'a> {
    cp: &'a ClassPath,
    classCache: HashMap<&'static str, Class>,
}

impl<'a> ClassLoader<'a> {
    pub(crate) fn new(cp: &ClassPath) -> ClassLoader {
        ClassLoader {
            cp,
            classCache: HashMap::new(),
        }
    }

    pub(crate) fn load_class(&self, name: &str) -> &Class {
        if self.classCache.contains_key(name) {
            return self.classCache.get(name).unwrap();
        }
        self.loadNonArray(name);
        todo!()
    }

    fn loadNonArray(&self, name: &str) {

    }

    fn read_class(&self, name: &str) -> (Vec<u8>, &dyn Entry) {
        match self.cp.read_class(name) {
            Ok((bytes, entry)) => (bytes, entry),
            Err(msg) => {
                panic!("{}", msg)
            }
        }
    }
    fn define_class(&self) -> Class {
        todo!()
    }
}
