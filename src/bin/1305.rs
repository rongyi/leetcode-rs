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
    pub fn get_all_elements(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<i32> {
        let mut t1 = Vec::new();
        let mut t2 = Vec::new();
        Self::in_order(root1.as_ref(), &mut t1);
        Self::in_order(root2.as_ref(), &mut t2);

        t1.extend(t2);
        t1.sort_unstable();

        t1
    }

    fn in_order(node: Option<&Rc<RefCell<TreeNode>>>, out: &mut Vec<i32>) {
        if let Some(node) = node {
            let node = node.borrow();
            Self::in_order(node.left.as_ref(), out);
            out.push(node.val);
            Self::in_order(node.right.as_ref(), out);
        }
    }
}

fn main() {}
