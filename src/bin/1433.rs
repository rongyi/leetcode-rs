#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn check_if_can_break(s1: String, s2: String) -> bool {
        let mut s1: Vec<char> = s1.chars().collect();
        let mut s2: Vec<char> = s2.chars().collect();
        s1.sort_unstable();
        s2.sort_unstable();
        let sz = s1.len();

        if s1[0] >= s2[sz - 1] || s1[sz - 1] <= s2[0] {
            return true;
        }
        // intersection, need to sort
        let mut relation = 0;
        for i in 0..sz {
            if relation == 0 {
                if s1[i] < s2[i] {
                    relation = -1;
                } else if s1[i] > s2[i] {
                    relation = 1;
                }
            } else {
                if relation < 0 && s1[i] > s2[i] {
                    return false;
                } else if relation > 0 && s1[i] < s2[i] {
                    return false;
                }
            }
        }

        true
    }
}

fn main() {}
