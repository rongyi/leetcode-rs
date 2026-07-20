struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut layers: Vec<i32> = Vec::new();
        Self::recur(root.as_ref(), &mut layers, 1);
        layers
    }

    fn recur(node: Option<&Rc<RefCell<TreeNode>>>, layers: &mut Vec<i32>, cur_depth: usize) {
        if let Some(node) = node {
            if cur_depth > layers.len() {
                layers.push(i32::MIN);
            }
            if node.borrow().val > layers[cur_depth - 1] {
                layers[cur_depth - 1] = node.borrow().val;
            }
            Self::recur(node.borrow().left.as_ref(), layers, cur_depth + 1);
            Self::recur(node.borrow().right.as_ref(), layers, cur_depth + 1);
        }
    }
}

fn main() {}
