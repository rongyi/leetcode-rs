struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut words: Vec<&str> = s.split_whitespace().collect();
        words.reverse();

        words.join(" ")
    }
}

fn main() {}
