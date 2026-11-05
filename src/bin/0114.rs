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

// Definition for a binary tree node.
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(cur) = root {
            let mut cur = cur.borrow_mut();
            Self::flatten(&mut cur.left);
            Self::flatten(&mut cur.right);
            let mut left = cur.left.take();
            let mut right = cur.right.take();
            match left.as_ref() {
                Some(l) => {
                    cur.right = Some(l.clone());
                    let mut right_most = l.clone();
                    while right_most.borrow().right.is_some() {
                        let tmp = right_most.borrow().right.as_ref().unwrap().clone();
                        right_most = tmp;
                    }
                    right_most.borrow_mut().right = right;
                }
                None => {
                    cur.right = right;
                }
            }
        }
    }
}

fn main() {}
