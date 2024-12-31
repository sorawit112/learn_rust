use std::fs::{read, File};
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error>{
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn main() {
    let read_result = read_username_from_file();
    match read_result {
        Ok(read_string) => println!("{read_string}"),
        Err(error) => println!("{error}")
    }
}
