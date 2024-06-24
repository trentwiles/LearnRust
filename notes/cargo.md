# Basic Cargo Guide

## Project Commands
1. New cargo project: `cargo new [project_name]` (a new Cargo.toml file has been made)
2. Compile: `cargo build`
3. Run: `cargo run`

## Crates
A crate is a package for rust. To add one, you can manually add it to the Cargo.toml file, like so:

```
[dependencies]
rand = "0.8.5"
```