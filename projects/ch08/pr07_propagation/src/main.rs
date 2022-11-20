use std::io::{Error, Read};
use std::fs::File;


fn main() {
    let filename = "data.txt";
    let file_data = read_file(filename);
    match file_data {
        Ok(data) => {
            println!("{}", data);
        },
        Err(_) => {
            
        }
    }
    // println!("{file_data}");
}

fn read_file(filename: &str) -> Result<String, Error> {
    let mut file_handler = File::open(filename)?;
    let mut file_data = String::new();
    file_handler.read_to_string(&mut file_data)?;
    Ok(file_data)

}