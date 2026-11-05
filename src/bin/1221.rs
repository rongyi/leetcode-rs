#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut vals: Vec<i32> = s.chars().map(|c| if c == 'L' { -1 } else { 1 }).collect();
        let sz = vals.len();
        let mut prefix = vec![0; sz + 1];
        for (i, &val) in vals.iter().enumerate() {
            prefix[i + 1] = prefix[i] + val;
        }
        let mut total = 0;
        for i in 1..=sz {
            if prefix[i] == 0 {
                total += 1;
            }
        }

        total
    }
}

fn main() {}
