fn main() {
    let mut s1 = String::from("hello");
    s1 = take_ownership(s1);
    take_ownership(s1);
}

fn take_ownership(str: String) -> String{
    str
}
