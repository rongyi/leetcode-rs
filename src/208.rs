struct Solution;

use std::collections::HashMap;
struct Trie {
    child: HashMap<char, Box<Trie>>,
    sealed: bool,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Trie {
            child: HashMap::new(),
            sealed: false,
        }
    }

    fn insert(&mut self, word: String) {
        let mut cur = self;
        for c in word.chars() {
            // let idx = c as usize - 'a' as usize;
            // cur = cur.child[idx].get_or_insert_with(|| Box::new(Trie::new()));
            cur = cur.child.entry(c).or_insert(Box::new(Trie::new()));
        }
        cur.sealed = true;
    }

    fn search(&self, word: String) -> bool {
        let mut cur = self;
        for c in word.chars() {
            match cur.child.get(&c) {
                Some(node) => {
                    cur = node;
                }
                None => {
                    return false;
                }
            }
        }
        cur.sealed
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut cur = self;
        for c in prefix.chars() {
            match cur.child.get(&c) {
                Some(node) => {
                    cur = node;
                }
                None => {
                    return false;
                }
            }
        }
        true
    }
}

fn main() {}
