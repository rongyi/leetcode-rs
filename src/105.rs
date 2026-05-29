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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::action(&preorder[..], &inorder[..])
    }

    fn action(preorder: &[i32], indorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            return None;
        }
        let root_val = preorder[0];
        let mut root = TreeNode::new(root_val);
        let mut split_idx = 0;
        for (i, &v) in indorder.iter().enumerate() {
            if v == root_val {
                split_idx = i;
            }
        }
        // left has split_idx total, right has sz - split_idx - 1
        let left_child = Self::action(&preorder[1..1 + split_idx], &indorder[0..split_idx]);

        let right_child = Self::action(&preorder[1 + split_idx..], &indorder[split_idx + 1..]);

        root.left = left_child;
        root.right = right_child;

        Some(Rc::new(RefCell::new(root)))
    }
}

fn main() {}
