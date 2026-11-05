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
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::count_child(root.as_ref())
    }
    fn count_child(root: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(cur) => {
                let cur = cur.borrow();

                let mut hl = 1;
                let mut p = cur.left.clone();
                while let Some(l) = p {
                    hl += 1;
                    p = l.borrow().left.clone();
                }

                let mut hr = 1;
                let mut p = cur.right.clone();
                while let Some(r) = p {
                    hr += 1;
                    p = r.borrow().right.clone();
                }
                if hl == hr {
                    return (1 << hr) - 1;
                }
                1 + Self::count_child(cur.left.as_ref()) + Self::count_child(cur.right.as_ref())
            }
        }
    }
}

fn main() {}
