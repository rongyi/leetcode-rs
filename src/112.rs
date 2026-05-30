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
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        let node = match root {
            None => return false,
            Some(n) => n,
        };
        let mut q = VecDeque::new();
        let val = node.borrow().val;
        // path sum including current node
        q.push_back((node, val));

        while let Some((cur, acc)) = q.pop_front() {
            let cur = cur.borrow();
            if cur.left.is_none() && cur.right.is_none() && acc == target_sum {
                return true;
            }
            if let Some(l) = cur.left.as_ref() {
                q.push_back((l.clone(), l.borrow().val + acc));
            }
            if let Some(r) = cur.right.as_ref() {
                q.push_back((r.clone(), r.borrow().val + acc));
            }
        }

        false
    }
}

fn main() {}
