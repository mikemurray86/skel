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

fn print_help(name: &str) {
    println!(
        "\nUsage: {name} <template> <target>\n\
              \ttemplate: the configured template to load\n\
              \ttarget: the directory to load the template into\n"
    );
}

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
