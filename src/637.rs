struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut levels = Vec::new();
        Self::visit(root.as_ref(), &mut levels, 1);
        levels
            .into_iter()
            .map(|v| {
                let sz = v.len();
                let sum: f64 = v.into_iter().map(|v| v as f64).sum();
                sum as f64 / sz as f64
            })
            .collect()
    }

    fn visit(node: Option<&Rc<RefCell<TreeNode>>>, levels: &mut Vec<Vec<i32>>, cur_height: usize) {
        if let Some(node) = node {
            if levels.len() < cur_height {
                levels.push(Vec::new());
            }
            let node = node.borrow();
            levels[cur_height - 1].push(node.val);
            Self::visit(node.left.as_ref(), levels, cur_height + 1);
            Self::visit(node.right.as_ref(), levels, cur_height + 1);
        }
    }
}

fn main() {}
