struct Solution;

impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let mut round = 0;
        let mut left = left;
        let mut right = right;

        // when left != right
        // we must have at least one even and one odd number
        // that means the last bit AND is zero
        // remember all those iterations
        // and padding 0 to left is the number we got
        while left != right {
            left >>= 1;
            right >>= 1;
            round += 1;
        }

        left << round
    }
}

fn main() {}
