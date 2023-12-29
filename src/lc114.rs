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

// Definition for a binary tree node.
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            let mut node_borrow = node.borrow_mut();

            Self::flatten(&mut node_borrow.left);
            Self::flatten(&mut node_borrow.right);

            let left_subtree = node_borrow.left.take();
            let right_subtree = node_borrow.right.take();

            if left_subtree.is_some() {
                node_borrow.right = left_subtree.clone();

                let mut rightmost = left_subtree;
                while rightmost.as_ref().unwrap().borrow().right.is_some() {
                    let tmp = rightmost.as_ref().unwrap().borrow().right.clone();
                    rightmost = tmp;
                }

                rightmost.as_ref().unwrap().borrow_mut().right = right_subtree;
            } else {
                node_borrow.right = right_subtree;
            }
        }
    }
}

