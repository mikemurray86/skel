use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command()]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Use(ArgsL),
    Save(ArgsL),
}

#[derive(Args, Debug)]
pub struct ArgsL {
    #[arg(short, long)]
    pub template: String,
    #[arg(short='T', long, default_value_t = String::from("./"))]
    pub target: String,
}
