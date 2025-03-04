#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_integer(num: String, mut k: i32) -> String {
        let mut chars: Vec<char> = num.chars().collect();
        let n = chars.len();
        let mut result = String::new();

        let mut i = 0;
        while i < n && k > 0 {
            // Find the smallest digit within k+i positions
            let search_end = std::cmp::min(n, (k as usize) + i + 1);
            let mut min_pos = i;
            let mut min_digit = chars[i];

            for j in i..search_end {
                if chars[j] < min_digit {
                    min_digit = chars[j];
                    min_pos = j;
                }
            }

            // Move the smallest digit to position i
            if min_pos > i {
                let cost = min_pos - i;
                k -= cost as i32;

                let digit = chars.remove(min_pos);
                chars.insert(i, digit);
            }

            i += 1;
        }

        chars.into_iter().collect()
    }
}

fn main() {}
