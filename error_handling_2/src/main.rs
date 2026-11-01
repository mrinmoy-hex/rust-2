#![allow(unused)]
use std::fs::File;
use std::io::{self, Read};

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        // If the file is missing, create it; other open failures are treated as fatal below.
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Something went wrong.");
            }
        },
    };

    // `expect` unwraps a successful open or panics with this message if opening fails.
    // let test_file = File::open("test.txt").expect("test.txt should be included in this project");
}

// Return the file contents, or pass the I/O error back to the caller.
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        // `return Err(e)` exits early and preserves the original I/O error.
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// The `?` operator returns an error to the caller if either file operation fails.
fn read_username_from_file_refactor() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("test.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
