struct Solution;


impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut words: Vec<_> = s.split_whitespace().rev().collect();
        words.join(" ")
    }
}


fn main() {}
