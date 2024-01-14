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
            cur = cur.child[idx].get_or_insert_with(||Box::new(Trie::new()));
        }
        cur.sealed = true;
    }

    fn search(&self, word: String) -> bool {
        let mut cur = self;
        for c in word.chars() {
            let idx = c as usize -  'a' as usize;
            match &cur.child[idx] {
                Some(node) => {
                    cur = node;
                },
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
            let idx = c as usize -  'a' as usize;
            match &cur.child[idx] {
                Some(node) => {
                    cur = node;
                },
                None => {
                    return false;
                }
            }
        }
        true
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

fn main() {}

