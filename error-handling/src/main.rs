use std::fs::File;

#[allow(unused)]
fn main() {
    result_test();
}

#[allow(unused)]
fn result_test() {
    let file_result = File::open("text.tst");

    let file = match file_result {
        Ok(file) => file,
        Err(error) => panic!("\nFollowing error when trying to open the file: \n{}\n", error),
    };
    

}
