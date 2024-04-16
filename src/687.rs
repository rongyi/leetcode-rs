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
    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut ret = 0;

        Self::recur(root.as_ref(), root.clone().unwrap().borrow().val, &mut ret);

        ret - 1
    }
    fn recur(root: Option<&Rc<RefCell<TreeNode>>>, target: i32, max_cnt: &mut i32) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            if node.val == target {
                let left = Self::recur(node.left.as_ref(), target, max_cnt);
                let right = Self::recur(node.right.as_ref(), target, max_cnt);
                *max_cnt = (*max_cnt).max(1 + left + right);
                left.max(right) + 1
            } else {
                Self::recur(root, node.val, max_cnt);
                0
            }
        } else {
            0
        }
    }
}
fn main() {}
