struct Solution;

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        let s = num.to_string();
        if s.len() <= 1 {
            return num;
        }
        let mut acc = 0;
        let mut num = num;
        while num != 0 {
            acc += num % 10;
            num /= 10;
        }

        Self::add_digits(acc)
    }
}

fn main() {}
