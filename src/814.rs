#![allow(dead_code)]

struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
//
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
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::do_prune(root)
    }

    fn do_prune(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let mut node = node.borrow_mut();
            let lok = Self::has_one(node.left.as_ref());
            let rok = Self::has_one(node.right.as_ref());
            // no one can save you, root, you are fucked
            if !lok && !rok {
                // worse
                if node.val == 0 {
                    return None;
                }
                // only the node it Self
                return Some(Rc::new(RefCell::new(TreeNode::new(node.val))));
            }

            if lok {
                node.left = Self::do_prune(node.left.clone());
            } else {
                node.left = None;
            }

            if rok {
                node.right = Self::do_prune(node.right.clone());
            } else {
                node.right = None;
            }

            // root can be saved whatever it's value is
            return Some(Rc::new(RefCell::new(TreeNode {
                val: node.val,
                left: node.left.clone(),
                right: node.right.clone(),
            })));
        } else {
            None
        }
    }

    fn has_one(root: Option<&Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = root {
            let node = node.borrow();
            if node.val == 1 {
                return true;
            }

            Self::has_one(node.left.as_ref()) || Self::has_one(node.right.as_ref())
        } else {
            false
        }
    }
}

fn main() {}
