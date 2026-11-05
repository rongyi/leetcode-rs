#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn unique_letter_string(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let sz = s.len();
        let mut index: Vec<[i32;2]> = vec![[-1,-1]; 26];
        
        let m = 1e9 as i32 + 7;
        let mut ret = 0;
        for i in 0..sz {
            let idx = (s[i] as u8 - 'A' as u8) as usize;
            let val = (i as i32 - index[idx][1]) * (index[idx][1] -  index[idx][0]) % m;
            ret = (ret + val) % m;
            index[idx][0] = index[idx][1];
            index[idx][1] = i as i32;
        }
        for i in 0..26 {
            ret = (ret + (sz as i32 - index[i][1]) * (index[i][1] - index[i][0]) % m) % m;
        }
        
        ret
    }
}

fn main() {}