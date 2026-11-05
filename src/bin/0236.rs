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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p: i32 = p.as_ref().unwrap().borrow().val;
        let q: i32 = q.as_ref().unwrap().borrow().val;
        Self::lca(root.as_ref(), p, q)
    }
    fn lca(root: Option<&Rc<RefCell<TreeNode>>>, p: i32, q: i32) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(cur) => {
                let n = cur.borrow();
                if n.val == p || n.val == q {
                    return Some(cur.to_owned());
                }
                let find_in_left = Self::lca(n.left.as_ref(), p, q);
                let find_in_right = Self::lca(n.right.as_ref(), p, q);
                match (find_in_left.as_ref(), find_in_right.as_ref()) {
                    (None, None) => None,
                    (Some(v1), Some(v2)) => return Some(cur.to_owned()),
                    (Some(l), None) => Some(l.to_owned()),
                    (None, Some(r)) => Some(r.to_owned()),
                }
            }
        }
    }
}
fn main() {}
