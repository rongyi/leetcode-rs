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
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_sum = i32::MIN;
        Self::recursive(&root, &mut max_sum);

        max_sum
    }
    fn recursive(root: &Option<Rc<RefCell<TreeNode>>>, max_sum: &mut i32) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();

            // take left subtree when subtree sum >= 0
            let left_sum = std::cmp::max(0, Self::recursive(&node.left, max_sum));

            // take right subtree when subtree sum >= 0
            let right_sum = std::cmp::max(0, Self::recursive(&node.right, max_sum));

            // the path contain left  right and current node
            let cur_sum = node.val + left_sum + right_sum;
            *max_sum = std::cmp::max(*max_sum, cur_sum);
            // this is the key part: choose one path returned for parent node
            // let the parent node decide to choose this path(start from current node and go to
            // left or right) or not
            node.val + std::cmp::max(left_sum, right_sum)
        } else {
            0
        }
    }
}

fn main() {}
