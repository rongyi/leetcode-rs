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
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::make_node(&nums, 0, (nums.len() - 1) as i32)
    }

    fn make_node(nums: &[i32], start: i32, end: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if start > end {
            return None;
        }
        if start == end {
            return Some(Rc::new(RefCell::new(TreeNode::new(nums[start as usize]))));
        }
        let mut max_idx = start;
        let mut max_num = nums[start as usize];
        for i in start..=end {
            max_num = max_num.max(nums[i as usize]);
            if max_num == nums[i as usize] {
                max_idx = i;
            }
        }
        let mut node = TreeNode::new(max_num);
        // all in right
        if max_idx == start {
            node.right = Self::make_node(nums, max_idx + 1, end);
        } else if max_idx == end {
            node.left = Self::make_node(nums, start, end - 1);
        } else {
            node.left = Self::make_node(nums, start, max_idx - 1);
            node.right = Self::make_node(nums, max_idx + 1, end);
        }

        Some(Rc::new(RefCell::new(node)))
    }
}

fn main() {}
