struct Solution;

impl Solution {
    pub fn decode_string(s: String) -> String {
        Self::decode(1, &s)
    }

    fn decode(repeat: usize, s: &str) -> String {
        // fast path
        if !s.contains('[') {
            return s.repeat(repeat);
        }
        let mut stack = Vec::new();
        let mut end_index = vec![0; s.len()];
        for (i, c) in s.chars().enumerate() {
            if c == '[' {
                stack.push(i);
            } else if c == ']' {
                // input is always valid
                let start = stack.pop().unwrap();
                end_index[start] = i;
            }
        }
        // 1. digit
        // 2. [
        // 3. normal string
        let mut normal = String::new();
        let mut acc = 0;
        let mut expand = String::new();
        let mut rest = String::new();
        for (i, c) in s.chars().enumerate() {
            if c.is_digit(10) {
                acc = acc * 10 + c.to_digit(10).unwrap();
            } else if c.is_alphabetic() {
                normal.push(c);
            } else if c == '[' {
                // println!("expand repeat: {} -> {}", acc, &s[i + 1..end_index[i]]);
                // println!("rest: {} -> {}", 1, &s[end_index[i] + 1..]);
                expand = Self::decode(acc as usize, &s[i + 1..end_index[i]]);
                rest = Self::decode(1, &s[end_index[i] + 1..]);
                break;
            }
        }
        (normal + &expand + &rest).repeat(repeat)
    }
}

fn main() {
    let a = Solution::decode_string("3[a2[c]]".to_string());
    println!("{}", a);
}
