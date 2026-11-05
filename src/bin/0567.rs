struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }
        // let s1: Vec<char> = s1.chars().collect();
        // let s2: Vec<char> = s2.chars().collect();
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s1freq: HashMap<u8, usize> = s1.iter().copied().fold(HashMap::new(), |mut acc, cur| {
            *acc.entry(cur).or_default() += 1;
            acc
        });
        let mut win_freq: HashMap<u8, usize> = HashMap::new();

        let k = s1.len();
        for i in 0..k {
            *win_freq.entry(s2[i]).or_insert(0) += 1;
        }
        if win_freq == s1freq {
            return true;
        }
        for i in k..s2.len() {
            *win_freq.entry(s2[i]).or_insert(0) += 1;
            // win_freq.entry(s2[i - k]).and_modify(|c| *c -= 1);
            if let Some(e) = win_freq.get_mut(&s2[i - k]) {
                *e -= 1;
                if *e == 0 {
                    win_freq.remove(&s2[i - k]);
                }
            }

            if win_freq == s1freq {
                return true;
            }
        }

        false
    }
}

fn main() {}
