use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command()]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Use(UseArgs),
    Save(SaveArgs),
}

#[derive(Args, Debug)]
pub struct UseArgs {
    #[arg(short, long)]
    pub template: String,
    #[arg(short='T', long, default_value_t = String::from("./"))]
    pub target: String,
    #[arg(short, long)]
    pub context_file: Option<Option<String>>,
}

#[derive(Args, Debug)]
pub struct SaveArgs {
    #[arg(short, long)]
    pub template: String,
    #[arg(short='T', long, default_value_t = String::from("./"))]
    pub target: String,
}
