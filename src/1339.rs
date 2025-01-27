#![allow(dead_code)]
struct Solution;
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

impl Solution {
    // it's not about edge, it's about subtree update
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut all_tree: HashSet<i64> = HashSet::new();
        let total = Self::dfs(root.as_ref(), &mut all_tree);
        const MOD: i64 = 1e9 as i64 + 7;
        let mut max_prod = 0;
        for val in all_tree.into_iter() {
            max_prod = max_prod.max((total - val) * val);
        }
        (max_prod % MOD) as i32
    }

    fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, all_tree: &mut HashSet<i64>) -> i64 {
        if let Some(node) = root {
            let node = node.borrow();
            let l = Self::dfs(node.left.as_ref(), all_tree);
            let r = Self::dfs(node.right.as_ref(), all_tree);
            let val = l + r + node.val as i64;
            all_tree.insert(val);
            val
        } else {
            0
        }
    }
}

fn main() {}
