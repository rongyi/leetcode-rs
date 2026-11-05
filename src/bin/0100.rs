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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Self::try_same_tree(p.as_ref(), q.as_ref())
    }
    fn try_same_tree(p: Option<&Rc<RefCell<TreeNode>>>, q: Option<&Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (Some(n1), Some(n2)) => {
                n1.borrow().val == n2.borrow().val
                    && Self::try_same_tree(n1.borrow().left.as_ref(), n2.borrow().left.as_ref())
                    && Self::try_same_tree(n1.borrow().right.as_ref(), n2.borrow().right.as_ref())
            }
            (None, None) => true,
            _ => false,
        }
    }
}
fn main() {}
