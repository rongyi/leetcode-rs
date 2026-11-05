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
    pub fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(root.as_ref(), 1, 1)
    }
    fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, parent: i32, grand_parent: i32) -> i32 {
        if let Some(node) = node {
            let node = node.borrow();
            let val = if grand_parent % 2 == 0 { node.val } else { 0 };

            val + Self::dfs(node.left.as_ref(), node.val, parent)
                + Self::dfs(node.right.as_ref(), node.val, parent)
        } else {
            0
        }
    }
}

fn main() {}
