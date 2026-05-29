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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut ok = true;
        Self::action(root.as_ref(), &mut ok);
        ok
    }
    fn action(root: Option<&Rc<RefCell<TreeNode>>>, out: &mut bool) -> i32 {
        if !*out {
            // no need to action, quick return
            return -1;
        }
        match root {
            Some(cur) => {
                let cur = cur.borrow();
                let l = Self::action(cur.left.as_ref(), out);
                let r = Self::action(cur.right.as_ref(), out);
                if (l - r).abs() > 1 {
                    *out = false;
                }
                1 + l.max(r)
            }
            None => 0,
        }
    }
}

fn main() {}
