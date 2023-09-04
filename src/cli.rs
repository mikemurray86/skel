use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command()]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Use a template at the requested location
    Use(UseArgs),
    /// Save a new template
    Save(SaveArgs),
    /// Display current configuration
    Config(ConfigArgs),
}

#[derive(Args, Debug)]
pub struct UseArgs {
    /// the name of the template to work with
    #[arg(short, long)]
    pub template: String,
    /// the target location
    #[arg(short='T', long, default_value_t = String::from("./"))]
    pub target: String,
    #[arg(short, long)]
    pub context_file: Option<Option<String>>,
}

#[derive(Args, Debug)]
pub struct SaveArgs {
    /// the name of the template to work with
    #[arg(short, long)]
    pub template: String,
    /// the target location
    #[arg(short='T', long, default_value_t = String::from("./"))]
    pub target: String,
}

#[derive(Args, Debug)]
pub struct ConfigArgs {}
