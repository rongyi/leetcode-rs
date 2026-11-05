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
    pub fn sufficient_subset(
        root: Option<Rc<RefCell<TreeNode>>>,
        limit: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }
        let root_node = root.unwrap();
        let val = root_node.borrow().val;
        if root_node.borrow().left.is_none() && root_node.borrow().right.is_none() {
            return if val < limit { None } else { Some(root_node) };
        }
        let left = {
            let left = root_node.borrow_mut().left.take();
            Self::sufficient_subset(left, limit - val)
        };
        let right = {
            let right = root_node.borrow_mut().right.take();
            Self::sufficient_subset(right, limit - val)
        };
        root_node.borrow_mut().left = left;
        root_node.borrow_mut().right = right;
        if root_node.borrow().left.is_none() && root_node.borrow().right.is_none() {
            None
        } else {
            Some(root_node)
        }
    }
}

fn main() {}
