struct Solution;

use std::collections::BTreeSet;
impl Solution {
    pub fn find_even_numbers(mut digits: Vec<i32>) -> Vec<i32> {
        let mut result = BTreeSet::new();
        let n = digits.len();

        for i in 0..n {
            if digits[i] == 0 {
                continue;
            }
            for j in 0..n {
                if j == i {
                    continue;
                }
                for k in 0..n {
                    if k == i || k == j {
                        continue;
                    }
                    let num = digits[i] * 100 + digits[j] * 10 + digits[k];
                    if num % 2 == 0 {
                        result.insert(num);
                    }
                }
            }
        }

        result.into_iter().collect()
    }
}

fn main() {}
