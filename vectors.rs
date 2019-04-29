fn main() {
    let s = String::from("नमस्ते");
    for c in s.bytes() {
        println!("{}", c);
    }

}
