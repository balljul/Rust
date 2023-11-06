struct Color(i32, i32, i32); // This is a Tuple struct and it cannot be accessed with . but it can
                             // be destroyed like a tuple

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u128,
}

fn main() {
    let user01 = build_user(String::from("balljul"), String::from("contact@juliusball.com"));
    let user02 = User {
        active: false,
        username: user01.username.clone(),
        email: String::from("juliusball197@gmail.com"),
        sign_in_count: 4,
    };
    let user03 = User {
        email: String::from("juliusball2006@gmail.com"),
        ..user01
    };

    let _black = Color(0, 0, 0);
    println!("The data of user01:\nActive: {}\nUsername: {}\nEmail: {}\nsign_in_count: {}", user01.active, user01.username, user01.email, user01.sign_in_count);
    println!("The data of user01:\nActive: {}\nUsername: {}\nEmail: {}\nsign_in_count: {}", user02.active, user02.username, user02.email, user02.sign_in_count);
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}


// We define structures with the struct keyword
// We can create an instance wby creating a variable and then writing = Structname and then
// defining the values. To make a instance mutable we must make the variable mutable. You can reuse
// the values of one instance in another instance. Datatypes which dont have the copy trait f.e.
// the String type will get moved.
// Each struct you define will be it's own type
