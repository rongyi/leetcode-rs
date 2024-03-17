struct Solution;

impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        let mut sum = 0;
        let end = num / 2;

        for i in 1..=end {
            if num % i == 0 {
                sum += i;
                sum += num % i;
            }
        }

        sum == num
    }
}

fn main() {}
