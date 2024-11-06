struct Solution;

impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut expect = heights.clone();
        expect.sort_unstable();
        (0..expect.len()).fold(0, |mut acc, i| {
            if expect[i] != heights[i] {
                acc += 1;
            }
            acc
        })
    }
}

fn main() {}
