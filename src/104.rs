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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::action(root.as_ref())
    }

    fn action(root: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(cur) => {
                let l = Self::action(cur.borrow().left.as_ref());
                let r = Self::action(cur.borrow().right.as_ref());
                l.max(r) + 1
            }
            None => 0,
        }
    }
}

fn main() {}
