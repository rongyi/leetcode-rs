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
    pub fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<String>> {
        let h = Self::height(root.as_ref());
        let w = 2i32.pow(h as u32) - 1;
        let mut ret = vec![vec!["".to_string(); w as usize]; h as usize];

        Self::recur(root.as_ref(), &mut ret, 0, 0, w - 1);

        ret
    }
    fn height(root: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            let l = Self::height(node.left.as_ref());
            let r = Self::height(node.right.as_ref());
            l.max(r) + 1
        } else {
            0
        }
    }
    fn recur(
        root: Option<&Rc<RefCell<TreeNode>>>,
        ret: &mut Vec<Vec<String>>,
        level: usize,
        l: i32,
        r: i32,
    ) {
        if let Some(node) = root {
            let node = node.borrow();
            let mid = l + (r - l) / 2;
            ret[level][mid as usize] = node.val.to_string();
            Self::recur(node.left.as_ref(), ret, level + 1, l, mid - 1);
            Self::recur(node.right.as_ref(), ret, level + 1, mid + 1, r);
        }
    }
}
fn main() {}
