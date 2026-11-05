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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        Self::recursive(&root, target_sum, 0)
    }
    fn recursive(root: &Option<Rc<RefCell<TreeNode>>>, target_sum: i32, prefix: i32) -> bool {
        if let Some(node) = root {
            let cur_prefix = prefix + node.borrow().val;
            let l = &node.borrow().left;
            let r = &node.borrow().right;
            if l.is_none() && r.is_none() && cur_prefix == target_sum {
                return true;
            }
            Self::recursive(l, target_sum, cur_prefix) || Self::recursive(r, target_sum, cur_prefix)
        } else {
            false
        }
    }
}
