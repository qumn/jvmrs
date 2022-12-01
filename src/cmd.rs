use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author = "qumn", version, about, long_about = None)]
pub(crate) struct Args {
    #[arg(short, long = "classpath", help = "Special class path")]
    pub(crate) cp: Option<String>,
    #[arg(help = "Special class")]
    pub(crate) class: String,

    pub(crate) args: Vec<String>,

    #[arg(short, long = "Xjre", help = "Special runtime jar path")]
    pub(crate) Xjre: Option<String>,
}
