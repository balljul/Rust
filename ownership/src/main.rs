fn main() {
// Strings:

    { // Varablie string_example doesnt exist
        let mut string_example = String::from("Hello");
        string_example.push_str(" World");
        println!("{}", string_example);
    } // Here the variable goes out of scope and its memory gets automaticly freed
}

// Notes to myself:
// The Stack works with the Last in, First out principle
// The Heap is slower but you get a pointer and you can access specific data
// literal string are not mutable 
// The String type is mutable
