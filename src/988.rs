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
    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut ret = String::new();
        Self::dfs(root.as_ref(), &mut Vec::new(), &mut ret);
        ret
    }
    fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, path: &mut Vec<char>, ret: &mut String) {
        if let Some(node) = node {
            let node = node.borrow();
            path.push((b'a' + node.val as u8) as char);
            if node.left.is_none() && node.right.is_none() {
                let cur: String = path.iter().rev().collect();
                if ret.is_empty() || cur < *ret {
                    *ret = cur;
                }
            } else {
                Self::dfs(node.left.as_ref(), path, ret);
                Self::dfs(node.right.as_ref(), path, ret);
            }

            path.pop();
        }
    }
}

fn main() {}
