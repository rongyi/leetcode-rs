struct Solution;

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut val = x ^ y;
        let mut ret = 0;
        while val != 0 {
            ret += 1;
            val &= val - 1;
        }

        ret
    }
}

fn main() {}
