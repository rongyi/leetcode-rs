struct Solution;

use std::collections::HashMap;
#[derive(Debug)]
struct Trie {
    count: usize,
    entry: HashMap<char, Box<Trie>>,
}

impl Trie {
    fn new() -> Self {
        Trie {
            count: 0,
            entry: HashMap::new(),
        }
    }
    fn insert(&mut self, s: &str) {
        let mut cur = self;
        for c in s.chars() {
            if !cur.entry.contains_key(&c) {
                cur.entry.insert(c, Box::new(Trie::new()));
            }
            cur = cur.entry.get_mut(&c).unwrap();
            cur.count += 1;
        }
    }

    fn count(&self, s: &str, pos: usize) -> usize {
        if pos >= s.len() {
            return self.count;
        }
        let c = s.as_bytes()[pos] as char;
        if let Some(next_layer) = self.entry.get(&c) {
            self.count + next_layer.count(s, pos + 1)
        } else {
            0
        }
    }
}

impl Solution {
    pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
        let mut t = Trie::new();
        for w in words.iter() {
            t.insert(w);
        }
        let mut ret = vec![];
        for w in words.iter() {
            ret.push(t.count(w, 0) as i32);
        }

        ret
    }
}

fn main() {}
