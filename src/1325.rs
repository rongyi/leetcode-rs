#![allow(dead_code)]
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
    pub fn remove_leaf_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let mut node_ref = node.borrow_mut();

            node_ref.left = Self::remove_leaf_nodes(node_ref.left.take(), target);
            node_ref.right = Self::remove_leaf_nodes(node_ref.right.take(), target);

            // delete current node
            if node_ref.left.is_none() && node_ref.right.is_none() && node_ref.val == target {
                return None;
            }

            drop(node_ref);
            Some(node)
        } else {
            None
        }
    }
}

fn main() {}
