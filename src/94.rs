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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut out = vec![];
        Self::inorder(root.as_ref(), &mut out);
        out
    }
    fn inorder(cur: Option<&Rc<RefCell<TreeNode>>>, out: &mut Vec<i32>) {
        if let Some(cur) = cur {
            Self::inorder(cur.borrow().left.as_ref(), out);

            out.push(cur.borrow().val);

            Self::inorder(cur.borrow().right.as_ref(), out);
        }
    }
}

fn main() {}
