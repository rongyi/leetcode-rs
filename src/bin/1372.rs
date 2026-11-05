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
    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(
            node: Option<&Rc<RefCell<TreeNode>>>,
            cur_len: i32,
            max_len: &mut i32,
            from_left: bool,
        ) {
            if let Some(node) = node {
                let node = node.borrow();

                *max_len = (*max_len).max(cur_len);
                if from_left {
                    dfs(node.right.as_ref(), cur_len + 1, max_len, false);
                    dfs(node.left.as_ref(), 1, max_len, true);
                } else {
                    dfs(node.left.as_ref(), cur_len + 1, max_len, true);
                    dfs(node.right.as_ref(), 1, max_len, false);
                }
            }
        }

        let mut max_len = 0;
        dfs(root.as_ref(), 0, &mut max_len, true);
        dfs(root.as_ref(), 0, &mut max_len, false);

        max_len
    }
}

fn main() {}
