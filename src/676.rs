struct Solution;

use std::collections::HashSet;

struct MagicDictionary {
    lens: HashSet<usize>,
    words: Vec<String>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MagicDictionary {
    fn new() -> Self {
        Self {
            lens: HashSet::new(),
            words: Vec::new(),
        }
    }

    fn build_dict(&mut self, dictionary: Vec<String>) {
        self.words = dictionary;
        for w in self.words.iter() {
            self.lens.insert(w.len());
        }
    }

    fn search(&self, search_word: String) -> bool {
        if !self.lens.contains(&search_word.len()) {
            return false;
        }
        for w in self.words.iter() {
            if w.len() != search_word.len() {
                continue;
            }

            if self.match_word(&w.as_bytes(), &search_word.as_bytes()) {
                return true;
            }
        }

        false
    }

    fn match_word(&self, dict: &[u8], search_word: &[u8]) -> bool {
        if dict.len() != search_word.len() {
            return false;
        }
        let mut prev_diff = -1;
        for i in 0..dict.len() {
            if dict[i] != search_word[i] {
                if prev_diff != -1 {
                    return false;
                }
                prev_diff = 1;
            }
        }
        if prev_diff == -1 {
            return false;
        }
        true
    }
}

fn main() {}
