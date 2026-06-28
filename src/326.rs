struct Solution;

impl Solution {

    pub fn is_power_of_three(mut n: i32) -> bool {
        while n > 1 && n % 3 == 0 {
            n /= 3;
        }
        n == 1
    }
}

fn main() {}
