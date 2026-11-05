struct Solution;


// Definition for a binary tree node
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
    pub fn subtree_with_all_deepest(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // Returns (depth, LCA candidate)
        Self::dfs(root.as_ref()).1
    }

    fn dfs(node: Option<&Rc<RefCell<TreeNode>>>) -> (i32, Option<Rc<RefCell<TreeNode>>>) {
        if let Some(n) = node {
            let left = Self::dfs(n.borrow().left.as_ref());
            let right = Self::dfs(n.borrow().right.as_ref());

            if left.0 > right.0 {
                // Left subtree is deeper
                (left.0 + 1, left.1)
            } else if right.0 > left.0 {
                // Right subtree is deeper
                (right.0 + 1, right.1)
            } else {
                // Both sides have same depth
                // Current node is the LCA of deepest nodes
                (left.0 + 1, Some(n.clone()))
            }
        } else {
            // Empty node: depth = 0, no LCA
            (0, None)
        }
    }
}

fn main() {}
