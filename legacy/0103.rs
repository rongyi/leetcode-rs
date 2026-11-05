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
use std::collections::{LinkedList, VecDeque};
use std::rc::Rc;
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
        if let Some(root) = root {
            let mut queue = VecDeque::new();
            let mut is_left_to_right = true;
            queue.push_back(root);

            while !queue.is_empty() {
                let sz = queue.len();
                let mut cur_level = LinkedList::new();

                for _ in 0..sz {
                    let cur = queue.pop_front().unwrap();
                    let cur_borrow = cur.borrow();
                    if is_left_to_right {
                        cur_level.push_back(cur_borrow.val);
                    } else {
                        cur_level.push_front(cur_borrow.val);
                    }

                    if let Some(left) = &cur_borrow.left {
                        queue.push_back(left.clone());
                    }
                    if let Some(right) = &cur_borrow.right {
                        queue.push_back(right.clone());
                    }
                }
                is_left_to_right = !is_left_to_right;
                ret.push(cur_level.into_iter().collect());
            }
        }

        ret
    }
}

