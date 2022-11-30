use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author = "qumn", version, about, long_about = None)]
pub struct Args {
    #[arg(short, long = "classpath", help = "Special class path")]
    cp: Option<String>,
    #[arg(help = "Special class")]
    class: String,

    args: Vec<String>,
}
