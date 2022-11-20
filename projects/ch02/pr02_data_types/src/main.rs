#![allow(unused)]
fn main() {
    let location: [f32;2] = [41.4094069, -81.8546911];

    let location = [0.0; 10000];

    // tuple
    let location = ("KCLE", 41.4094069, -81.8546911);
    println!("Location name: {}, latitude: {}, longitude: {}", location.0, location.1, location.2);

    // Destructuring
    let (name, latitude, longitude) = location;
    println!("Location name: {}, latitude: {}, longitude: {}", name, latitude, longitude);

}
