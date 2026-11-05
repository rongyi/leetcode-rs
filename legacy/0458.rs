struct Solution;

impl Solution {
    // https://leetcode.com/problems/poor-pigs/discuss/94266/Another-explanation-and-solution
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        let mut pigs = 0;
        let base = minutes_to_test / minutes_to_die + 1;
        while base.pow(pigs) < buckets {
            pigs += 1;
        }
        pigs as i32
    }
}

fn main() {}
