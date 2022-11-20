#![allow(unused)]

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let filename = "customer.json";

    match File::open(filename) {
        Ok(file) => {
            println!("{:#?}", file);
        }
        Err(error) => {
            // println!("{:#?}", error);
            match error.kind() {
                ErrorKind::NotFound => match File::create(filename) {
                    Ok(file) => {
                        println!("File created");
                    }
                    Err(err) => {
                        println!("{:#?}", error);
                    }
                },
                _ => {
                    println!("{:#?}", error);
                }
            }
        }
    }

    // let file_handler = File::open(filename);

    //    panic!("Sorry, I've panicked.");
}
