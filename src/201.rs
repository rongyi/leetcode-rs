struct Solution;

fn main() {}

impl Solution {
    pub fn range_bitwise_and(mut left: i32, mut right: i32) -> i32 {
        let mut shift = 0;
        // common prefix
        while left != right {
            left >>= 1;
            right >>= 1;
            shift += 1;
        }

        // restore the common prefix
        left << shift
    }
}
