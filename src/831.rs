#![allow(dead_code)]
struct Solution;

impl Solution {
    // shitty problem
    pub fn mask_pii(s: String) -> String {
        if s.contains('@') {
            let s = s.to_lowercase();
            let chunks: Vec<String> = s.split('@').map(|s| s.to_string()).collect();
            let mut name = chunks[0].clone();
            name.replace_range(1..name.len() - 1, "*****");

            format!("{}@{}", name, chunks[1])
        } else {
            let num = s.chars().filter(|c| c.is_ascii_digit()).collect::<String>();
            if num.len() == 10 {
                format!("***-***-{}", &num[6..])
            } else if num.len() == 11 {
                format!("+*-***-***-{}", &num[7..])
            } else if num.len() == 12 {
                format!("+**-***-***-{}", &num[8..])
            } else {
                format!("+***-***-***-{}", &num[9..])
            }
        }
    }
}

fn main() {}
