#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let mut count = [0; 3];
        let mut i = 0;
        let mut ret = 0;
        for j in 0..s.len() {
            let idx = s[j] as usize - 'a' as usize;
            count[idx] += 1;
            while count.iter().all(|&x| x > 0) {
                let idx = s[i] as usize - 'a' as usize;
                count[idx] -= 1;
                i += 1;
            }
            ret += i as i32;
        }
        ret
    }
}
fn main() {}
