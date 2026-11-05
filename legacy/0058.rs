struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let s = s.trim();
        match s.rfind(' ') {
            Some(i) => (s.len() - i - 1) as i32,
            None => s.len() as i32,
        }
    }
}
