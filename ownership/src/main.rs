fn _string_ownerships() {
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

fn give_me_string(random_string: String) {
    // The strings value getx moved to the variable random string
    println!("The string you gave me is: {}\n", random_string);
    // Here the scope ends and the mem gets freed and the string from is invalid
}

fn give_me_integer(random_int: i32) {
    // The intereger just gets copied so its still valid in the main function
    println!("The int you gave me is: {}\n", random_int);
    // Here the int gets dropped and nothing special happens
}

fn main() {
    let x = 5;
    let y = x;

    let mystr = String::from("Test12345");

    give_me_string(mystr);
    give_me_integer(y);

}


// Notes to myself:
// The Stack works with the Last in, First out principle
// The Heap is slower but you get a pointer and you can access specific data
// literal string are not mutable 
// The String type is mutable
