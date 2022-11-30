use clap::Parser;

use crate::cmd::Args;


mod cmd;
fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
