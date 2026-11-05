#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
        // Find the maximum square length by taking the minimum side of each rectangle
        let mut max_len = 0;
        let mut count = 0;

        // First find the maximum possible square length
        for rectangle in &rectangles {
            let side_len = rectangle[0].min(rectangle[1]);
            max_len = max_len.max(side_len);
        }

        // Then count how many rectangles can form a square of the max length
        for rectangle in &rectangles {
            let side_len = rectangle[0].min(rectangle[1]);
            if side_len == max_len {
                count += 1;
            }
        }

        count
    }
}
fn main() {}
