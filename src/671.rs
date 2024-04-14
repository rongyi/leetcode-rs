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
    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root.clone() {
            let node = node.borrow();
            Self::recur(root.as_ref(), node.val)
        } else {
            -1
        }
    }
    fn recur(node: Option<&Rc<RefCell<TreeNode>>>, first_val: i32) -> i32 {
        if let Some(node) = node {
            let node = node.borrow();
            if node.val != first_val {
                return node.val;
            }
            let left = Self::recur(node.left.as_ref(), first_val);
            let right = Self::recur(node.right.as_ref(), first_val);
            if left == -1 {
                return right;
            }
            if right == -1 {
                return left;
            }
            left.min(right)
        } else {
            -1
        }
    }
}
fn main() {}
