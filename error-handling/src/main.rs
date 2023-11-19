use std::{fs::File, io::{self, ErrorKind, Read}};

#[allow(unused)]
fn main() {
    let user = read_user_from_file();
    dbg!(&user);
}

#[allow(unused)]
fn read_user_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("username.txt");
    
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

#[allow(unused)]
fn result_test() {
    let file_result = File::open("text.txt");

    let file = {
        match file_result {
        Ok(file) => {
            file
        },
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => match File::create("text.txt") {
                    Ok(fc) => fc,
                    Err(error) => panic!("Following error occured when creating the file: {}",error),
                },
                other_error => panic!("Following error occured when trying to open the file: {}", other_error),
            } 
        }
        } 
    };
    dbg!(file);
}

#[allow(unused)]
fn result_test_without_match() -> File {
    let file_result = File::open("text.txt");

    let file = {
        File::open("text.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("text.txt").unwrap_or_else(|error| {
                    panic!("Error trying to create file: {}", error)
                })
            }
            else {
                panic!("Error occured when trying to open the file: {}", error);
            }
        })
    };

    file    
}
