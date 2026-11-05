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
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
        if let Some(root) = root {
            let mut q = VecDeque::new();
            q.push_back(root);

            while !q.is_empty() {
                let mut cur_level = Vec::new();
                let sz = q.len();
                for _ in 0..sz {
                    let cur = q.pop_front().unwrap();
                    let cur = cur.borrow();
                    cur_level.push(cur.val);

                    if let Some(left) = &cur.left {
                        q.push_back(left.clone());
                    }
                    if let Some(right) = &cur.right {
                        q.push_back(right.clone());
                    }
                }
                ret.push(cur_level);
            }
        }

        ret.reverse();

        ret
    }
}

