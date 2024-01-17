struct Trie {
    child: [Option<Box<Trie>>; 26],
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
            let idx = c as usize - 'a' as usize;
            cur = cur.child[idx].get_or_insert_with(|| Box::new(Trie::new()));
        }
        cur.sealed = true;
    }

    fn search(&self, word: String) -> bool {
        let mut cur = self;
        for c in word.chars() {
            let idx = c as usize - 'a' as usize;
            match &cur.child[idx] {
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
            let idx = c as usize - 'a' as usize;
            match &cur.child[idx] {
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
        Self::search_after(&Some(self.trie.as_ref()), word)
    }

    fn search_after(cur: &Option<&Trie>, word: String) -> bool {
        match cur {
            None => return false,
            Some(mut node) => {
                for (i, c) in word.chars().enumerate() {
                    if c == '.' {
                        let after = word[i + 1..].to_owned();
                        for j in 0..26 {
                            let n = node.child[j].as_deref();
                            if Self::search_after(&n, after.clone()) {
                                return true;
                            }
                        }
                        return false;
                    } else {
                        let idx = c as usize - 'a' as usize;
                        if node.child[idx].is_none() {
                            return false;
                        }
                        node = node.child[idx].as_ref().unwrap();
                    }
                }
                node.sealed
            }
        }
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */

fn main() {
    unimplemented!();
}
