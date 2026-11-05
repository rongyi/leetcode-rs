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
use std::i64;
use std::rc::Rc;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::validate_bst(root.as_ref(), i64::MIN, i64::MAX)
    }
    fn validate_bst(cur: Option<&Rc<RefCell<TreeNode>>>, l: i64, r: i64) -> bool {
        if let Some(cur) = cur {
            let v = cur.borrow().val as i64;
            if v < r && v > l {
                Self::validate_bst(cur.borrow().left.as_ref(), l, v)
                    && Self::validate_bst(cur.borrow().right.as_ref(), v, r)
            } else {
                false
            }
        } else {
            true
        }
    }
}

fn main() {}
