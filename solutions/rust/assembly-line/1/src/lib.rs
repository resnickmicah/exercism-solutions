// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let float_speed = speed as f64;
    match(speed) {
        0..=4 => float_speed * 221.0,
        5..=8 => float_speed * 221.0 * 0.9,
        9..=10 => float_speed * 221.0 * 0.77,
        _ => panic!("Invalid speed")
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
