use crate::classfile::{self, ConstantPool};

pub(super) struct Class {
    accessFlags: u16,
    name: String, // thisClassName
    superClassName: String,
    interfaceNames: Vec<String>,
    constantPool: ConstantPool,
    // fields:                Vec<Field>,
    // methods:               Vec<Method>,
    // loader:                *ClassLoader,
    // superClass:            *Class,
    // interfaces:            []*Class,
    // instanceSlotCount:     uint,
    // staticSlotCount:       uint,
    // staticVars:            *Slots,
}
impl Class {
    fn new(cf: &classfile::ClassFile) -> Class {
        todo!()
    }
}
