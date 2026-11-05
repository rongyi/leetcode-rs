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
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut out = 0;

        Self::action(root.as_ref(), 0, &mut out);
        out
    }
    //  acc, parent accumuation, not include itself
    fn action(root: Option<&Rc<RefCell<TreeNode>>>, mut acc: i32, out: &mut i32) {
        if let Some(cur) = root {
            let cur = cur.borrow();
            acc = acc * 10 + cur.val;
            if cur.left.is_none() && cur.right.is_none() {
                *out += acc;
            }
            Self::action(cur.left.as_ref(), acc, out);
            Self::action(cur.right.as_ref(), acc, out);
        }
    }
}

fn main() {}
