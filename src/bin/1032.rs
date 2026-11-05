use std::collections::HashMap;

#[derive(Debug)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_word: bool,
}

#[derive(Debug)]
struct Trie {
    root: TrieNode,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_word: false,
        }
    }
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }
    fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for c in word.chars().rev() {
            node = node.children.entry(c).or_insert(TrieNode::new());
        }
        node.is_word = true;
    }
    fn search(&self, stream: &[char]) -> bool {
        let mut node = &self.root;
        for &c in stream.iter().rev() {
            if let Some(next) = node.children.get(&c) {
                if next.is_word {
                    return true;
                }
                node = next;
            } else {
                return false;
            }
        }
        false
    }
}

struct StreamChecker {
    trie: Trie,
    stream: Vec<char>,
    max_length: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StreamChecker {
    fn new(words: Vec<String>) -> Self {
        let mut trie = Trie::new();
        let mut max_length = 0;
        for w in words.into_iter() {
            max_length = max_length.max(w.len());
            trie.insert(w);
        }
        Self {
            trie,
            stream: Vec::new(),
            max_length,
        }
    }

    fn query(&mut self, letter: char) -> bool {
        self.stream.push(letter);
        if self.stream.len() > self.max_length {
            self.stream.remove(0);
        }
        self.trie.search(&self.stream)
    }
}

fn main() {}
