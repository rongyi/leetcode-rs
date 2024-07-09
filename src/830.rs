#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        let s: Vec<char> = s.chars().collect();
        let mut ret = Vec::new();
        
        let mut i = 0;
        let sz = s.len();
        while i < sz {
            let mut j = i;
            while j < sz && s[j] == s[i] {
                j += 1;
            }
            let cur_range = j - i;
            if cur_range >= 3 {
               ret.push(vec![i as i32, j as i32 - 1]);
            }
            
            i = j;
        }
        
        ret
    }
}

fn main() {}
