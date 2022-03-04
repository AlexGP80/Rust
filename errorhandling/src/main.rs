#![allow(unused)]
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];
    //
    // v[99];
    //
    // let f = File::open("hello.txt");
    //
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         }
    //     },
    // };

    // Shortcuts for Panic on Error: unwrap and expect
    // let f = File::open("hello.txt").expect("Failed to open hello.txt");

    let username = read_username_from_file().expect("Cagada del mono");
}
