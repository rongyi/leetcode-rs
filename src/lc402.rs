struct Solution;

impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let mut s: Vec<char> = Vec::new();
        let mut k = k;
        for c in num.chars() {
            while !s.is_empty() && *s.last().unwrap() > c && k > 0 {
                s.pop();
                k -= 1;
            }
            if !s.is_empty() || c != '0' {
                s.push(c);
            }
        }

        while !s.is_empty() && k > 0 {
            s.pop();
            k -= 1;
        }

        if s.is_empty() {
            return "0".to_string();
        }

        s.iter().collect()
    }
}

fn main() {
    println!("{}", 10i32.pow(2));
}
