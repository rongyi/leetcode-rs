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
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        fn helper(root: &Option<Rc<RefCell<TreeNode>>>, ret: &mut i32, cur_sum: i32) {
            if let Some(node) = root {
                let node = node.borrow();
                let cur_sum = cur_sum * 10 + node.val;

                if node.left.is_none() && node.right.is_none() {
                    *ret += cur_sum;
                }
                helper(&node.left, ret, cur_sum);
                helper(&node.right, ret, cur_sum);
            }
        }

        helper(&root, &mut sum, 0);
        sum
    }
}

fn main() {}
