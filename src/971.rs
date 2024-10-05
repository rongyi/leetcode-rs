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
    pub fn flip_match_voyage(root: Option<Rc<RefCell<TreeNode>>>, voyage: Vec<i32>) -> Vec<i32> {
        let mut flips = Vec::new();
        let mut index = 0;
        if Self::dfs(root.as_ref(), &voyage, &mut index, &mut flips) {
            flips
        } else {
            vec![-1]
        }
    }
    fn dfs(
        root: Option<&Rc<RefCell<TreeNode>>>,
        voyage: &Vec<i32>,
        index: &mut usize,
        flips: &mut Vec<i32>,
    ) -> bool {
        if let Some(node) = root {
            let node = node.borrow();
            if *index >= voyage.len() || node.val != voyage[*index] {
                return false;
            }
            *index += 1;

            if node.left.is_some()
                && node.left.as_ref().unwrap().borrow().val
                    != voyage.get(*index).copied().unwrap_or(-1)
            {
                flips.push(node.val);
                Self::dfs(node.right.as_ref(), voyage, index, flips)
                    && Self::dfs(node.left.as_ref(), voyage, index, flips)
            } else {
                Self::dfs(node.left.as_ref(), voyage, index, flips)
                    && Self::dfs(node.right.as_ref(), voyage, index, flips)
            }
        } else {
            true
        }
    }
}
fn main() {}
