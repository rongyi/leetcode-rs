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
    pub fn lca_deepest_leaves(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }
        let l = Self::count_depth(root.as_ref().unwrap().borrow().left.as_ref());
        let r = Self::count_depth(root.as_ref().unwrap().borrow().right.as_ref());
        if l == r {
            return root;
        }
        if l < r {
            return Self::lca_deepest_leaves(root.as_ref().unwrap().borrow().right.clone());
        }

        return Self::lca_deepest_leaves(root.as_ref().unwrap().borrow().left.clone());
    }

    fn count_depth(node: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = node {
            let l = Self::count_depth(node.borrow().left.as_ref());
            let r = Self::count_depth(node.borrow().right.as_ref());

            1 + l.max(r)
        } else {
            0
        }
    }
}

fn main() {}
