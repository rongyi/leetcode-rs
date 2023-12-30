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
            let mut node = node.borrow_mut();
            Self::flatten(&mut node.left);
            Self::flatten(&mut node.right);

            let left = node.left.take();
            let right = node.right.take();

            if left.is_some() {
                node.right = left.clone();

                let mut rightmost = left;
                while rightmost.as_ref().unwrap().borrow().right.is_some() {
                    let tmp = rightmost.as_ref().unwrap().borrow().right.clone();
                    rightmost = tmp;
                }
                rightmost.as_ref().unwrap().borrow_mut().right = right;
            } else {
                node.right = right;
            }
        }
    }
}

