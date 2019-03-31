# Learning Rust #

## Hello World! ##


File: main.rs
``` rust
fn main() {
	println!("Hello world!");
}
```

Compile with `rustc main.rs`

## Cargo ##
Cargo is Rust's build system and package manager.
Create new project with:
`cargo new`

Package configuration is stored in `Cargo.toml`

Build with `cargo build`. Binary will be created in `/target/debug` directory.
Build and run with `cargo run`.
Check if project will compile without outputting binary or running with: `cargo check`
