struct Solution;
use std::collections::HashSet;

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
}

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut board = board;
        let mut trie = Trie::new();
        for word in words.into_iter() {
            trie.insert(word);
        }
        let mut result: HashSet<String> = HashSet::new();
        let m = board.len() as i32;
        let n = board[0].len() as i32;

        for i in 0..m {
            for j in 0..n {
                let mut cur = String::new();
                Self::dfs(&trie, &mut board, m, n, i, j, &mut cur, &mut result);
            }
        }

        result.into_iter().collect()
    }
    fn dfs(
        mut trie: &Trie,
        board: &mut Vec<Vec<char>>,
        m: i32,
        n: i32,
        x: i32,
        y: i32,
        cur: &mut String,
        result: &mut HashSet<String>,
    ) {
        if x < 0 || x >= m || y < 0 || y >= n || board[x as usize][y as usize] == ' ' {
            return;
        }
        let x = x as usize;
        let y = y as usize;
        let c = board[x][y];
        let idx = board[x][y] as usize - 'a' as usize;
        if trie.child[idx].is_none() {
            return;
        }
        // we try to use this postion
        board[x][y] = ' ';
        cur.push(c);
        trie = trie.child[idx].as_ref().unwrap();
        if trie.sealed {
            result.insert(cur.clone());
        }

        Self::dfs(trie, board, m, n, x as i32 + 1, y as i32, cur, result);
        Self::dfs(trie, board, m, n, x as i32 - 1, y as i32, cur, result);
        Self::dfs(trie, board, m, n, x as i32, y as i32 + 1, cur, result);
        Self::dfs(trie, board, m, n, x as i32, y as i32 - 1, cur, result);

        board[x][y] = c;
        cur.pop();
    }
}

fn main() {
    unimplemented!();
}
