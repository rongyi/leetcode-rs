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
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::recursive(&inorder[..], &postorder[..])
    }
    fn recursive(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.is_empty() || postorder.is_empty() {
            return None;
        }
        let root_val = postorder[postorder.len() - 1];
        let root = Rc::new(RefCell::new(TreeNode::new(root_val)));

        let root_idx = inorder.iter().position(|&x| x == root_val).unwrap();
        let inorder_left = &inorder[0..root_idx];
        let inorder_right = &inorder[root_idx + 1..];

        let postorder_left = &postorder[0..inorder_left.len()];
        let postorder_right = &postorder[inorder_left.len()..postorder.len() - 1];
        root.borrow_mut().left = Self::recursive(inorder_left, postorder_left);
        root.borrow_mut().right = Self::recursive(inorder_right, postorder_right);

        Some(root)
    }
}

