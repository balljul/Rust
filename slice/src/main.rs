fn _old_ma_in() {
    let s = String::from("Bye World");
    let word = _first_word(&s[..]);

    let s2 = "Hello World";
    let second_word = _first_word(&s2);

    let _hello = &s[0..5];
    let _hello02 = &s[..5];
    let _world = &s[6..9];

    // println!("The first word in the following Sentence {}\nEnds at position {}", s, first_word_end);
    // println!("First Word: {}\nSecond Word: {}", hello, world);
    println!("The words are: {} and {}", word, second_word);
}

fn _first_word (string: &str) -> &str {
    let bytes = string.as_bytes();

    for (i, &character ) in bytes.iter().enumerate() {
        if character == b' ' {
            return &string[0..i];
        }
    }

    &string[..]
}

fn _first_word_without_slice (string: &String) -> usize {
    let bytes = string.as_bytes();

    for (index, &character) in bytes.iter().enumerate() {
        if character == b' ' {
            return index;
        }
    }
    string.len()
}

fn main() {
    let a = [1, 2, 3, 4, 5];
    
    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);

}
// Notes to myself:
// When I use the first_word method the value isn´t bound to variable.
// If i get back the value 5 and change the string it will introduce a wrong result
// Slices refernce a specific part of a string
// The range sytanx allow you to write either &s[x..y]; or if you want to start at index 0 you can just
// write &s[..y]; You can also ignore the ending_index if you want the reference to go to the last
// index so for example &s[5..len] is the same as &s[5..];
// I can also drop both values if I want to take a slice of the whole string: &s[..];
// I used the input type of the first_word function to &str so i can take in literal strings
// Literal strings themself are just a slice of the whole word we save in it not a string
