fn main() {
    for index in 1..=10 {
        println!("{index}");
    }

    let duck_aircraft = ["Boeing 737", "Boeing 767", "Boeing 787", "Airbus 319", "Airbus 320"];
    for aircraft in duck_aircraft.iter() {
        println!("{aircraft}");
    }
}
