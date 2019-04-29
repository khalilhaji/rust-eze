use std::fs::File;

fn main( {
    match helper() {
        Ok(i) => println!("{}", i),
        _ => (),
    };
}

fn helper() -> Result<i32, String> {
}
