use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line!");
    input = input.trim().to_string();
    latinize(&mut input);
    println!("{}", input);
}

fn latinize(input_str: &mut String) {
    if let Some(first) = input_str.chars().next() {
        if is_vowel(first) {
            input_str.push_str("-hay");
        } else {
            input_str.remove(0);
            input_str.push_str(&format!("-{}ay", first));
        }
    }
}

fn is_vowel(c: char) -> bool {
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}
