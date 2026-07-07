struct Solution;

impl Solution {
    pub fn decode_string(s: String) -> String {
        let (flated, _) = Self::flat(s.as_bytes(), 0, 1);
        flated
    }

    fn flat(s: &[u8], start: usize, parent_repeat: usize) -> (String, usize) {
        if start >= s.len() {
            return ("".to_string(), s.len());
        }
        let mut cur: String = String::new();
        let mut repeat = 0;
        let mut i = start;
        let sz = s.len();

        while i < sz {
            if s[i].is_ascii_digit() {
                repeat = repeat * 10 + (s[i] - b'0') as usize;
                i += 1;
            } else if s[i].is_ascii_alphabetic() {
                cur.push(s[i] as char);
                i += 1;
            } else if s[i] == b']' {
                return (cur.repeat(parent_repeat), i + 1);
            } else if s[i] == b'[' {
                let (sub, next_start) = Self::flat(s, i + 1, repeat);
                // reset
                i = next_start;
                repeat = 0;
                cur = format!("{}{}", cur, sub);
            }
        }
        (cur.repeat(parent_repeat), i)
    }
}

fn main() {
    let input = "3[a]2[bc]".to_string();
    let val = Solution::decode_string(input);
    println!("{}", val);
}
