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
            child: Default::default(),
            sealed: false,
        }
    }

    fn insert(&mut self, word: String) {
        let mut cur = self;
        for c in word.chars() {
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

struct WordDictionary {
    trie: Box<Trie>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    fn new() -> Self {
        WordDictionary {
            trie: Box::new(Trie::new()),
        }
    }

    fn add_word(&mut self, word: String) {
        self.trie.insert(word);
    }

    fn search(&self, word: String) -> bool {
        Self::search_after(self.trie.as_ref(), &word)
    }

    fn search_after(root: &Trie, word: &str) -> bool {
        let mut cur = root;
        for (i, c) in word.chars().enumerate() {
            if c == '.' {
                let rest = word[i + 1..].to_string();
                for (_k, v) in cur.child.iter() {
                    if Self::search_after(v, &rest) {
                        return true;
                    }
                }
                return false;
            } else {
                match cur.child.get(&c) {
                    Some(node) => cur = &node,
                    None => return false,
                }
            }
        }

        cur.sealed
    }
}

fn main() {}
