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
        let mut new_tree;
        let mut changed;
        loop {
            (new_tree, changed) = Self::dfs(root.clone(), target);
            if new_tree.is_none() || !changed {
                break;
            }
        }
        new_tree
    }

    fn dfs(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: i32,
    ) -> (Option<Rc<RefCell<TreeNode>>>, bool) {
        if let Some(node) = root {
            // target found
            if node.borrow().left.is_none()
                && node.borrow().right.is_none()
                && node.borrow().val == target
            {
                return (None, true);
            }
            let (lsub, lchanged) = Self::dfs(node.borrow().left.clone(), target);
            let (rsub, rchanged) = Self::dfs(node.borrow().right.clone(), target);
            node.borrow_mut().left = lsub.clone();
            node.borrow_mut().right = rsub.clone();
            (Some(node.clone()), lchanged || rchanged)
        } else {
            (None, false)
        }
    }
}

fn main() {}
