# Rust - basics

## Setup

### Installation

Rust can be installed using the following command:

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Compiling rust

The compiler for rust can be invoked using `rustc module.rs`.

This will generate a binary for the module passed

## Cargo

Cargo is the package manager for rust. Each package is called a crate.

The repository for cargo packages is available at
[Crates.io](https://crates.io/)

### Start a project

To create a new rust package, run `cargo new <package_name>`

This will create a new directory containing:

- `src/main.rs` - a basic hello world script
- `Cargo.toml` - rust package definition file for cargo

### Build a project

To build a project created with cargo, run `cargo build`

By default the build is created in debug mode (without optimizations). To create
a production build, run `cargo build --release`

The build is placed as an executable the `target` directory.

- `target/debug/<package_name>` for debug build
- `target/release/<package_name>` for release build

### Run a project

To run a project created with cargo, run `cargo run`
