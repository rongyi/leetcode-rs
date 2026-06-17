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
        let q = q.as_ref().unwrap().borrow().val;
        Self::lca(root.as_ref(), p, q)
    }
    fn lca(root: Option<&Rc<RefCell<TreeNode>>>, p: i32, q: i32) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(currc) => {
                let cur = currc.borrow();
                let v = cur.val;
                if v == p || v == q {
                    return Some(currc.to_owned());
                }
                let p_is_left = p < v;
                let q_is_left = q < v;
                if p_is_left ^ q_is_left {
                    return Some(currc.to_owned());
                }
                if p_is_left {
                    return Self::lca(cur.left.as_ref(), p, q);
                }
                return Self::lca(cur.right.as_ref(), p, q);
            }
        }
    }
}

fn main() {}
