struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.trim_end().split(' ').last().map(|w| w.len()).unwrap_or(0) as i32
    }
}

fn main() {}
