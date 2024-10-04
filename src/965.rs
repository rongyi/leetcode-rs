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
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let val = root.as_ref().unwrap().borrow().val;
        Self::dfs(root.as_ref(), val)
    }
    fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, val: i32) -> bool {
        if let Some(node) = root {
            let node = node.borrow();
            if node.val != val {
                return false;
            }
            Self::dfs(node.left.as_ref(), val) && Self::dfs(node.right.as_ref(), val)
        } else {
            true
        }
    }
}

fn main() {}
