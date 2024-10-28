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
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(root.as_ref(), 10000, 0)
    }
    fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, mut cur_min: i32, mut cur_max: i32) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            cur_min = node.val.min(cur_min);
            cur_max = node.val.max(cur_max);
            let v1 = Self::dfs(node.left.as_ref(), cur_min, cur_max);
            let v2 = Self::dfs(node.right.as_ref(), cur_min, cur_max);
            v1.max(v2)
        } else {
            cur_max - cur_min
        }
    }
}

fn main() {}
