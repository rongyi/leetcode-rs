#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn shifting_letters(s: String, mut shifts: Vec<i32>) -> String {
        let mut s: Vec<char> = s.chars().collect();
        let sz = shifts.len();
        shifts[sz - 1] = shifts[sz - 1] % 26;

        for i in (0..sz - 1).rev() {
            shifts[i] = (shifts[i] % 26 + shifts[i + 1]) % 26;
        }
        for i in 0..s.len() {
            s[i] = ((((s[i] as u8 - 'a' as u8) as i32 + shifts[i]) % 26) as u8 + 'a' as u8) as char;
        }

        s.into_iter().collect()
    }
}

fn main() {}
