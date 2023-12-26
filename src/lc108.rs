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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build(&nums[..])
    }
    fn build(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }

        let mid = nums.len() / 2;

        Some(Rc::new(RefCell::new(TreeNode {
            val: nums[mid],
            left: Self::build(&nums[..mid]),
            right: Self::build(&nums[mid + 1..]),
        })))
    }
}

