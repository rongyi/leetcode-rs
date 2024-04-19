#![allow(dead_code)]

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
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let new_node = TreeNode::new(val);
        if root.is_none() {
            return Some(Rc::new(RefCell::new(new_node)));
        }
        Self::do_insert(root.clone(), new_node);

        root
    }
    fn do_insert(mut root: Option<Rc<RefCell<TreeNode>>>, new_node: TreeNode) {
        if let Some(node) = root {
            let mut node = node.borrow_mut();
            if new_node.val < node.val {
                if node.left.is_none() {
                    node.left = Some(Rc::new(RefCell::new(new_node)));
                    return;
                }
                Self::do_insert(node.left.clone(), new_node);
            } else {
                if node.right.is_none() {
                    node.right = Some(Rc::new(RefCell::new(new_node)));
                    return;
                }
                Self::do_insert(node.right.clone(), new_node);
            }
        }
    }
}

fn main() {}
