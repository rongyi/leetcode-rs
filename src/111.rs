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
use std::i32;
use std::rc::Rc;
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let node = match root {
            None => return 0,
            Some(n) => n,
        };

        let mut q = VecDeque::new();
        q.push_back((node, 1));
        while let Some((cur, path_nodes)) = q.pop_front() {
            let cur = cur.borrow();
            if cur.left.is_none() && cur.right.is_none() {
                return path_nodes;
            }
            if let Some(l) = cur.left.as_ref() {
                q.push_back((l.clone(), path_nodes + 1));
            }
            if let Some(r) = cur.right.as_ref() {
                q.push_back((r.clone(), path_nodes + 1));
            }
        }

        unreachable!()
    }
}
fn main() {}
