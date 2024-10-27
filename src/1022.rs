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
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut total = 0;
        Self::recur(root.as_ref(), 0, &mut total);

        total
    }

    fn recur(node: Option<&Rc<RefCell<TreeNode>>>, sum: i32, total: &mut i32) {
        if let Some(node) = node {
            let n = node.borrow();
            if n.left.is_none() && n.right.is_none() {
                *total += (sum << 1) + n.val;
            }
            Self::recur(n.left.as_ref(), (sum << 1) + n.val, total);
            Self::recur(n.right.as_ref(), (sum << 1) + n.val, total);
        }
    }
}

fn main() {}
