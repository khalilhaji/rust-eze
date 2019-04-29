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
  * Compound Types:
	* Tuples:
		* Fixed length
		``` rust
			fn main() {
				let tup: (u32, f64, u8) = (500, 6.4, 1);
				let (x, y, z) = tup;
				println!("The value of y is: {}", y);
				println!("The second value of the tuple is: {}", tup.1);
			}
		```
	* Arrays:
		* All elements must be same data type.
		* Fixed size
		* 

### Ownership ###
	* Rust has block-scope.
	* If a variable is reassigned and the type does not have the Copy trait set, the value is moved to the new variable. The old variable is invalidated.
	* Memory is automatically freed by calling drop function when the owner of a value goes out of scope.
	* If `Copy` trait is set, reassignment copies the value as is the case with primitive/atomic types.
