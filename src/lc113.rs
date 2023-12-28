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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut all_paths = Vec::new();
        let mut cur_path = Vec::new();

        Self::recursive(&root, target_sum, &mut all_paths, &mut cur_path, 0);

        all_paths
    }

    fn recursive(
        root: &Option<Rc<RefCell<TreeNode>>>,
        target_sum: i32,
        all_paths: &mut Vec<Vec<i32>>,
        cur_path: &mut Vec<i32>,
        prefix_sum: i32,
    ) {
        if let Some(node) = root {
            cur_path.push(node.borrow().val);

            let cur_prefix = prefix_sum + node.borrow().val;
            let l = &node.borrow().left;
            let r = &node.borrow().right;
            if l.is_none() && r.is_none() && cur_prefix == target_sum {
                all_paths.push(cur_path.clone());
            }
            Self::recursive(l, target_sum, all_paths, cur_path, cur_prefix);
            Self::recursive(r, target_sum, all_paths, cur_path, cur_prefix);

            cur_path.pop();
        }
    }
}

