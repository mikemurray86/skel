// pub mod cli;

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
pub struct Config {
    pub template_dir: String,
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

/// Returns a Config struct representing options defined for the application
pub fn get_config() -> Config {
    let config: Config = Figment::new()
        .merge(Yaml::file(get_config_path()))
        .merge(Env::prefixed("SKEL_"))
        .extract()
        .unwrap_or(Config {
            template_dir: String::from("."),
        });
    config
}

/// Reads files from `template` and copies it into `target`
/// if the `target` does not exist it will be created for you
pub fn use_template(template: &str, target: &str) {
    if !Path::new(target).exists() {
        println!("{target} does not exist. Creating...");
        fs::create_dir_all(target).unwrap();
    }

    let entries = fs::read_dir(template)
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
