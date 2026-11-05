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
        Self::action(&inorder[..], &postorder[..])
    }

    fn action(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.is_empty() {
            return None;
        }
        let sz = inorder.len();
        let root_val = postorder[sz - 1];
        let mut root = TreeNode::new(root_val);

        let mut split_idx = 0;
        for (i, &v) in inorder.iter().enumerate() {
            if v == root_val {
                split_idx = i;
                break;
            }
        }
        // so left child has total split_idx len nodes
        // and right child has total sz - split_idx - 1
        let left = Self::action(&inorder[..split_idx], &postorder[..split_idx]);
        let right = Self::action(&inorder[split_idx + 1..], &postorder[split_idx..sz - 1]);
        root.left = left;
        root.right = right;

        Some(Rc::new(RefCell::new(root)))
    }
}
fn main() {}
