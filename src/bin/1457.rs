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
    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut acc = 0;

        Self::dfs(root.as_ref(), 0, &mut acc);

        acc
    }

    fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, mut path: i32, acc: &mut i32) {
        if let Some(node) = node {
            let node = node.borrow();
            path ^= 1 << node.val;

            if node.left.is_none() && node.right.is_none() {
                if path.count_ones() <= 1 {
                    *acc += 1;
                }
            }

            Self::dfs(node.left.as_ref(), path, acc);
            Self::dfs(node.right.as_ref(), path, acc);
        }
    }
}
fn main() {}
