struct Solution;

impl Solution {
    pub fn percentage_letter(s: String, letter: char) -> i32 {
        let a: usize = s.chars().filter(|c| *c == letter).count();

        (a * 100 / s.len()) as _
    }
}

fn main() {}
