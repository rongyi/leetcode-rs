
struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn longest_valid_substring(word: String, forbidden: Vec<String>) -> i32 {
        let forbiden: HashSet<String> = forbidden.into_iter().collect();
        let s = word.as_bytes();
        let sz = s.len();
        let mut left = 0;
        let mut ret = 0;

        for j in 0..sz {
            let start = if j >= 9 { j - 9 } else { 0 };

            for i in (left.max(start)..=j).rev() {
                let slice = word[i..=j].to_string();
                if forbiden.contains(&slice) {
                    left = i + 1;
                    break;
                }
            }
            ret = ret.max(j - left + 1);
        }
        ret as _
    }
}

fn main() {}
