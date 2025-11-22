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
    pub fn replace_value_in_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let root = if let Some(root) = root {
            root
        } else {
            return None;
        };
        let mut level_sum = 0;
        let mut q = vec![(root.clone(), 0)];
        while !q.is_empty() {
            let mut new_queue = vec![];
            let mut new_level_sum = 0;
            for (node, subtree_sum) in q.into_iter() {
                node.borrow_mut().val = level_sum - subtree_sum;
                let mut subtree_sum = 0;
                let mut children = vec![];
                if let Some(l) = node.borrow().left.clone() {
                    subtree_sum += l.borrow().val;
                    children.push(l);
                }
                if let Some(r) = node.borrow().right.clone() {
                    subtree_sum += r.borrow().val;
                    children.push(r);
                }
                new_level_sum += subtree_sum;
                children
                    .into_iter()
                    .for_each(|node| new_queue.push((node, subtree_sum)));
            }
            q = new_queue;
            level_sum = new_level_sum;
        }
        Some(root)
    }
}

fn main() {}
