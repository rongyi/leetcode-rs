#![allow(dead_code)]

struct Solution;
struct TreeAncestor {
    // For each node i, ancestors[i][j] is the (2^j)th ancestor of node i
    ancestors: Vec<Vec<i32>>,
}

impl TreeAncestor {
    fn new(n: i32, parent: Vec<i32>) -> Self {
        let n = n as usize;
        let max_pow = (n as f64).log2().ceil() as usize;

        // Initialize the ancestors matrix with -1 (no ancestor)
        // ancestors[i][j] -> i node's (2^j)th ancestor
        let mut ancestors = vec![vec![-1; max_pow + 1]; n];

        // Set direct parents (2^0 ancestors)
        for i in 0..n {
            ancestors[i][0] = parent[i];
        }

        // Fill the ancestors matrix using dynamic programming
        for j in 1..=max_pow {
            for i in 0..n {
                if ancestors[i][j - 1] != -1 {
                    // To find the (2^j)th ancestor of node i, find the (2^(j-1))th ancestor
                    // and then find its (2^(j-1))th ancestor
                    let mid_ancestor = ancestors[i][j - 1] as usize;
                    ancestors[i][j] = ancestors[mid_ancestor][j - 1];
                }
            }
        }

        TreeAncestor { ancestors }
    }

    fn get_kth_ancestor(&self, node: i32, k: i32) -> i32 {
        let mut node = node as usize;
        let k = k;

        // If k is out of bounds, return -1
        if k >= (1 << self.ancestors[0].len()) {
            return -1;
        }

        // Process each bit of k from least significant to most significant
        for j in 0..32 {
            if k & (1 << j) != 0 {
                // If this bit is set, jump to the (2^j)th ancestor
                if node >= self.ancestors.len() || self.ancestors[node][j] == -1 {
                    return -1;
                }

                node = self.ancestors[node][j] as usize;
            }
        }

        node as i32
    }
}
fn main() {}
