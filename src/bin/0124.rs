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
use std::i32;
use std::rc::Rc;
impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut out = i32::MIN;

        Self::action(root.as_ref(), &mut out);
        out
    }
    // return max sum for subpath though current node(include this current node)
    fn action(cur: Option<&Rc<RefCell<TreeNode>>>, out: &mut i32) -> i32 {
        match cur {
            None => 0,
            Some(cur) => {
                let cur = cur.borrow();
                // means I can choose not to have both sub path to left child and right child
                let left = Self::action(cur.left.as_ref(), out).max(0);
                let right = Self::action(cur.right.as_ref(), out).max(0);

                let cur_sum = cur.val + left.max(right);

                *out = (*out).max(cur.val + left + right);

                cur_sum
            }
        }
    }
}
fn main() {}
