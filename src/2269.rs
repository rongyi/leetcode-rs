struct Solution;

impl Solution {
    pub fn divisor_substrings(num: i32, k: i32) -> i32 {
        let s = num.to_string();
        let s = s.as_bytes();
        let k = k as usize;
        let mut acc = 0;
        for i in 0..=s.len() - k {
            let cur = s[i..i + k].to_vec();
            let cur: i32 = String::from_utf8(cur).unwrap().parse().unwrap();
            if cur == 0 {
                continue;
            }
            if num % cur == 0 {
                acc += 1;
            }
        }
        acc
    }
}

fn main() {}
