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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ret = vec![];
        if let Some(cur) = root.as_ref() {
            let mut level = vec![cur.clone()];
            while !level.is_empty() {
                let last_val = level.last().unwrap().borrow().val;
                ret.push(last_val);
                let mut new_level = vec![];
                for node in level.into_iter() {
                    let n = node.borrow();
                    if let Some(l) = n.left.as_ref() {
                        new_level.push(l.clone());
                    }
                    if let Some(r) = n.right.as_ref() {
                        new_level.push(r.clone());
                    }
                }
                level = new_level;
            }
        }
        ret
    }
}

fn main() {}
