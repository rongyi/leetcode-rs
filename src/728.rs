#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        fn check(mut i: i32) -> bool {
            let num = i;
            while i != 0 {
                let cur = i % 10;

                if cur == 0 || num % cur != 0 {
                    return false;
                }

                i /= 10;
            }

            true
        }

        (left..=right).into_iter().filter(|&i| check(i)).collect()
    }
}

fn main() {}
