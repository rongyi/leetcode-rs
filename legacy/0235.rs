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
        let p = p.unwrap().borrow().val;
        let q = q.unwrap().borrow().val;
        Self::recur(&root, p, q)
    }
    fn recur(
        node: &Option<Rc<RefCell<TreeNode>>>,
        p: i32,
        q: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(cur) = node {
            let cur = cur.borrow();
            if cur.val == p {
                return node.clone();
            } else if cur.val == q {
                return node.clone();
            }
            let isp_left = p < cur.val;
            let isq_left = q < cur.val;
            if isp_left ^ isq_left {
                return node.clone();
            }
            if isp_left {
                return Self::recur(&cur.left, p, q);
            }
            return Self::recur(&cur.right, p, q);
        }
        None
    }
}

fn main() {}
