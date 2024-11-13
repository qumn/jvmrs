use std::{
    borrow::Borrow,
    cell::{RefCell, UnsafeCell},
    ops::{Deref, DerefMut},
};

use tracing::debug;
use tracing_subscriber::field::debug;

use crate::classfile::{self, FieldrefInfo};

use super::{
    class::Class, constant_pool::ConstantPool, cp_memberref::MemberRef, cp_symref::SymRef,
    field::Field, SharedConstantPool,
};

#[derive(Debug)]
pub struct FieldRef {
    memberRef: MemberRef,
    field: Option<UnsafeCell<Field>>,
}

impl FieldRef {
    pub fn new(cp: SharedConstantPool, class_name: &str, name: &str, descriptor: &str, refInfo: &classfile::FieldrefInfo) -> FieldRef {
        let memberRef = MemberRef::new(cp, class_name, name, descriptor, refInfo);
        FieldRef {
            memberRef,
            field: None,
        }
    }

    fn field(&mut self) -> &Field {
        if self.field.is_none() {
            self.field = Some(UnsafeCell::new(self.resolve_field()));
        }
        unsafe { &*self.field.as_ref().unwrap().get() }
    }

    pub fn resolve_field(&self) -> Field {
        debug!("starte to resolve field: {:?}", self);
        let mut_self = unsafe { &(self) };
        let cp = &self.cp;
        let d = &cp.class;
        let c = self.resolve_class();
        let field = lookup_field(&c, &self.memberRef.name, &self.memberRef.descriptor);
        if field.is_none() {
            debug!("can not find the field: {:?}", &self.memberRef.name);
            panic!("java.lang.NoSuchFieldError");
        }
        if !field.unwrap().is_accessible_to(d) {
            panic!("java.lang.IllegalAccessError");
        }
        return field.unwrap().clone();
    }
}

impl Deref for FieldRef {
    type Target = SymRef;

    fn deref(&self) -> &Self::Target {
        &self.memberRef.symRef
    }
}

impl DerefMut for FieldRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.memberRef.symRef
    }
}

fn lookup_field<'a>(c: &'a Class, name: &str, descriptor: &str) -> Option<&'a Field> {
    debug!("find the field ({}-{}) from class({:?})", name, descriptor, c);
    if let Some(field) = c
        .fields
        .iter()
        .find(|f| f.name == name && f.descriptor == descriptor)
    {
        return Some(field);
    }

    for iface in c.interfaces.iter() {
        if let Some(field) = lookup_field(iface, name, descriptor) {
            return Some(field);
        }
    }

    if c.superClass.is_some() {
        return lookup_field(c.superClass.as_ref().unwrap(), name, descriptor);
    }

    None
}
