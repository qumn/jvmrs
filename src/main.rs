#![allow(non_snake_case)]
#![allow(unused)]
#![allow(non_upper_case_globals)]

#![recursion_limit = "256"]

use anyhow::Ok;
use clap::Parser;
use classfile::ClassFile;
use classfile::MemberInfo;
use classpath::ClassPath;
use classpath::Entry;

use crate::classfile::ClassReader;
use crate::cmd::Args;
use crate::interpreter::interpret;

mod classfile;
mod classpath;
mod cmd;
mod instructions;
mod interpreter;
mod rtda;

fn main() {
    tracing_subscriber::fmt::init();
    let args = Args::parse();
    startJvm(args).unwrap();
}

fn startJvm(args: Args) -> anyhow::Result<()> {
    let class_loader = ClassPath::new(args.cp.as_deref(), args.Xjre.as_deref());
    let (data, _entry) = class_loader.read_class(&args.class)?;

    let class_reader = ClassReader::new(data);
    let class_file = ClassFile::new(class_reader);
    let main = getMainMethod(&class_file).unwrap();
    interpret(main);
    println!("{:#?}", class_file);
    Ok(())
}
fn getMainMethod(cf: &ClassFile) -> Option<&MemberInfo> {
    for m in &cf.methods {
        println!("{:?}", m);
        if m.name() == "main" && m.descriptor() == "([Ljava/lang/String;)V" {
            return Some(m);
        }
    }
    None
}

#[cfg(test)]
mod test {
    #[test]
    fn sh() {
        let a: i32 = 1;
        let b: i32 = a << 31;
        //let c: i32 = a << 32;
        println!("{}", a);
        println!("{}", b);
        //println!("{}", c);
    }
}
