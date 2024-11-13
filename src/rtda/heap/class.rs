use core::panic;
use std::{
    cell::{RefCell, UnsafeCell},
    ops::{Deref, DerefMut, Index},
    rc::{Rc, Weak},
    slice::Iter,
    sync::Arc,
};

use tracing::debug;
use tracing_subscriber::field::debug;

use crate::{
    classfile::{self},
    rtda::{heap::SharedConstantPool, slot::{Slot, SlotVec}},
};

use super::{
    access_flag::AccessFlags, class_loader::ClassLoader, constant_pool::ConstantPool,
    cp_interface_methodref, field::Field, method::Method, Object, SharedObject,
};

#[derive(Clone, Debug)]
pub struct SharedClass {
    class: Rc<UnsafeCell<Box<Class>>>,
}

impl SharedClass {
    fn new(class: Class) -> SharedClass {
        SharedClass {
            class: Rc::new(UnsafeCell::new(Box::new(class))),
        }
    }

    pub fn new_object(&self) -> SharedObject {
        if self.is_interface() || self.is_abstract() {
            panic!("java.lang.InstantiationError")
        }
        Object::new_object(self.clone())
    }
}

impl Deref for SharedClass {
    type Target = Class;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.class.get() }
    }
}
impl DerefMut for SharedClass {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.class.get() }
    }
}

#[derive(Debug)]
pub struct Class {
    pub accessFlags: u16,
    pub name: String, // thisClassName
    pub superClassName: String,
    pub interfaceNames: Vec<String>,
    pub constantPool: SharedConstantPool,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
    pub loader: Weak<ClassLoader>,
    pub superClass: Option<SharedClass>,
    pub interfaces: Vec<SharedClass>,
    pub instanceSlotCount: usize,
    pub staticSlotCount: usize,
    pub(crate) staticVars: SlotVec,
}

impl Class {
    pub fn new(cf: &classfile::ClassFile, classLoader: &ClassLoader) -> SharedClass {
        let name = cf.class_name().to_string();
        debug!("start load class: {}", name);
        let superClassName = cf.super_class_name().to_string();
        let interfaceNames = cf.interface_names();
        let superClass = Class::resolveSuperClass(classLoader, &superClassName);
        let interfaces = Class::resolveInterfaces(classLoader, interfaceNames.iter());

        let mut class = SharedClass::new(Class {
            accessFlags: cf.access_flags,
            name,
            superClassName,
            constantPool: SharedConstantPool::default(),
            interfaceNames: cf.interface_names(),
            fields: Vec::new(),
            methods: Vec::new(),
            loader: unsafe { Weak::from_raw(classLoader) },
            superClass: None,
            interfaces: Vec::new(),
            instanceSlotCount: 0,
            staticSlotCount: 0,
            staticVars: SlotVec::new(0),
        });
        class.constantPool = SharedConstantPool::new(class.clone(), &cf.constant_pool);
        class.fields = Field::newFields(class.clone(), cf.fields.iter().collect());
        class.methods = Method::newMethods(class.clone(), cf.methods.iter().collect());
        class.cacInstanceFieldSlotIds();
        class.cacStaticFieldSlotIds();
        class.allocAndInitStaticVars();
        debug!("loaded class {}", class.name);
        class
    }

    pub fn get_main_method(&self) -> Option<&Method> {
        for m in &self.methods {
            debug!("{:?}", m);
            if m.name == "main" && m.descriptor == "([Ljava/lang/String;)V" {
                return Some(m);
            }
        }
        None
    }

    pub fn get_loader(&self) -> Rc<ClassLoader> {
        match self.loader.upgrade() {
            Some(loader) => loader,
            None => unreachable!("can't get the loader of class"),
        }
    }

    pub fn is_accessible_to(&self, other: &Class) -> bool {
        if self.is_public() {
            return true;
        }
        self.get_package_name() == other.get_package_name()
    }

    pub fn get_package_name(&self) -> &str {
        let package_name = match self.name.rsplit_once("/") {
            Some((package_name, _)) => package_name,
            _ => "",
        };
        package_name
    }

    fn resolveSuperClass(loader: &ClassLoader, name: &str) -> Option<SharedClass> {
        if name == "java/lang/Object" {
            let super_class = loader.load_class(name);
            Some(super_class)
        } else {
            None
        }
    }

    fn resolveInterfaces<'a, T, S>(loader: &ClassLoader, names: T) -> Vec<SharedClass>
    where
        T: Iterator<Item = S>,
        S: AsRef<str>,
    {
        names
            .map(|interfaceName| loader.load_class(interfaceName.as_ref()))
            .collect()
    }

    fn cacInstanceFieldSlotIds(&mut self) {
        let mut slotId = 0;
        slotId = match &self.superClass {
            Some(sueprClass) => sueprClass.instanceSlotCount,
            _ => slotId,
        };

        for f in self.fields.iter_mut().filter(|f| !f.is_static()) {
            f.slotId = slotId;
            slotId += 1;
            if f.is_long_or_double() {
                slotId += 1;
            }
        }

        self.instanceSlotCount = slotId;
    }

    fn cacStaticFieldSlotIds(&mut self) {
        let mut slotId = 0;

        for f in self.fields.iter_mut().filter(|f| f.is_static()) {
            f.slotId = slotId;
            slotId += 1;
            if f.is_long_or_double() {
                slotId += 1;
            }
        }

        self.staticSlotCount = slotId;
    }

    fn allocAndInitStaticVars(&mut self) {
        let mut staticVars = SlotVec::new(self.staticSlotCount);
        for static_field in self.fields.iter().filter(|f| f.is_static()) {
            self.initStaticVars(&mut staticVars, static_field);
        }
        self.staticVars = staticVars;
    }

    fn initStaticVars(&self, vars: &mut SlotVec, field: &Field) {
        let cp = &self.constantPool;
        let cpIdx = field.constValueIndex;
        let slotId = field.slotId;

        if cpIdx > 0 {
            match field.descriptor.as_ref() {
                "Z" | "B" | "C" | "S" | "I" => {
                    let val = cp.get_int(cpIdx);
                    vars.set_int(slotId, val);
                }
                "J" => {
                    let val = cp.get_long(cpIdx);
                    vars.set_long(slotId, val);
                }
                "F" => {
                    let val = cp.get_float(cpIdx);
                    vars.set_float(slotId, val);
                }
                "D" => {
                    let val = cp.get_double(cpIdx);
                    vars.set_double(slotId, val);
                }
                "Ljava/lang/String" => {
                    panic!("todo")
                }
                desc => unreachable!("unknow field type: {}", desc),
            }
        }
    }

    pub fn is_public(&self) -> bool {
        self.accessFlags & AccessFlags::ACC_PUBLIC.bits() != 0
    }

    pub fn is_interface(&self) -> bool {
        self.accessFlags & AccessFlags::ACC_INTERFACE.bits() != 0
    }

    pub fn is_abstract(&self) -> bool {
        self.accessFlags & AccessFlags::ACC_ABSTRACT.bits() != 0
    }
}

impl PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
