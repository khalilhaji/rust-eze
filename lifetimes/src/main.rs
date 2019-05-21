struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("call me Ishamel. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt {part: first_sentence};
}
