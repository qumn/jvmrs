#![allow(non_snake_case)]
#![allow(unused)]
#![allow(non_upper_case_globals)]
#![recursion_limit = "512"]

use std::io::stdout;

use anyhow::Ok;
use clap::Parser;
use classfile::ClassFile;
use classfile::MemberInfo;
use classpath::ClassPath;
use classpath::Entry;
use rtda::heap::ClassLoader;
use tracing::info;

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
    info!("args: {:?}", args);
    startJvm(args).unwrap();
}

fn startJvm(args: Args) -> anyhow::Result<()> {
    let class_path = ClassPath::new(args.cp.as_deref(), args.Xjre.as_deref());
    let class_loader = ClassLoader::new(&class_path);

    let class_name = args.class.replace(".", "/");
    let main_class = class_loader.load_class(&class_name);
    let main_method = main_class.get_main_method();

    match main_method {
        Some(method) => {
            interpret(method);
        },
        _ => {
            println!("main method not found in class {}", args.class);
        }
    }

    Ok(())
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
