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
    pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        // full binary have 2^(n) - 1 node, always odd number
        if n < 0 || n % 2 == 0 {
            return vec![];
        }
        if n == 1 {
            return vec![Some(Rc::new(RefCell::new(TreeNode::new(0))))];
        }
        let mut ret = Vec::new();
        for left_cnt in (1..n).step_by(2) {
            let right_cnt = n - 1 - left_cnt;
            let left_subtrees = Self::all_possible_fbt(left_cnt);
            let right_subtrees = Self::all_possible_fbt(right_cnt);
            for left in left_subtrees.iter() {
                for right in right_subtrees.iter() {
                    let root = Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: left.clone(),
                        right: right.clone(),
                    }));
                    ret.push(Some(root));
                }
            }
        }

        ret
    }
}

fn main() {}
