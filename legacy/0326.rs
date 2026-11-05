struct Solution;

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        let mut n = n;
        if n <= 0 {
            return false;
        }
        while n > 1 && n % 3 == 0 {
            n /= 3;
        }

        n == 1
    }
}

fn main() {}
