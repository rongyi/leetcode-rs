
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

use std::{cell::RefCell, rc::Rc};
struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut ret = BSTIterator { stack: Vec::new() };
        ret.inorder(root.clone());

        ret
    }

    fn next(&mut self) -> i32 {
        let cur = self.stack.pop().unwrap();
        let v = cur.borrow().val;
        self.inorder(cur.borrow().right.clone());
        v
    }

    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }

    fn inorder(&mut self, mut node: Option<Rc<RefCell<TreeNode>>>) {
        while let Some(inner) = node {
            self.stack.push(inner.clone());
            node = inner.borrow().left.clone();
        }
    }
}

/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */
fn main() {}
