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
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        let mut x_level = 0;
        let mut x_parent: Option<i32> = None;
        let mut y_level = 0;
        let mut y_parent: Option<i32> = None;

        Self::dfs(root.as_ref(), None, &mut x_level, &mut x_parent, x, 0);
        Self::dfs(root.as_ref(), None, &mut y_level, &mut y_parent, y, 0);

        // x != y so, if either parent is None, we can return false
        if x_parent.is_none() || y_parent.is_none() {
            return false;
        }

        if x_level != y_level {
            return false;
        }
        if x_parent.unwrap() == y_parent.unwrap() {
            return false;
        }

        true
    }
    fn dfs(
        node: Option<&Rc<RefCell<TreeNode>>>,
        parent: Option<i32>,
        level: &mut i32,
        target_parent: &mut Option<i32>,
        x: i32,
        height: i32,
    ) {
        if let Some(n) = node {
            let n = n.borrow();
            if n.val == x {
                *level = height;
                *target_parent = parent;
                return;
            }
            Self::dfs(
                n.left.as_ref(),
                Some(n.val),
                level,
                target_parent,
                x,
                height + 1,
            );
            Self::dfs(
                n.right.as_ref(),
                Some(n.val),
                level,
                target_parent,
                x,
                height + 1,
            );
        }
    }
}

fn main() {}
