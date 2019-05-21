use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let query = &args[1];
    let filename = &args[2];

    println!("The query is {}", query);
    println!("The filename is {}", filename);
}
