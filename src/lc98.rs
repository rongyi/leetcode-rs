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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn recursive(node: &Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
            if let Some(inner) = node {
                let val = inner.borrow().val as i64;
                if val <= min || val >= max {
                    return false;
                }
                return recursive(&inner.borrow().left, min, val)
                    && recursive(&inner.borrow().right, val, max);
            }
            true
        }

        recursive(&root, i64::MIN, i64::MAX)
    }
}

