fn main() {
// Strings:

    { // Varablie string_example doesnt exist
        let mut _string_example = String::from("Hello");
        _string_example.push_str(" World");
    } // Here the variable goes out of scope and its memory gets automaticly freed

let s1 = String::from("Hello world");
let s2 = s1;
let s3 = s2.clone();

println!("s1 is not valid anymore.\ns2: {}\ns3: {}\n", s2, s3);

}

// Notes to myself:
// The Stack works with the Last in, First out principle
// The Heap is slower but you get a pointer and you can access specific data
// literal string are not mutable 
// The String type is mutable
