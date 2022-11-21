#![allow(unused)]

#[derive(Debug)]
struct NavAid<T, U> {
    name: String,
    frequency: T,
    data: U
}

// fn mutliply<T> (operand1: T, operand2: T) -> T {

// }

fn main() {
    let vor = NavAid {
        name: String::from("DQN"),
        frequency: 114.5,
        data: String::from("DQN is VOR")
    };

    let ndb_data: Option<String> = Option::None;
    let ndb = NavAid {
        name: String::from("HKF"),
        frequency: 239,
        data: ndb_data
    };

    println!("VOR information is {:?}", vor);
    println!("NDB information is {:?}", ndb);
}
