struct Solution;

/// LeetCode 653: Two Sum IV - Input is a BST
///
/// DFS + HashSet. For each node, check if (k - node.val) was already
/// seen. Check BEFORE inserting self → a node can never pair with itself.
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut cache = HashSet::new();
        Self::find_two_sum(root.as_ref(), k, &mut cache)
    }

    fn find_two_sum(
        node: Option<&Rc<RefCell<TreeNode>>>,
        k: i32,
        cache: &mut HashSet<i32>,
    ) -> bool {
        let Some(node) = node else {
            return false;
        };
        let node = node.borrow();

        // Pair found? (checked before inserting self → no self-pairing)
        if cache.contains(&(k - node.val)) {
            return true;
        }
        cache.insert(node.val);

        Self::find_two_sum(node.left.as_ref(), k, cache)
            || Self::find_two_sum(node.right.as_ref(), k, cache)
    }
}

fn main() {}
