# cargo-renamepkg
This is a simple utility to help you rename a cargo package when you may create a package with an unexpected name or just want to change its name.

## Use Case
Suppose that you wanted to do this:
```shell
cargo new my_project
```

But actually you did this:
```shell
cargo new my_projcet
```

When facing such an embarrassing situation, just use this utility:
```shell
cargo renamepkg my_projcet my_project
```
And `cargo renamepkg` does two things for you: 
- renames the directory
- replaces the package name with new one in `Cargo.toml`

## Usage
```shell
cargo renamepkg <TARGET_PATH> <NEW_NAME>
```

## Installation
```shell
cargo install cargo-renamepkg
```

## Contribution
Any questions/advices/contributions are welcomed!