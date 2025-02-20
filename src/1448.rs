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
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut valid_total = 0;

        Self::dfs(root.as_ref(), None, &mut valid_total);

        valid_total + 1
    }

    fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, mut max_val: Option<i32>, valid_total: &mut i32) {
        if let Some(node) = node {
            let node = node.borrow();
            if let Some(cur_max) = max_val {
                if node.val >= cur_max {
                    *valid_total += 1;
                }
                // for lower node
                max_val = Some(cur_max.max(node.val))
            } else {
                max_val = Some(node.val);
            }
            Self::dfs(node.left.as_ref(), max_val, valid_total);
            Self::dfs(node.right.as_ref(), max_val, valid_total);
        }
    }
}

fn main() {}
