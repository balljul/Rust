fn main() {
    let s1 = String::from("Hello World");
    let len = calculate_length(&s1);

    println!("The length of {} is {}", s1, len);
}

fn _old_function() {
    let _s1 = String::from("Hello");
//    let (s2, len) = calculate_length(s1);
    
  //  println!("The length of {} is: {}", s2, len);

}

fn _take_ownership() -> String {
    let string = String::from("S1 String");
    string
}

fn _give_and_take_ownership(string: String) -> String {
    string
}

fn _old_calculate_length(string: String) -> (String, usize) {
    let length = string.len();

    (string, length)
}


fn calculate_length(string: &String) -> usize {
    string.len()
}

// Notes to myself:
// When useing a reference it is garanteed to point at a value with a specific type in the heap
// Also when using references you dont transfer ownership
//
