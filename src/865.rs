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
    pub fn subtree_with_all_deepest(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.as_ref() {
            let node = node.borrow();
            let l = Self::depth(node.left.as_ref());
            let r = Self::depth(node.right.as_ref());
            if l == r {
                return root.clone();
            } else if l > r {
                return Self::subtree_with_all_deepest(node.left.clone());
            }

            return Self::subtree_with_all_deepest(node.right.clone());
        } else {
            None
        }
    }

    fn depth(root: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            let left = Self::depth(node.left.as_ref());
            let right = Self::depth(node.right.as_ref());
            1 + left.max(right)
        } else {
            0
        }
    }
}

fn main() {}
