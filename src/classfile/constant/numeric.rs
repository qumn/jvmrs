use bytes::Buf;

use crate::classfile::class_reader::ClassReader;

use super::ConstantInfoRead;

#[derive(Debug)]
pub(crate) struct IntegerInfo {
    pub(crate) val: i32,
}

impl ConstantInfoRead for IntegerInfo {
    fn read_info(reader: &mut ClassReader) -> Self {
        IntegerInfo {
            val: reader.get_i32(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct LongInfo {
    pub(crate) val: i64,
}
impl ConstantInfoRead for LongInfo {
    fn read_info(reader: &mut ClassReader) -> Self {
        LongInfo {
            val: reader.get_i64(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct FloatInfo {
    pub(crate) val: f32,
}
impl ConstantInfoRead for FloatInfo {
    fn read_info(reader: &mut ClassReader) -> Self {
        FloatInfo {
            val: reader.get_f32(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct DoubleInfo {
    pub(crate) val: f64,
}
impl ConstantInfoRead for DoubleInfo {
    fn read_info(reader: &mut ClassReader) -> Self {
        DoubleInfo {
            val: reader.get_f64(),
        }
    }
}
