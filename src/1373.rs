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
use std::rc::Rc;
impl Solution {
    pub fn max_sum_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_sum = 0;
        if let Some(node) = root.as_ref() {
            Self::dfs(node, &mut max_sum);
        }

        max_sum
    }
    // (sum, min, max)
    // sum: all subtree sum including current node
    // min: all subtree minimum number inclusing itself
    // max: all subtree maxsum including itself
    fn dfs(node: &Rc<RefCell<TreeNode>>, max_sum: &mut i32) -> (i32, i32, i32) {
        let node = node.borrow();
        let l;
        let r;
        if let Some(lnode) = node.left.as_ref() {
            l = Self::dfs(lnode, max_sum);
        } else {
            l = (
                0,
                node.val,
                node.val - 1, /*just to make the following check pass */
            )
        }
        if let Some(rnode) = node.right.as_ref() {
            r = Self::dfs(rnode, max_sum);
        } else {
            r = (
                0,
                node.val + 1, /*just to make the folling check pass, a little hack */
                node.val,
            )
        }
        if l.2 < node.val && node.val < r.1 {
            *max_sum = (*max_sum).max(node.val + l.0 + r.0);
            return (node.val + l.0 + r.0, l.1, r.2);
        }

        (0, i32::MIN, i32::MAX)
    }
}

fn main() {}
