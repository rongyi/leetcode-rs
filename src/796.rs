#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let goal: Vec<char> = goal.chars().collect();
        if s.len() != goal.len() {
            return false;
        }
        let sz = s.len();

        for i in 0..sz {
            if goal[i] == s[0] {
                if Self::same_with_shift(&s, &goal, i) {
                    return true;
                }
            }
        }

        false
    }

    fn same_with_shift(s: &Vec<char>, t: &Vec<char>, shift: usize) -> bool {
        let sz = s.len();
        for (i, j) in (shift..sz).enumerate() {
            if s[i] != t[j] {
                return false;
            }
        }
        for (i, j) in (0..shift).enumerate() {
            if s[sz - shift + i] != t[j] {
                return false;
            }
        }

        true
    }
}

fn main() {
    let input = vec![1, 1, 1];
}
