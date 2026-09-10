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
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }
        let mut q = VecDeque::new();

        q.push_back(root.clone());

        while let Some(cur) = q.pop_front() {
            match cur {
                Some(node) => {
                    let n = node.borrow();
                    q.push_back(n.left.clone());
                    q.push_back(n.right.clone());
                }
                None => {
                    return q.into_iter().all(|n| n.is_none());
                }
            }
        }

        true
    }
}

fn main() {}
