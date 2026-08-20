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
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut l1 = vec![];
        let mut l2 = vec![];
        Self::collect_leaf(root1.as_ref(), &mut l1);
        Self::collect_leaf(root2.as_ref(), &mut l2);

        l1 == l2
    }
    fn collect_leaf(node: Option<&Rc<RefCell<TreeNode>>>, out: &mut Vec<i32>) {
        if let Some(node) = node {
            let node = node.borrow();
            if node.left.is_none() && node.right.is_none() {
                out.push(node.val);
            }
            Self::collect_leaf(node.left.as_ref(), out);
            Self::collect_leaf(node.right.as_ref(), out);
        }
    }
}
fn main() {}
