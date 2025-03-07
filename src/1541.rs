#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let mut left = 0;
        // stratege: only allow ( to left, not right,
        let s: Vec<char> = s.chars().collect();
        let sz = s.len();
        let mut i = 0;
        let mut total_change = 0;
        while i < sz {
            if s[i] == '(' {
                left += 1;
            } else {
                if i + 1 < sz && s[i + 1] == ')' {
                    if left > 0 {
                        left -= 1;
                    } else {
                        total_change += 1;
                    }

                    i += 1;
                } else {
                    // a single )
                    if left > 0 {
                        left -= 1;
                        total_change += 1;
                    } else {
                        // need to add ( and )
                        total_change += 2;
                    }
                }
            }

            i += 1;
        }

        total_change + left * 2
    }
}
fn main() {
    let val = Solution::min_insertions("(()))".to_string());
    println!("{val}");
}
