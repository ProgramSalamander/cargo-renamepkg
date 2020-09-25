#[macro_use]
extern crate clap;
use clap::{App, Arg, SubCommand};
use std::process;

fn main() {
    let config = App::new("cargo renamepkg")
        .bin_name("cargo")
        .subcommand(
            SubCommand::with_name("renamepkg")
                .author(crate_authors!())
                .version(crate_version!())
                .about("A simple utility to help you rename a package")
                .arg(
                    Arg::with_name("target_path")
                        .value_name("TARGET_PATH")
                        .help("Sets the path of the target package")
                        .index(1)
                        .required(true),
                )
                .arg(
                    Arg::with_name("new_name")
                        .value_name("NEW_NAME")
                        .help("Sets the new name of the target package")
                        .index(2)
                        .required(true),
                ),
        )
        .get_matches();

    if let Err(e) = cargo_renamepkg::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}
