#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn maximize_xor(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort();

        // Prepare queries with their original indices
        let mut indexed_queries: Vec<(usize, i32, i32)> = queries
            .iter()
            .enumerate()
            .map(|(i, q)| (i, q[0], q[1]))
            .collect();

        // Sort queries by m (constraint) to process them in ascending order
        indexed_queries.sort_by_key(|&(_, _, m)| m);

        let mut result = vec![-1; queries.len()];
        let mut trie = Trie::new();
        let mut nums_idx = 0;

        for (query_idx, x, m) in indexed_queries {
            // Insert all elements ≤ m into the trie
            while nums_idx < nums.len() && nums[nums_idx] <= m {
                trie.insert(nums[nums_idx]);
                nums_idx += 1;
            }

            // If trie is not empty, find the maximum XOR
            if !trie.is_empty() {
                result[query_idx] = trie.max_xor(x);
            }
        }

        result
    }
}

// Trie implementation for this problem
struct TrieNode {
    children: [Option<Box<TrieNode>>; 2],
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: [None, None],
        }
    }
}

struct Trie {
    root: TrieNode,
    is_empty: bool,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
            is_empty: true,
        }
    }

    fn is_empty(&self) -> bool {
        self.is_empty
    }

    fn insert(&mut self, num: i32) {
        let mut node = &mut self.root;
        self.is_empty = false;

        // Process bits from most significant to least significant (31 to 0)
        for i in (0..31).rev() {
            let bit = ((num >> i) & 1) as usize;

            if node.children[bit].is_none() {
                node.children[bit] = Some(Box::new(TrieNode::new()));
            }

            node = node.children[bit].as_mut().unwrap();
        }
    }

    fn max_xor(&self, num: i32) -> i32 {
        let mut node = &self.root;
        let mut result = 0;

        // Process bits from most significant to least significant (31 to 0)
        for i in (0..31).rev() {
            let bit = ((num >> i) & 1) as usize;
            let opposite_bit = 1 - bit;

            // Try to go to the opposite bit to maximize XOR
            if node.children[opposite_bit].is_some() {
                result |= 1 << i;
                node = node.children[opposite_bit].as_ref().unwrap();
            } else {
                node = node.children[bit].as_ref().unwrap();
            }
        }

        result
    }
}
fn main() {}
