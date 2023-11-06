fn main() {
    let s1 = String::from("Hello");

    let (s2, len) = calculate_length(s1);
    
    println!("The length of {} is: {}", s2, len);

}

fn _take_ownership() -> String {
    let string = String::from("S1 String");
    string
}

fn _give_and_take_ownership(string: String) -> String {
    string
}

fn calculate_length(string: String) -> (String, usize) {
    let length = string.len();

    (string, length)
}
