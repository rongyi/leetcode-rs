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
    pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }
        let mut q = VecDeque::new();

        let mut prev_hole = false;
        q.push_back(root.clone());

        while !q.is_empty() {
            let sz = q.len();
            let mut has_hole = false;
            for _ in 0..sz {
                let cur = q.pop_front().unwrap();
                if let Some(node) = cur {
                    if has_hole || prev_hole {
                        return false;
                    }
                    let node = node.borrow();
                    q.push_back(node.left.clone());
                    q.push_back(node.right.clone());
                } else {
                    has_hole = true;
                    continue;
                }
            }

            prev_hole = has_hole;
        }

        true
    }
}

fn main() {}
