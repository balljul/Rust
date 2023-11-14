#[allow(unused)]
fn main() {
    let mut s = String::from("Initial ");
    s.push_str("Commit");

    assert_eq!(s, "Initial Commit", "Message One does not equal Message two!");

    dbg!(&s);
}

#[allow(unused)]
fn updating_string() {}

#[allow(unused)]
fn string_basics() {
    let data = "Yay";
    let s = data.to_string();

    {
        let s = "Initial Commit".to_string(); 
        let s = String::from("Inital Commit with String::from"); // Same as above
        dbg!(&s);
    }

    {
        // Strings are UTF8 encoded so I can store every properly encoded value in it
        let s = String::from("Здравствуйте"); 
        dbg!(&s);
    }


    dbg!(&s);
}
