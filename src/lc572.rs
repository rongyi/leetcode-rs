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
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match root.clone() {
            None => false,
            Some(node) => {
                Self::is_same(root.as_ref(), sub_root.as_ref())
                    || Self::is_subtree(node.borrow().left.clone(), sub_root.clone())
                    || Self::is_subtree(node.borrow().right.clone(), sub_root.clone())
            }
        }
    }

    fn is_same(
        node1: Option<&Rc<RefCell<TreeNode>>>,
        node2: Option<&Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (node1, node2) {
            (Some(node1), Some(node2)) => {
                let node1 = node1.borrow();
                let node2 = node2.borrow();
                node1.val == node2.val
                    && Self::is_same(node1.left.as_ref(), node2.left.as_ref())
                    && Self::is_same(node1.right.as_ref(), node2.right.as_ref())
            }
            (None, None) => true,
            (_, _) => false,
        }
    }
}

fn main() {}
