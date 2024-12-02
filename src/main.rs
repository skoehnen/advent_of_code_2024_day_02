mod verify_log;

use std::fs;
use crate::verify_log::verify;

fn main() {
    let file_path = "data/test.data";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");

    println!(" ");

    verify(contents);
}