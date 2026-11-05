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
        let mut layers: Vec<Vec<i32>> = Vec::new();
        Self::recur(root.as_ref(), &mut layers, 1);
        let bottom_layers = layers.pop().unwrap();
        bottom_layers[0]
    }

    fn recur(node: Option<&Rc<RefCell<TreeNode>>>, layers: &mut Vec<Vec<i32>>, cur_depth: usize) {
        if let Some(node) = node {
            if cur_depth > layers.len() {
                layers.push(Vec::new());
            }
            layers[cur_depth - 1].push(node.borrow().val);
            Self::recur(node.borrow().left.as_ref(), layers, cur_depth + 1);
            Self::recur(node.borrow().right.as_ref(), layers, cur_depth + 1);
        }
    }
}

fn main() {}
