# cargo-renamepkg
This is a simple tool to help you rename a cargo package when you may create packages with unexpected names or just want to change its name.

Suppose that you just used `cargo new` to create a package whose name is expected to be `my_project`, but you find that you mistyped and now its name is `my_projcet`. 

`cargo renamepkg` can help a lot when facing such an embarrassing situation, just use:
```shell
cargo renamepkg my_project
```

This tool only does two things: 
- renames the directory
- replaces the package name with new one in `Cargo.toml`

*Note:* in current version, it only works in a package root.

## Installation
Simply run the install command and then you can use `cargo renamepkg`.

```shell
cargo install --path .
```

## Usage 
```shell
cargo renamepkg new_name
```

## Contribution
Any questions/advices/contributions are welcomed!