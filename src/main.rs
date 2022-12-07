#![allow(non_snake_case)]

use anyhow::Ok;
use clap::Parser;
use classfile::ClassFile;
use classpath::ClassPath;
use classpath::Entry;

use crate::classfile::ClassReader;
use crate::cmd::Args;

mod classfile;
mod classpath;
mod cmd;
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
    println!("{:#?}", class_file);
    Ok(())
}


#[cfg(test)]
mod test {
    #[test]
    fn sh(){
        let a: i32 = 1;
        let b: i32 = a << 31;
        //let c: i32 = a << 32;
        println!("{}", a);
        println!("{}", b);
        //println!("{}", c);
    }
}