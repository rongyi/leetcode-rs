#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        let mut cnt = 0;
        for num in left..=right {
            if Self::is_prime(num.count_ones()) {
                cnt += 1;
            }
        }

        cnt
    }

    fn is_prime(num: u32) -> bool {
        for i in 2.. {
            if i * i > num {
                break;
            }
            if num % i == 0 {
                return false;
            }
        }
        num > 1
    }
}

fn main() {}
