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
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut sum = 0;
        Self::dfs(root.as_ref(), &mut sum);
        root
    }

    fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, sum: &mut i32) {
        if let Some(node) = node {
            let mut node = node.borrow_mut();
            Self::dfs(node.right.as_ref(), sum);
            *sum += node.val;
            node.val = *sum;

            Self::dfs(node.left.as_ref(), sum);
        }
    }
}

fn main() {}
