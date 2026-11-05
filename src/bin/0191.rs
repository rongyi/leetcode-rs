struct Solution;

impl Solution {
    pub fn hamming_weight(mut n: i32) -> i32 {
        let mut acc = 0;
        while n != 0 {
            acc += 1;
            n &= n - 1;
        }

        acc
    }
}

fn main() {}
