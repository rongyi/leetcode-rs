struct Solution;

impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let s = s.to_uppercase();
        let s: Vec<char> = s.chars().filter(|&c| c != '-').collect();
        let k = k as usize;
        let mut out = vec![];
        if s.len() % k == 0 {
            for i in (0..s.len()).step_by(k) {
                out.push(&s[i..i + k]);
            }
        } else {
            let first_len = s.len() % k;
            out.push(&s[0..first_len]);
            for i in (first_len..s.len()).step_by(k) {
                out.push(&s[i..i + k]);
            }
        }
        out.into_iter()
            .map(|s| s.iter().to_owned().collect::<String>())
            .collect::<Vec<_>>()
            .join("-")
    }
}

fn main() {}
