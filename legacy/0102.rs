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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
        let mut queue = VecDeque::new();
        if let Some(node) = root {
            queue.push_back(node);
            while !queue.is_empty() {
                let sz = queue.len();
                let mut cur_level = Vec::new();
                for _ in 0..sz {
                    let cur = queue.pop_front().unwrap();
                    let cur_borrow = cur.borrow();
                    cur_level.push(cur_borrow.val);
                    if let Some(left) = &cur_borrow.left {
                        queue.push_back(left.clone());
                    }
                    if let Some(right) = &cur_borrow.right {
                        queue.push_back(right.clone());
                    }
                }
                ret.push(cur_level);
            }
        }

        ret
    }
}

