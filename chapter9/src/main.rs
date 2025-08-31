use std::{
    fs::{File, OpenOptions},
    io::{self, ErrorKind, Read, Write},
};

fn main() {
    let f = File::open("hello.txt");

    match f {
        Ok(file) => {
            println!("Find file");
            file
        }
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => {
                panic!("Cannot crate file {:?}", e)
            }
        },
        Err(error) => {
            panic!("Error {:?}", error)
        }
    };

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("hello.txt")
        .expect("open failed");
    let _ = file.write(b"sample user").expect("error");

    let username = read_username_from_file().expect("user name?");
    println!("{}", username);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
