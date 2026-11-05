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
    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        let mut out: Vec<i64> = vec![];
        Self::level_sum(root.as_ref(), &mut out, 1);
        // out.select_nth_unstable(k as usize)
        out.sort_by_key(|v| -v);
        *out.get(k as usize - 1).unwrap_or(&-1)
    }
    fn level_sum(cur: Option<&Rc<RefCell<TreeNode>>>, level_sum: &mut Vec<i64>, height: usize) {
        if let Some(cur) = cur {
            if level_sum.len() < height {
                level_sum.push(0);
            }
            let cur = cur.borrow();
            level_sum[height - 1] += cur.val as i64;
            Self::level_sum(cur.left.as_ref(), level_sum, height + 1);
            Self::level_sum(cur.right.as_ref(), level_sum, height + 1);
        }
    }
}

fn main() {}
