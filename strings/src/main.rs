#[allow(unused)]
fn main() {
    {
        let mut s = String::from("Initial ");
        s.push_str("Commit");
        s.push(' ');
        s.push('J');
        s.push('B');
        // assert_eq!(s, "Initial Commit", "Message One does not equal Message two!");
    }

    {
        let s2 = String::from("ADDED ");
        let s3 = String::from("Features");
        let s4 = s2 + &s3; // s2 gets moved so it wont be able to be used again
    }
    
    let s1 = String::from("REWROTE");
    let s2 = String::from("OLD");
    let s3 = String::from("BACKEND!!!");

    let s = format!("{} {} {}", s1, s2, s3);

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
