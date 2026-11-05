#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        let hour = hour as f64;
        let minutes = minutes as f64;
        // const UNIT: f64 = 360.0 / (12.0 * 5.0);
        let min_angle = minutes * 6.0;
        let hour_angle = (minutes / 60.0 + hour % 12.0) * 30.0;

        let b1 = (hour_angle - min_angle + 360.0) % 360.0;

        let b2 = (360.0 - b1 + 360.0) % 360.0;

        b1.min(b2)
    }
}

fn main() {}
