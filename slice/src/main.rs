fn main() {
    let s = String::from("Hello world");
    let _first_word_end = first_word(&s);

    let hello = &s[0..5];
    let _hello02 = &s[..5];
    let world = &s[6..11];

    // println!("The first word in the following Sentence {}\nEnds at position {}", s, first_word_end);
    println!("First Word: {}\nSecond Word: {}", hello, world);
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

// Notes to myself:
// When I use the first_word method the value isn´t bound to variable.
// If i get back the value 5 and change the string it will introduce a wrong result
// Slices refernce a specific part of a string
// The range sytanx allow you to write either &s[x..y]; or if you want to start at index 0 you can just
// write &s[..y]; You can also ignore the ending_index if you want the reference to go to the last
// index so for example &s[5..len] is the same as &s[5..];
