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
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build_tree(
            preorder: &Vec<i32>,
            index: &mut usize,
            min: i32,
            max: i32,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if *index >= preorder.len() || preorder[*index] < min || preorder[*index] > max {
                return None;
            }
            let val = preorder[*index];
            *index += 1;
            let node = Rc::new(RefCell::new(TreeNode::new(val)));
            node.borrow_mut().left = build_tree(preorder, index, min, val);
            node.borrow_mut().right = build_tree(preorder, index, val, max);
            Some(node)
        }
        let mut index = 0;

        build_tree(&preorder, &mut index, i32::MIN, i32::MAX)
    }
}

fn main() {}
