//! Copies a template folder into the requested location.
//!
//! This module is used to read a template folder from a defined location and
//! then copy it into the target folder. The template folder location can be specified
//! in a configuration file or an environment variable. Inside the template folder will
//! be folders containing each template. The template name will be the same name as the
//! folder containing it.

use figment::{
    providers::{Env, Format, Yaml},
    Figment,
};
use serde::Deserialize;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Deserialize)]
struct Config {
    template_dir: String,
}

/// Returns a configuration file path to attempt to read data from.
///
/// If the $XDG_CONFIG_HOME environment variable is set then the file will
/// be in `$XDG_CONFIG_HOME/skel.yaml` otherwise it will be in `$HOME/.config/skel.yaml`
///
/// if $HOME is not defined the function will panic
fn get_config_path() -> OsString {
    let config_dir = match env::var_os("XDG_CONFIG_HOME") {
        Some(mut file) => {
            file.push("/skel.yaml");
            file
        }
        None => {
            let mut home_dir = env::var_os("HOME").unwrap();
            home_dir.push("/.config/skel.yaml");
            home_dir
        }
    };
    env::var_os("SKEL_CONFIG").unwrap_or(config_dir)
}

/// Prints a basic help message to help show how to use the app
fn print_help(name: &str) {
    println!(
        "\nUsage: {name} <template> <target>\n\
              \ttemplate: the configured template to load\n\
              \ttarget: the directory to load the template into\n"
    );
}

/// Main function that handles reading the config file for the template directory then
/// picking the correct template and copying it's contents into the target location
fn main() {
    let config: Config = Figment::new()
        .merge(Yaml::file(get_config_path()))
        .merge(Env::prefixed("SKEL_"))
        .extract()
        .unwrap_or(Config {
            template_dir: String::from("."),
        });

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("\nNot enough arguments provided.");
        print_help(&args[0]);
        return;
    }

    if args[1].to_lowercase() == "help" {
        print_help(&args[0]);
        return;
    }

    let template = format!("{}/{}/", config.template_dir, &args[1]);
    let target = &args[2];

    if !Path::new(target).exists() {
        println!("{target} does not exist. Creating...");
        fs::create_dir_all(target).unwrap();
    }

    let entries = fs::read_dir(&template)
        .unwrap()
        .map(|res| res.map(|e| e.file_name()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();

    for file in entries {
        let mut src = OsString::from(&template);
        src.push(file.as_os_str());
        let mut dest = OsString::from(&target);
        dest.push("/");
        dest.push(file.as_os_str());
        fs::copy(src, dest).unwrap();
    }
    println!("Done!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_config_dir() {
        let home = env::var("HOME").unwrap();
        let config = env::var("XDG_CONFIG_HOME").unwrap_or("".to_string());
        env::set_var("HOME", "/home/test");
        env::set_var("XDG_CONFIG_HOME", "/home/test/.config");
        assert_eq!(get_config_path(), "/home/test/.config/skel.yaml");
        env::set_var("HOME", home);
        if config != "" {
            env::set_var("XDG_CONFIG_HOME", config);
        } else {
            env::remove_var("XDG_CONFIG_HOME");
        }
    }
}
