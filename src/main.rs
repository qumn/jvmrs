#![allow(non_snake_case)]

use anyhow::Ok;
use clap::Parser;
use classpath::ClassPath;
use classpath::Entry;

use crate::cmd::Args;

mod classpath;
mod cmd;

fn main() {
    tracing_subscriber::fmt::init();
    let args = Args::parse();
    startJvm(args).unwrap();
}

fn startJvm(args: Args) -> anyhow::Result<()> {
    println!("start jvm with {:?}", args);
    let class_loader = ClassPath::new(args.cp.as_deref(), args.Xjre.as_deref());
    let (data, _entry) = class_loader.read_class(&args.class)?;
    println!("{:?}", data);
    Ok(())
}
