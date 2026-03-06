struct Solution;

impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        let mut ret = vec![];

        for (i, w) in words.iter().enumerate() {
            if w.contains(x) {
                ret.push(i as i32);
            }
        }

        ret
    }
}

fn main() {}
