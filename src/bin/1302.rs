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
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_depth = 0;
        let mut sum = 0;

        fn dfs(
            root: Option<&Rc<RefCell<TreeNode>>>,
            max_depth: &mut i32,
            sum: &mut i32,
            cur_depth: i32,
        ) {
            if let Some(node) = root {
                let node = node.borrow();
                if cur_depth > *max_depth {
                    *max_depth = cur_depth;
                    *sum = node.val;
                } else if cur_depth == *max_depth {
                    *sum += node.val;
                }
                dfs(node.left.as_ref(), max_depth, sum, cur_depth + 1);
                dfs(node.right.as_ref(), max_depth, sum, cur_depth + 1);
            }
        }

        dfs(root.as_ref(), &mut max_depth, &mut sum, 0);

        sum
    }
}

fn main() {}
