fn main() {
    let a = String::from("test");
    let mut my_iter = a.chars();
    if let Some(b) = my_iter.next() {
        println!("{}", b);
        if let Some(c) = my_iter.next() {
            println!("{}", c);
        }
    }
    println!("{}", a);
}
