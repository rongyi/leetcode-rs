#![allow(dead_code)]

struct Solution;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }

        // Use a queue for level-order traversal
        let mut queue = VecDeque::new();
        queue.push_back(root);

        let mut level = 0;

        while !queue.is_empty() {
            let level_size = queue.len();
            let mut prev_val = if level % 2 == 0 { 0 } else { 1_000_001 };

            for _ in 0..level_size {
                if let Some(node) = queue.pop_front() {
                    if let Some(node_ref) = node {
                        let node_borrow = node_ref.borrow();
                        let val = node_borrow.val;

                        // Check even level conditions: values should be odd and strictly increasing
                        if level % 2 == 0 {
                            if val % 2 == 0 || val <= prev_val {
                                return false;
                            }
                        }
                        // Check odd level conditions: values should be even and strictly decreasing
                        else {
                            if val % 2 == 1 || val >= prev_val {
                                return false;
                            }
                        }

                        prev_val = val;

                        // Add children to queue
                        if let Some(left) = &node_borrow.left {
                            queue.push_back(Some(Rc::clone(left)));
                        }

                        if let Some(right) = &node_borrow.right {
                            queue.push_back(Some(Rc::clone(right)));
                        }
                    }
                }
            }

            level += 1;
        }

        true
    }
}

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

fn main() {}
