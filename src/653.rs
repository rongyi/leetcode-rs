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
use std::collections::HashSet;
use std::rc::Rc;
impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut cache = HashSet::new();

        Self::find_two_sum(root.as_ref(), k, &mut cache)
    }
    fn find_two_sum(
        node: Option<&Rc<RefCell<TreeNode>>>,
        k: i32,
        cache: &mut HashSet<i32>,
    ) -> bool {
        if let Some(node) = node {
            let node = node.borrow();
            let diff = k - node.val;
            if cache.contains(&diff) {
                return true;
            }
            cache.insert(node.val);
            Self::find_two_sum(node.left.as_ref(), k, cache)
                || Self::find_two_sum(node.right.as_ref(), k, cache)
        } else {
            false
        }
    }
}

fn main() {}
