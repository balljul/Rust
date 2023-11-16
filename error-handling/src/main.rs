use std::{fs::File, io::ErrorKind};

#[allow(unused)]
fn main() {
    result_test();
}

#[allow(unused)]
fn result_test() {
    let file_result = File::open("text.txt");

    let file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("text.txt") {
                Ok(fc) => fc,
                Err(error) => panic!("Following error occured: {}",error),
            },
            other_error => panic!("Following error occured: {}", other_error),
        }
    };
    

}
