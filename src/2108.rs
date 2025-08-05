struct Solution;

impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        for st in words.iter() {
            let s = st.as_bytes();
            let sz = s.len();

            let mut valid = true;
            let mut i = 0;
            let mut j = sz - 1;
            while i < j {
                if s[i] != s[j] {
                    valid = false;
                    break;
                }
                i += 1;
                j -= 1;
            }
            if valid {
                return st.to_string();
            }
        }

        "".to_string()
    }
}

fn main() {}
