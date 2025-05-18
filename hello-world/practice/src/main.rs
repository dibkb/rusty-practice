fn main() {
    let mut sentence = String::from("Hello world");
    println!("{}", append_rust_suffix(&mut sentence));
}

fn append_rust_suffix(sen: &mut String) -> &mut String {
    sen.push_str(" from rust");
    sen
}
