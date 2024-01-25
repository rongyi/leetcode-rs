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
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut hl = 0;
        let mut p = root.clone();
        while let Some(node) = p {
            hl += 1;
            p = node.borrow().left.clone();
        }

        let mut hr = 0;
        let mut p = root.clone();
        while let Some(node) = p {
            hr += 1;
            p = node.borrow().right.clone();
        }
        if hl == hr {
            return (1 << hl) - 1;
        }

        1 + Self::count_nodes(root.as_ref().unwrap().borrow().left.clone())
            + Self::count_nodes(root.as_ref().unwrap().borrow().right.clone())
    }
}

fn main() {}
