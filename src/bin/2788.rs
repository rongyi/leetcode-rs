struct Solution;

impl Solution {
    pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
        let mut ret: Vec<String> = Vec::new();
        for w in words.iter() {
            for s in w.split(separator) {
                if !s.is_empty() {
                    ret.push(s.to_string());
                }
            }
        }

        ret
    }
}

fn main() {}
