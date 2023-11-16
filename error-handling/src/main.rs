use std::fs::File;
use std::fs::read_to_string;

#[allow(unused)]
fn main() {
    let file_result = File::open("text.txt");
    let file_content = read_to_string("text.txt");
    dbg!(file_content);
}
