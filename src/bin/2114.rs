struct Solution;

impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        let mut max_words = 0;

        for s in sentences.iter() {
            let cur_words: usize = s.split(' ').count();
            max_words = max_words.max(cur_words);
        }

        max_words as _
    }
}

fn main() {}
