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
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut q = VecDeque::new();
        let mut ret: i64 = 0;
        q.push_back((root.clone(), 0));
        while !q.is_empty() {
            let sz = q.len();
            // l is the begnning of current level start
            let l = q.front().unwrap().1;
            let mut r = 0;
            for _ in 0..sz {
                let (node, idx) = q.pop_front().unwrap();
                if let Some(node) = node {
                    let node = node.borrow();
                    if node.left.is_some() {
                        q.push_back((node.left.clone(), (idx - l) * 2));
                    }
                    if node.right.is_some() {
                        q.push_back((node.right.clone(), (idx - l) * 2 + 1));
                    }
                }
                r = r.max(idx);
            }
            ret = ret.max(r - l + 1);
        }
        ret as i32
    }
}

fn main() {}
