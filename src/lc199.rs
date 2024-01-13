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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ret: Vec<i32> = Vec::new();
        if let Some(node) = root {
            let mut level = vec![node];

            while !level.is_empty() {
                ret.push(level.last().unwrap().borrow().val);
                let mut next_level: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
                for node in &level {
                    let node = node.borrow();
                    if let Some(left) = &node.left {
                        next_level.push(left.clone());
                    }
                    if let Some(right) = &node.right {
                        next_level.push(right.clone());
                    }
                }

                level = next_level;
            }
        }

        ret
    }
}

fn main() {}
