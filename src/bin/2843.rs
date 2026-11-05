struct Solution;

impl Solution {
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        let mut ret = 0;

        for mut i in low..=high {
            let mut digits: Vec<i32> = vec![];
            while i > 0 {
                digits.push(i % 10);
                i /= 10;
            }
            if digits.len() % 2 == 0
                && digits[0..digits.len() / 2].iter().copied().sum::<i32>()
                    == digits[digits.len() / 2..].iter().copied().sum::<i32>()
            {
                ret += 1;
            }
        }

        ret
    }
}

fn main() {}
