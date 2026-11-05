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
    // Leaf nodes have either the value 0 or 1, where 0 represents False and 1 represents True.
    // Non-leaf nodes have either the value 2 or 3, where 2 represents the boolean OR and 3 represents the boolean AND.
    // Every node has either 0 or 2 children.
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::dfs(root.as_ref())
    }
    pub fn dfs(root: Option<&Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = root {
            let node = node.borrow();
            // ok, a leaf node
            if node.left.is_none() {
                if node.val == 1 {
                    return true;
                } else {
                    return false;
                }
            } else {
                let lval = Self::dfs(node.left.as_ref());
                let rval = Self::dfs(node.right.as_ref());
                if node.val == 2 {
                    return lval || rval;
                } else {
                    return lval && rval;
                }
            }
        } else {
            // unreachable
            true
        }
    }
}
fn main() {}
