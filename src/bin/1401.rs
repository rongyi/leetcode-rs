#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn check_overlap(
        radius: i32,
        x_center: i32,
        y_center: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> bool {
        let closest_x = x_center.max(x1).min(x2);
        let closest_y = y_center.max(y1).min(y2);

        let dx = closest_x - x_center;
        let dy = closest_y - y_center;

        let sq_dist = dx * dx + dy * dy;

        sq_dist <= radius * radius
    }
}
fn main() {}
