#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn freq_alphabets(s: String) -> String {
        let s: Vec<char> = s.chars().collect();
        let mut ret = String::new();

        let mut i = 0;
        let sz = s.len();
        while i < sz {
            // check single or two
            if i + 2 < sz && s[i + 2] == '#' {
                let d =
                    s[i].to_digit(10).unwrap() as u8 * 10 + s[i + 1].to_digit(10).unwrap() as u8;
                let c = ('a' as u8 + d - 1) as char;
                ret.push(c);

                i += 3;
            } else {
                // single char process
                let d = s[i].to_digit(10).unwrap() as u8;
                let c = ('a' as u8 + d - 1) as char;
                ret.push(c);

                i += 1;
            }
        }

        ret
    }
}

fn main() {}
