//! # cargo renamepkg
//! 
//! `cargo renamepkg` is a simple tool to help you rename a cargo package
//! when you may create packages with unexpected names or just want to change its name.
//! 
//! *Note:* in current version, it only works in a package root.
//! 
//! # Usage
//! 
//! ```shell
//! cargo renamepkg new_name
//! ```
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fs;
use std::io;

pub struct Config {
    pub new_name: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments\nUsage:\n \tcargo renamepkg [NEW_NAME]");
        }
        // skip the exe path
        args.next();

        // skip if the next argument is "renamepkg"
        let arg = args.next().unwrap();
        let new_name = if arg == "renamepkg" {
            match args.next() {
                Some(name) => name,
                None => return Err("didn't get the new name for this package"),
            }
        } else {
            arg
        };

        Ok(Config { new_name })
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct CargoToml {
    package: Package,
}

#[derive(Serialize, Deserialize, Debug)]
struct Package {
    name: String,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let current_dir = env::current_dir()?;
    let mut cargo_toml: CargoToml = match fs::read_to_string(current_dir.join("Cargo.toml")) {
        Ok(contents) => toml::from_str(&contents)?,
        Err(_) => return Err("cannot find Cargo.toml in current directory\nHelp: try to use this command in the package root".into()),
    };

    let old_name = cargo_toml.package.name;
    if old_name.is_empty() {
        return Err("No package name found in Cargo.toml".into());
    }
    let new_name = config.new_name;

    if old_name == new_name {
        println!("This package is already named as [{}]", new_name);
        return Ok(())
    }

    println!(
        "Are you going to rename this package from [{}] to [{}]? (y/N)",
        old_name, new_name
    );
    let mut input = String::new();
    if let Err(err) = io::stdin().read_line(&mut input) {
        return Err(err.into())
    };

    if input.trim() != "y" {
        println!("Cancelled");
        return Ok(());
    }

    let new_dir = current_dir.parent().unwrap().join(&new_name);
    fs::rename(&current_dir, &new_dir)?;
    cargo_toml.package.name = new_name;
    fs::write(new_dir.join("Cargo.toml"), toml::to_string_pretty(&cargo_toml)?)?;

    println!("Completed");

    Ok(())
}
