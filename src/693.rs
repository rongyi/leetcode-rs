#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn has_alternating_bits(mut n: i32) -> bool {
        let mut prev_is_one = n % 2 == 1;
        n /= 2;
        while n != 0 {
            let cur_is_one = n % 2 == 1;
            if !(cur_is_one ^ prev_is_one) {
                return false;
            }
            prev_is_one = cur_is_one;
            n /= 2;
        }
        true
    }
}

fn main() {
    let val = Solution::has_alternating_bits(5);
    println!("{}", val);
}
