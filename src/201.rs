struct Solution;

impl Solution {
    pub fn range_bitwise_and(mut left: i32, mut right: i32) -> i32 {
        let mut step = 0;
        // common prefix
        while left != right {
            left >>= 1;
            right >>= 1;
            step += 1;
        }

        left << step
    }
}

fn main() {}
