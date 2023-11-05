fn main() {
// Strings:
let mut string_example = String::from("Hello");
string_example.push_str(" World");

println!("{}", string_example);
}

// Notes to myself:
// The Stack works with the Last in, First out principle
// The Heap is slower but you get a pointer and you can access specific data
// literal string are not mutable 
// The String type is mutable
