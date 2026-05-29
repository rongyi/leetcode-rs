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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root.as_ref() {
            None => true,
            Some(cur) => Self::check(cur.borrow().left.as_ref(), cur.borrow().right.as_ref()),
        }
    }
    fn check(l: Option<&Rc<RefCell<TreeNode>>>, r: Option<&Rc<RefCell<TreeNode>>>) -> bool {
        match (l, r) {
            (Some(n1), Some(n2)) => {
                n1.borrow().val == n2.borrow().val
                    && Self::check(n1.borrow().left.as_ref(), n2.borrow().right.as_ref())
                    && Self::check(n1.borrow().right.as_ref(), n2.borrow().left.as_ref())
            }
            (None, None) => true,
            _ => false,
        }
    }
}

fn main() {}
