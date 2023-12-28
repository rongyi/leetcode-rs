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
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut leaf_path = i32::MAX;
        Self::do_depth(&root, &mut leaf_path, 1);
        if leaf_path == i32::MAX {
            return 0;
        }

        leaf_path
    }

    fn do_depth(root: &Option<Rc<RefCell<TreeNode>>>, leaf_path: &mut i32, cur_height: i32) {
        if let Some(node) = root {
            if node.borrow().left.is_none() && node.borrow().right.is_none() {
                *leaf_path = i32::min(*leaf_path, cur_height);
            }
            let l = &node.borrow().left;
            let r = &node.borrow().right;
            Self::do_depth(l, leaf_path, cur_height + 1);
            Self::do_depth(r, leaf_path, cur_height + 1);
        }
    }
}

