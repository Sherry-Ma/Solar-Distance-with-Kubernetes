/*A library that returns back the solar distance of planets */
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref SOLAR_DISTANCES: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();

        map.insert("Mercury", "0.39");
        map.insert("Venus", "0.72");
        map.insert("Earth", "1.0");
        map.insert("Mars", "1.52");
        map.insert("Jupiter", "5.20");
        map.insert("Saturn", "9.58");
        map.insert("Uranus", "19.18");
        map.insert("Neptune", "30.07");

        map
    };
}

pub fn get_solar_distance(planet_name: &'static str) -> &'static str {
    //let distance = SOLAR_DISTANCES.get(planet_name).copied();
    SOLAR_DISTANCES[planet_name]
}
