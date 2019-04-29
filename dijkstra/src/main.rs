use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn main() {
    let mut a: HashMap<String, String> = HashMap::new();
    a.insert(String::from("test"), String::from("entry123"));
    let b = a.entry(String::from("test"));
    match b {
        Entry::Vacant(_) => println!("yay"),
        Entry::Occupied(something) => println!("{:?}", something.get()),
    };
}
