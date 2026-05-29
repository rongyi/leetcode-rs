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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut out = vec![];

        Self::visit(root.as_ref(), 1, &mut out);
        out
    }
    fn visit(root: Option<&Rc<RefCell<TreeNode>>>, height: usize, out: &mut Vec<Vec<i32>>) {
        if let Some(cur) = root {
            if out.len() < height {
                out.push(vec![]);
            }
            let cur = cur.borrow();
            out[height - 1].push(cur.val);
            Self::visit(cur.left.as_ref(), height + 1, out);
            Self::visit(cur.right.as_ref(), height + 1, out);
        }
    }
}
fn main() {}
