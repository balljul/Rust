fn main() {
    let mut s1 = String::from("Hello");
    let len = calculate_length(&s1);
    println!("The length of {} is {}", s1, len);

    {
        let _r1 = &mut s1;
    }
    let r2 = &mut s1;

    println!("{}", r2);
}

fn _change_string(string: &mut String) {
    string.push_str(" World");
}

fn calculate_length(string: &String) -> usize {
    string.len()
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

fn _old_calculate_len(string: String) -> (String, usize) {
    let length = string.len();

    (string, length)
}


// Notes to myself:
// When useing a reference it is garanteed to point at a value with a specific type in the heap
// Also when using references you dont transfer ownership
// A refernce stores a reference to the pointer of the value on the stack which stores the pointer
// to the stored data on the heap
// The action of referencing a value is called Borrowing
// Another important factor is that, by default, you cant modify a borrowed value
// We can modify a borrowed value by adding mut
// At any point rust allows either one mutable reference or multiple immutable references
