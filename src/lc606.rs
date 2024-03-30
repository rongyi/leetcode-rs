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
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if let Some(node) = root {
            let node = node.borrow();
            let mut ret = node.val.to_string();
            if node.left.is_some() || node.right.is_some() {
                ret.push_str(&format!("({})", Self::tree2str(node.left.clone())));
                if node.right.is_some() {
                    ret.push_str(&format!("({})", Self::tree2str(node.right.clone())));
                }
            }

            ret
        } else {
            String::new()
        }
    }
}

fn main() {}
