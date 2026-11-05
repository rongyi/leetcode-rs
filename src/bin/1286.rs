#![allow(dead_code)]

struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};
struct CombinationIterator {
    cache: BinaryHeap<Reverse<String>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CombinationIterator {
    fn new(characters: String, combinationLength: i32) -> Self {
        let mut ret = CombinationIterator {
            cache: BinaryHeap::new(),
        };

        let raw: Vec<char> = characters.chars().collect();
        let mut cur = Vec::new();
        ret.dfs(&raw, raw.len(), combinationLength as usize, 0, &mut cur);

        ret
    }

    fn next(&mut self) -> String {
        match self.cache.pop() {
            Some(Reverse(e)) => e,
            _ => unreachable!(),
        }
    }

    fn has_next(&self) -> bool {
        !self.cache.is_empty()
    }

    fn dfs(&mut self, raw: &Vec<char>, n: usize, k: usize, idx: usize, cur: &mut Vec<char>) {
        if cur.len() == k {
            self.cache.push(Reverse(cur.iter().cloned().collect()));
            return;
        }
        if idx >= n {
            return;
        }
        // don't pick current
        self.dfs(raw, n, k, idx + 1, cur);
        // pick current
        cur.push(raw[idx]);
        self.dfs(raw, n, k, idx + 1, cur);
        cur.pop();
    }
}

fn main() {
    let mut bh = BinaryHeap::new();
    // bh.push("Hello");
    // bh.push("World");
    bh.push(1);
    bh.push(3);
    let cur = bh.pop().unwrap();
    println!("{cur}");
}
