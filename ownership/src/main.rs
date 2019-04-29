fn main() {
    let mut x = String::from("hello");
    use_imm(&x);
    x.push_str(", world");
    use_imm(&x);
}


fn use_imm(x: &String) {
    println!("{}", x);
}
