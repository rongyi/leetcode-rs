struct Solution;

impl Solution {
    pub fn min_max_difference(num: i32) -> i32 {
        let s: Vec<char> = num.to_string().chars().collect();
        // to get max, we replace first non '9' to '9' from right to left
        let mut replace_char: Option<char> = None;
        for &c in s.iter() {
            if c != '9' {
                replace_char = Some(c);
                break;
            }
        }
        let max_val = match replace_char {
            None => s
                .iter()
                .fold(String::new(), |mut s, cur| {
                    s.push(*cur);
                    s
                })
                .parse::<i32>()
                .unwrap(),
            Some(ch) => s
                .iter()
                .fold(String::new(), |mut s, &cur| {
                    if cur == ch {
                        s.push('9');
                    } else {
                        s.push(cur);
                    }
                    s
                })
                .parse::<i32>()
                .unwrap(),
        };
        // min val is to make first char to '0'
        let ch = s[0];
        let mut j = 0;
        for i in 0..s.len() {
            if s[i] != ch {
                break;
            }
            j = i;
        }
        let min_str = s[j..].iter().map(|&c| if c == ch { '0' } else { c }).fold(
            String::new(),
            |mut acc, cur| {
                acc.push(cur);
                acc
            },
        );
        let min_val = min_str.parse::<i32>().unwrap();

        max_val - min_val
    }
}

fn main() {
    Solution::min_max_difference(11891);
}
