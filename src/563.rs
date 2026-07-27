
struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum_tilt = 0;
        Self::dfs(root.as_ref(), &mut sum_tilt);
        sum_tilt
    }
    fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, sum: &mut i32) -> i32 {
        if let Some(node) = node {
            let node = node.borrow();
            let left = Self::dfs(node.left.as_ref(), sum);
            let right = Self::dfs(node.right.as_ref(), sum);
            *sum += (left - right).abs();

            node.val + left + right
        } else {
            0
        }
    }
}



fn main(){}
