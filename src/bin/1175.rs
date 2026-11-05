#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn num_prime_arrangements(n: i32) -> i32 {
        let m: u64 = 1e9 as u64 + 7;
        let prime_cnt: u64 = (2..=n).filter(|&x| Self::is_prime(x)).count() as u64;
        let per1: u64 = (1..=prime_cnt).fold(1u64, |acc, cur| (acc * cur) % m);
        let per2 = (1..=(n as u64 - prime_cnt)).fold(1, |acc, cur| (acc * cur) % m);

        ((per1 * per2) % m) as i32
    }

    fn is_prime(n: i32) -> bool {
        for i in 2.. {
            if i * i > n {
                break;
            }
            if n % i == 0 {
                return false;
            }
        }

        true
    }
}

fn main() {}
