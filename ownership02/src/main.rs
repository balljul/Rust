fn main() {
    let s1 = take_ownership();

    let s2 = String::from("S2 String");
    let s3 = give_and_take_ownership(s2);

    println!("S1: {}\nS3: {}\n", s1, s3);
}

fn take_ownership() -> String {
    let string = String::from("S1 String");
    string
}

fn give_and_take_ownership(string: String) -> String {
    string
}
