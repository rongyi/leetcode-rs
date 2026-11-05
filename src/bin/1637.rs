#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
        // Sort the points based on x-coordinates
        let mut sorted_x = points.iter().map(|p| p[0]).collect::<Vec<i32>>();
        sorted_x.sort_unstable();

        // Find the maximum width between adjacent points
        let mut max_width = 0;
        for i in 1..sorted_x.len() {
            max_width = max_width.max(sorted_x[i] - sorted_x[i - 1]);
        }

        max_width
    }
}
fn main() {}
