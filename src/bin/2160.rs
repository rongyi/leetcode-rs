struct Solution;

impl Solution {
    pub fn minimum_sum(mut num: i32) -> i32 {
        let mut digits: Vec<i32> = vec![];
        while num != 0 {
            digits.push(num % 10);
            num /= 10;
        }


        digits.sort_unstable();
        digits[0] * 10 + digits[1] * 10 + digits[2] + digits[3]
    }
}

fn main() {}
