fn main() {
    let s = String::from("Hello world");
    let first_word_end = first_word(&s);

    println!("The first word in the following Sentence {}\nEnds at position {}", s, first_word_end);
}

fn first_word(string: &String) -> usize {
    let bytes = string.as_bytes();

    for (index, &character) in bytes.iter().enumerate() {
        if character == b' ' {
            return index;
        }
    }
    string.len()
}
