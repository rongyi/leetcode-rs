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
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut out: Vec<i32> = vec![];
        Self::collect(root.as_ref(), &mut out);

        Self::reconstruct(&out)
    }
    fn reconstruct(lst: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if lst.is_empty() {
            return None;
        }
        let mid = lst.len() / 2;
        let val = lst[mid];
        let mut node = TreeNode::new(val);
        node.left = Self::reconstruct(&lst[..mid]);
        node.right = Self::reconstruct(&lst[mid + 1..]);

        Some(Rc::new(RefCell::new(node)))
    }
    fn collect(root: Option<&Rc<RefCell<TreeNode>>>, out: &mut Vec<i32>) {
        if let Some(node) = root {
            let node = node.borrow();
            Self::collect(node.left.as_ref(), out);
            out.push(node.val);
            Self::collect(node.right.as_ref(), out);
        }
    }
}

fn main() {}
