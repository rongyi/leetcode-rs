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
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut out = vec![];
        Self::action(root.as_ref(), 1, true, &mut out);

        out.into_iter().map(|vd| vd.into_iter().collect()).collect()
    }
    fn action(
        root: Option<&Rc<RefCell<TreeNode>>>,
        height: usize,
        to_left: bool,
        out: &mut Vec<VecDeque<i32>>,
    ) {
        if let Some(cur) = root {
            let cur = cur.borrow();

            if out.len() < height {
                out.push(VecDeque::new());
            }

            if to_left {
                out[height - 1].push_back(cur.val);
            } else {
                out[height - 1].push_front(cur.val);
            }
            Self::action(cur.left.as_ref(), height + 1, !to_left, out);
            Self::action(cur.right.as_ref(), height + 1, !to_left, out);
        }
    }
}

fn main() {}
