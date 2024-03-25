struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }
        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();
        let mut s1freq: HashMap<char, usize> = HashMap::new();
        let mut win_freq: HashMap<char, usize> = HashMap::new();

        for &c in s1.iter() {
            *s1freq.entry(c).or_insert(0) += 1;
        }
        let s1sz = s1.len();
        for i in 0..s1sz {
            *win_freq.entry(s2[i]).or_insert(0) += 1;
        }
        if win_freq == s1freq {
            return true;
        }
        for i in s1sz..s2.len() {
            *win_freq.entry(s2[i]).or_insert(0) += 1;
            win_freq.entry(s2[i - s1sz]).and_modify(|c| *c -= 1);
            if win_freq.contains_key(&s2[i - s1sz]) && win_freq[&s2[i - s1sz]] == 0 {
                win_freq.remove(&s2[i - s1sz]);
            }

            if win_freq == s1freq {
                return true;
            }
        }

        false
    }
}

fn main() {}
