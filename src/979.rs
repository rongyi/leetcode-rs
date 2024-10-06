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
    pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ret = 0;

        Self::dfs(root.as_ref(), &mut ret);

        ret
    }
    fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, mv: &mut i32) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            let l = Self::dfs(node.left.as_ref(), mv);
            let r = Self::dfs(node.right.as_ref(), mv);
            *mv += l.abs() + r.abs();

            node.val + l + r - 1
        } else {
            0
        }
    }
}
fn main() {}
