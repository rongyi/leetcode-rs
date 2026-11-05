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
    pub fn insert_into_max_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() || val > root.as_ref().unwrap().borrow().val {
            let mut node = TreeNode::new(val);
            node.left = root;
            return Some(Rc::new(RefCell::new(node)));
        }

        let root = root.unwrap();
        let right = root.borrow_mut().right.take();
        root.borrow_mut().right = Self::insert_into_max_tree(right, val);

        Some(root)
    }
}

fn main() {}
