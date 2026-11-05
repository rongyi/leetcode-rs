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
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let mut sum = 0;

        Self::dfs(root.as_ref(), &mut sum, low, high);

        sum
    }
    fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, sum: &mut i32, low: i32, high: i32) {
        if let Some(node) = node {
            let node = node.borrow();
            if node.val <= high && node.val >= low {
                *sum += node.val;
            }
            Self::dfs(node.left.as_ref(), sum, low, high);
            Self::dfs(node.right.as_ref(), sum, low, high);
        }
    }
}

fn main() {}
