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
        Self::action(&nums[..])
    }
    fn action(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }
        let mid = nums.len() / 2;
        let mid_val = nums[mid];
        let mut root = TreeNode::new(mid_val);
        let left = Self::action(&nums[..mid]);
        let right = Self::action(&nums[mid + 1..]);
        root.left = left;
        root.right = right;

        Some(Rc::new(RefCell::new(root)))
    }
}
// Definition for a binary tree node.
fn main() {}
