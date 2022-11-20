#![allow(unused)]

struct Waypoint {
    name: String,
    latitude: f64,
    longitude: f64,
}

struct Segment {
    start: Waypoint,
    end: Waypoint,
}

impl Segment {
    fn new(start: Waypoint, end: Waypoint) -> Self {
        Self { start, end }
    }

    fn distance(&self) -> f32 {
        const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;

        let kcle_latitude_degrees: f64 = 41.4075;
        let kcle_longitude_degrees: f64 = -81.851111;

        let kslc_latitude_degrees: f64 = 40.7861;
        let kslc_longitude_degrees: f64 = -111.9822;

        let kcle_latitude_radius = kcle_latitude_degrees.to_radians();
        let kslc_latitude_radius = kslc_latitude_degrees.to_radians();

        let delta_latitude = (kcle_latitude_degrees - kslc_latitude_degrees).to_radians();
        let delta_longitude = (kcle_longitude_degrees - kslc_longitude_degrees).to_radians();

        let inner_central_angle = f64::powi((delta_latitude / 2.0).sin(), 2)
            + kcle_latitude_radius.cos()
                * kslc_latitude_radius.cos()
                * f64::powi((delta_longitude / 2.0).sin(), 2);
        let central_angle = 2.0 * inner_central_angle.sqrt().asin();
        let distance = EARTH_RADIUS_IN_KILOMETERS * central_angle;
        distance as f32
    }
}

fn main() {
    const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;

    let kcle = Waypoint {
        name: "KCLE".to_string(),
        latitude: 41.4075,
        longitude: -81.851111,
    };

    let kslc = Waypoint {
        name: "KSLC".to_string(),
        ..kcle
    };

    let kcle_kslc = Segment::new(kcle, kslc);
    let distance = kcle_kslc.distance();
    println!(
        "The distance between the two points is {:.1} kilometers",
        distance
    );
    // println!("Hello, world!");
}
