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
    pub fn trim_bst(
        node: Option<Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match node {
            None => None,
            Some(node1) => {
                let mut node = node1.borrow_mut();
                if node.val < low {
                    return Self::trim_bst(node.right.take(), low, high);
                } else if node.val > high {
                    return Self::trim_bst(node.left.take(), low, high);
                }
                node.left = Self::trim_bst(node.left.take(), low, high);
                node.right = Self::trim_bst(node.right.take(), low, high);

                Some(node1.clone())
            }
        }
    }
}

fn main() {}
