struct Solution;

impl Solution {
    pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
        let mut diff = start ^ goal;
        let mut ret = 0;
        while diff != 0 {
            ret += 1;
            diff &= diff - 1;
        }
        ret
    }
}

fn main() {}
