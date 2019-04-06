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


## Programming Concepts ##
### Keywords and Identifiers ####
  * Keywords are reserved terms in Rust
  * Identifiers must start with an alphabetic character OR _.
  * Reuse keywords with `r#`
### Variables and Mutability ###
#### let/const ####
  * Define variables with `let`.
  * Make variables mutable with `let mut ...`
  * Define constants with `const`
  * Constants must be annotated
#### Types ####
  * Rust is statically typed
  * Scalar Types:
	* integers
		* `u32`: unsigned integer (32 bits)
		* `i32`: signed integer (32 bits)
	* floating-point numbers
		* `f32`: floating-point number (32 bits)
	* Booleans
		* `true`
		* `false`
	* Characters: unicode scalar value 
