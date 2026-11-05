#![allow(dead_code)]

// struct Solution;

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

use std::{cell::RefCell, collections::HashSet, rc::Rc};
struct FindElements {
    vals: HashSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindElements {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut ret = FindElements {
            vals: HashSet::new(),
        };

        ret.dfs(root.as_ref(), 0);

        println!("{:?}", ret.vals);

        ret
    }

    fn dfs(&mut self, node: Option<&Rc<RefCell<TreeNode>>>, cur_val: i32) {
        if let Some(node) = node {
            let node = node.borrow();
            self.vals.insert(cur_val);
            self.dfs(node.left.as_ref(), cur_val * 2);
            self.dfs(node.right.as_ref(), cur_val * 2 + 1);
        }
    }

    fn find(&self, target: i32) -> bool {
        self.vals.contains(&target)
    }
}

/**
 * Your FindElements object will be instantiated and called as such:
 * let obj = FindElements::new(root);
 * let ret_1: bool = obj.find(target);
 */

fn main() {}
