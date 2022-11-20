#![allow(unused)]
#![allow(irrefutable_let_patterns)]

fn main() {
    let mut animal = "Duck";
    if animal.is_ascii() {
        animal = "Dog";
    }
    // match animal {
    //     "Duck" => {
    //         println!("Quack");
    //     },
    //     _ => {
    //         println!("All quiet out here");
    //     }
    // }

    if let animal = "Duck" {
        println!("Quack");
    }

}
