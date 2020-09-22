# cargo-renamepkg
This is a simple tool to help you rename a cargo package when you may create packages with unexpected names or just want to change its name.

*Note:* in current version, it only works in a package root.

## Installation
Simply run the install command and then you can use `cargo renamepkg`.

```shell
cargo install --path .
```

## Usage 
```shell
cd some_package_root
cargo renamepkg new_name
```