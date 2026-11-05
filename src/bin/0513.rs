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
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut layers = vec![];
        Self::recur(root.as_ref(), &mut layers, 1);

        layers.last().unwrap()[0]
    }

    fn recur(node: Option<&Rc<RefCell<TreeNode>>>, out: &mut Vec<Vec<i32>>, cur_height: usize) {
        if let Some(node) = node {
            let n = node.borrow();
            let cur_val = n.val;
            if cur_height > out.len() {
                out.push(vec![]);
            }
            // only one numer is enough
            if out[cur_height - 1].is_empty() {
                out[cur_height - 1].push(cur_val);
            }

            Self::recur(node.borrow().left.as_ref(), out, cur_height + 1);
            Self::recur(node.borrow().right.as_ref(), out, cur_height + 1);
        }
    }
}

fn main() {}
