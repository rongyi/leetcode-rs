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
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;

        if Self::dfs(root.as_ref(), &mut sum) == 0 {
            sum += 1;
        }

        sum
    }
    // 尽量把节点往上放，所以身上没有相机的时候，被照顾到应该只能是子节点来照顾
    // 0: there is no camera at this node, and it's not monitored by camera at
    // either of its children, which means neither of child nodes has camera.
    // 1: there is no camera at this node; however, this node is monitored by at
    // least 1 of its children, which means at least 1 of its children has camera.
    // 2: there is a camera at this node.
    // #define NO_CAMERA       0
    // #define HAS_CAMERA      2
    // #define NOT_NEEDED      1
    fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, sum: &mut i32) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            let l = Self::dfs(node.left.as_ref(), sum);
            let r = Self::dfs(node.right.as_ref(), sum);
            if l == 0 || r == 0 {
                *sum += 1;
                return 2;
            } else if l == 2 || r == 2 {
                return 1;
            }
            return 0;
        } else {
            1
        }
    }
}

fn main() {}
