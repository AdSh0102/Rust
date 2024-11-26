fn main() {
    let s: String = String::from("Hello, world");
    let first_word: &str = first_word(&s);
    println!("{first_word}");

}

fn first_word(s:&String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }
    return &s[..];
}


