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
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn is_leaf(node: Option<&Rc<RefCell<TreeNode>>>) -> bool {
            if let Some(n) = node {
                let n = n.borrow();
                n.left.is_none() && n.right.is_none()
            } else {
                false
            }
        }

        let mut sum = 0;
        fn sum_left_leaf(node: Option<&Rc<RefCell<TreeNode>>>, is_left: bool, sum: &mut i32) {
            if let Some(inner) = node {
                let n = inner.borrow();
                if is_left && is_leaf(node) {
                    *sum += n.val;
                } else {
                    sum_left_leaf(n.left.as_ref(), true, sum);
                    sum_left_leaf(n.right.as_ref(), false, sum);
                }
            }
        }

        sum_left_leaf(root.as_ref(), false, &mut sum);

        sum
    }
}

fn main() {
    println!("{}", 10i32.pow(2));
}
