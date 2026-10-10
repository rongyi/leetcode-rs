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
use std::collections::HashSet;
use std::rc::Rc;
impl Solution {
    pub fn del_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        to_delete: Vec<i32>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut ret = Vec::new();
        let to_deletes: HashSet<i32> = to_delete.into_iter().collect();

        Self::dfs(root.clone(), &to_deletes, &mut ret, true);

        ret
    }

    fn dfs(
        root: Option<Rc<RefCell<TreeNode>>>,
        target_vals: &HashSet<i32>,
        acc: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        is_root: bool,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(n) = root.clone() {
            let mut node = n.borrow_mut();
            let should_delete = target_vals.contains(&node.val);
            node.left = Self::dfs(node.left.take(), target_vals, acc, should_delete);
            node.right = Self::dfs(node.right.take(), target_vals, acc, should_delete);

            if should_delete {
                return None;
            } else {
                if is_root {
                    acc.push(root.clone());
                }
                return root;
            }
        } else {
            None
        }
    }
}
fn main() {}
