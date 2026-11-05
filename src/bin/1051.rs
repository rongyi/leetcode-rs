struct Solution;

fn main() {}

impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut order = heights.clone();
        order.sort_unstable();

        order
            .into_iter()
            .zip(heights.into_iter())
            .filter(|p| p.0 != p.1)
            .count() as _
    }
}
