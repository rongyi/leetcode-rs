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
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut valid_acc = 0;

        Self::dfs(root.as_ref(), &mut valid_acc);
        valid_acc
    }

    // return (sum, node_count) of current subtree
    pub fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, valid_acc: &mut i32) -> (i32, i32) {
        if let Some(node) = root {
            let val = node.borrow().val;
            let left = Self::dfs(node.borrow().left.as_ref(), valid_acc);
            let right = Self::dfs(node.borrow().right.as_ref(), valid_acc);
            let sum = val + left.0 + right.0;
            let nodes = 1 + left.1 + right.1;
            if sum / nodes == val {
                *valid_acc += 1;
            }
            (sum, nodes)
        } else {
            (0, 0)
        }
    }
}

fn main() {}
