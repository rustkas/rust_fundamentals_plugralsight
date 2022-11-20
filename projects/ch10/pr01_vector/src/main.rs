fn main() {
    let mut flights: Vec<&str> = Vec::new();

    flights.push("DA113\tto Boston departs at 06:20");
    flights.push("DA98\tto London departs at 09:43");
    flights.push("DA428\tto Salt departs at 12:05");
    flights.push("DA41\tto Berlin departs at 15:30");
    flights.push("DA2815\tto Nashville departs at 17:11");

    for flight in flights.iter() {
        println!("{flight}");
    }

    println!("The third entry in the vector is: {}", flights[2]);

    let fourth = flights.get(3);
    match fourth {
        Some(fourth) => {
            println!("{fourth}");
        }
        _ => {}
    }

    if let Some(flight_value) = flights.get(4) {
        println!("{flight_value}");
    }

    flights.insert(2, "DA918\t\to Orlando departs at 11:12");

    flights.remove(1);
}
