#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let (x0, y0) = (coordinates[0][0], coordinates[0][1]);
        let (x1, y1) = (coordinates[1][0], coordinates[1][1]);
        let dx = x0 - x1;
        let dy = y0 - y1;

        for p in coordinates.iter().skip(2) {
            let dx2 = p[0] - x0;
            let dy2 = p[1] - y0;
            if dy * dx2 != dy2 * dx {
                return false;
            }
        }

        true
    }
}

fn main() {}
