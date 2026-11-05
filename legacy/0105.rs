struct Solution;

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
        Self::recursive_build(&preorder[..], &inorder[..])
    }

    fn recursive_build(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() || inorder.is_empty() {
            return None;
        }
        let root_val = preorder[0];
        let root = Rc::new(RefCell::new(TreeNode::new(root_val)));

        let root_idx = inorder.iter().position(|&x| x == root_val).unwrap();
        let inorder_left = &inorder[0..root_idx];
        let inorder_right = &inorder[root_idx + 1..];

        let preorder_left = &preorder[1..1 + root_idx];
        let preorder_right = &preorder[1 + root_idx..];

        root.borrow_mut().left = Self::recursive_build(preorder_left, inorder_left);
        root.borrow_mut().right = Self::recursive_build(preorder_right, inorder_right);

        Some(root)
    }
}

