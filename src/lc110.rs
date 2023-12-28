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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::check_height(&root).is_some()
    }
    fn check_height(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
        match root {
            Some(node) => {
                let l = Self::check_height(&node.borrow().left);
                let r = Self::check_height(&node.borrow().right);
                match (l, r) {
                    (Some(l), Some(r)) => {
                        if (l - r).abs() <= 1 {
                            Some(1 + l.max(r))
                        } else {
                            None
                        }
                    }
                    _ => return None,
                }
            }
            None => Some(0),
        }
    }
}

