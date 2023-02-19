//! Copies a template folder into the requested location.
//!
//! This module is used to read a template folder from a defined location and
//! then copy it into the target folder. The template folder location can be specified
//! in a configuration file or an environment variable. Inside the template folder will
//! be folders containing each template. The template name will be the same name as the
//! folder containing it.

use clap::Parser;
use skel::{get_config, use_template};
mod cli;

/// Main function that handles reading the config file for the template directory then
/// picking the correct template and copying it's contents into the target location
fn main() {
    let config = get_config();
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::Use(args) => {
            let template = format!("{}/{}/", config.template_dir, args.template.as_str());
            use_template(template.as_str(), args.target.as_str());
        }
        cli::Commands::Save(args) => {
            let target = format!("{}/{}", config.template_dir, args.template.as_str());
            let template = format!("{}/", args.target.as_str());
            use_template(template.as_str(), target.as_str());
        }
    }
    println!("Done!");
}
