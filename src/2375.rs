struct Solution;

impl Solution {
    pub fn smallest_number(pattern: String) -> String {
        let mut out = String::new();
        let mut tmp = Vec::new();
        let p = pattern.as_bytes();
        let sz = p.len();

        for i in 0..=sz {
            tmp.push((i + 1).to_string());
            if i == sz || p[i] == b'I' {
                while let Some(num) = tmp.pop() {
                    out.push_str(&num);
                }
            }
        }

        out
    }
}

fn main() {}
