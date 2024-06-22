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
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut prev: Option<i32> = None;
        let mut ret: i32 = i32::MAX;

        Self::in_order(root.as_ref(), &mut prev, &mut ret);
        ret
    }
    fn in_order(node: Option<&Rc<RefCell<TreeNode>>>, prev: &mut Option<i32>, ret: &mut i32) {
        if let Some(node) = node {
            Self::in_order(node.borrow().left.as_ref(), prev, ret);
            let val = node.borrow().val;
            if prev.is_some() {
                *ret = (*ret).min(val - prev.unwrap());
            }
            *prev = Some(val);
            Self::in_order(node.borrow().right.as_ref(), prev, ret);
        }
    }
}

fn main() {}
