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
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;
use std::{env::current_dir, error::Error};
use toml::Value;
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

    fs::write(&target_path.join("Cargo.toml"), doc.to_string().as_bytes())?;
    let new_path = target_path.parent().unwrap().join(&new_name);
    fs::rename(&target_path, &new_path)?;

    // find the Cargo.toml for the workspace
    let workspace_toml_path = current_dir()
        .expect("should have current dir")
        .join("Cargo.toml");
    if workspace_toml_path.is_file() {
        let mut toml_workspace_file = OpenOptions::new()
            .read(true)
            .write(true)
            .append(false)
            .open(workspace_toml_path)?;
        let mut toml_contents_workspace = String::new();
        toml_workspace_file.read_to_string(&mut toml_contents_workspace)?;

        let mut doc_workspace: Value = toml::from_str(toml_contents_workspace.as_str())?;
        if let Some(members_mut) = doc_workspace
            .get_mut("workspace")
            .and_then(|w| w.get_mut("members"))
            .and_then(Value::as_array_mut)
        {
            let target_name = target_path
                .file_name()
                .expect("Expected the target path to have valid file name. This is a YOU problem.")
                .to_string_lossy();
            if let Some(index) = members_mut
                .iter()
                .position(|member| *member == Value::String(target_name.to_string()))
            {
                members_mut[index] = Value::String(new_name.to_string());
            }
        }
        fs::write(
            current_dir()
                .expect("should have current dir")
                .join("Cargo.toml"),
            doc_workspace.to_string().as_bytes(),
        )?;
    }

    println!("completed");
    Ok(())
}
