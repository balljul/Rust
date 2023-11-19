use std::{fs::{self, File}, io::{self, ErrorKind}};

#[allow(unused)]
fn main() {
    dbg!(last_char_of_first_line("JAJAJAA\nNeineNeinenin"));
    dbg!(last_char_of_first_line(""));
}

#[allow(unused)]
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()  
}

#[allow(unused)]
fn read_user_from_file() -> Result<String, io::Error> {
//    let mut username = String::new();
//    File::open("username.txt")?.read_to_string(&mut username)?;
//    Ok(username)

    fs::read_to_string("username.txt") 
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
