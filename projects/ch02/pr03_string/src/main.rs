#![allow(unused)]

fn main() {
    let person_name_slice = "Donald Mallard";
    let person_name_string = person_name_slice.to_string();
    let person_name_string = String::from("Donal Mallard");

    let person_name_slice = (&person_name_string).as_str();

    let duck = "Duck";
    let airlines = "Airlines";
    let airline_name = format!("{} {}", duck, airlines);
    println!("{}", airline_name);
    // println!("Hello, world!");

    let mut slogan = String::new();
    slogan.push_str("We hit the ground");
    slogan.push(' ');
    slogan += "every time";
    println!("{slogan}"); 
}
