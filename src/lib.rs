//! # cargo renamepkg
//!
//! `cargo renamepkg` is a simple utility to help you rename a cargo package
//!  when you may create packages with unexpected names or just want to change its name.
//!
//!
//! # Usage
//!
//! ```shell
//! cargo renamepkg <TARGET_PATH> <NEW_NAME>
//! ```
extern crate toml_edit;
use clap::ArgMatches;
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;
use toml_edit::{value, Document};

pub fn run(config: ArgMatches) -> Result<(), Box<dyn Error>> {
    let config = config.subcommand_matches("renamepkg").unwrap();
    let target_path = PathBuf::from(config.value_of("target_path").unwrap());
    let new_name = config.value_of("new_name").unwrap();

    // find the Cargo.toml
    let mut toml_file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(false)
        .open(target_path.join("Cargo.toml"))?;
    let mut toml_contents = String::new();
    toml_file.read_to_string(&mut toml_contents)?;

    let mut doc = match toml_contents.parse::<Document>() {
        Ok(doc) => doc,
        Err(_) => return Err("failed parsing Cargo.toml".into()),
    };

    let old_name = doc["package"]["name"].as_str().unwrap_or("");
    if old_name.is_empty() {
        return Err("no package name in Cargo.toml".into());
    }
    if old_name == new_name {
        println!("package already named as [{}]", new_name);
        return Ok(());
    }

    println!(
        "you are going to rename this package from [{}] to [{}] (y/N)",
        old_name, new_name
    );
    let mut input = String::new();
    if let Err(err) = io::stdin().read_line(&mut input) {
        return Err(err.into());
    };

    if input.trim().to_lowercase() != "y" {
        println!("cancelled");
        return Ok(());
    }

    // update Cargo.toml
    doc["package"]["name"] = value(new_name);
    // fs::write(toml_file.get_path(), );
    fs::write(&target_path.join("Cargo.toml"), doc.to_string().as_bytes())?;

    let new_path = target_path.parent().unwrap().join(&new_name);
    fs::rename(&target_path, &new_path)?;

    println!("completed");
    Ok(())
}

