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
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut total = 0;

        Self::recur(root.as_ref(), &mut total, false);
        total
    }

    fn recur(root: Option<&Rc<RefCell<TreeNode>>>, out: &mut i32, is_parent_left: bool) {
        if let Some(cur) = root {
            let n = cur.borrow();
            if n.left.is_none() && n.right.is_none() && is_parent_left {
                *out += n.val;
            }
            Self::recur(n.left.as_ref(), out, true);
            Self::recur(n.right.as_ref(), out, false);
        }
    }
}
fn main() {}
