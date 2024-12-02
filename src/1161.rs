#![allow(dead_code)]

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
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut acc: Vec<Vec<i32>> = Vec::new();
        Self::bfs(root.as_ref(), &mut acc, 1);
        let sum: Vec<i32> = acc.iter().map(|lst| lst.iter().sum()).collect();
        let mut max_val = i32::MIN;
        let mut max_idx = 0;
        for (i, &val) in sum.iter().enumerate() {
            // not >=
            if val > max_val {
                max_val = val;
                max_idx = i;
            }
        }
        // 1 indexed
        max_idx as i32 + 1
    }

    fn bfs(root: Option<&Rc<RefCell<TreeNode>>>, acc: &mut Vec<Vec<i32>>, height: usize) {
        if let Some(root) = root {
            if acc.len() < height {
                acc.push(vec![]);
            }

            let node = root.borrow();
            acc[height - 1].push(node.val);
            Self::bfs(node.left.as_ref(), acc, height + 1);
            Self::bfs(node.right.as_ref(), acc, height + 1);
        }
    }
}
fn main() {}
