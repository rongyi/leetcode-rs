struct Solution;

impl Solution {
    pub fn check_record(s: String) -> bool {
        let absent = s.chars().filter(|&c| c == 'A').count();
        if absent >= 2 {
            return false;
        }
        let s: Vec<char> = s.chars().collect();
        let sz = s.len();
        let mut i = 0;
        while i < sz {
            if s[i] == 'L' {
                let mut j = i;
                while j < sz && s[j] == s[i] {
                    j += 1;
                }
                if j - i >= 3 {
                    return false;
                }

                i = j;
            } else {
                i += 1;
            }
        }
        true
    }
}

fn main() {}
