struct Solution;

impl Solution {
    pub fn monotone_increasing_digits(n: i32) -> i32 {
        let mut digits: Vec<i32> = n
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();

        let sz = digits.len();
        let mut j = sz;

        for i in (1..sz).rev() {
            if digits[i] < digits[i - 1] {
                j = i;
                digits[i - 1] = digits[i - 1] - 1;
            }
        }

        for i in j..sz {
            digits[i] = 9;
        }

        digits.into_iter().fold(0, |acc, cur| acc * 10 + cur)
    }
}

fn main() {}
