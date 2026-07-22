struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ret = i32::MAX;
        let mut prev: Option<i32> = None;
        Self::dfs(root.as_ref(), &mut prev, &mut ret);
        ret
    }

    fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, prev: &mut Option<i32>, ret: &mut i32) {
        if let Some(node) = node {
            let node = node.borrow();
            Self::dfs(node.left.as_ref(), prev, ret);

            if let Some(prev) = prev {
                *ret = (*ret).min(node.val - *prev);
            }
            *prev = Some(node.val);

            Self::dfs(node.right.as_ref(), prev, ret);
        }
    }
}

fn main() {}
