
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
        let mut ret = Self { stack: vec![] };

        ret.visit_to_left_end(root);

        ret
    }

    fn next(&mut self) -> i32 {
        let cur = self.stack.pop().unwrap();
        let val = cur.borrow().val;

        self.visit_to_left_end(cur.borrow().right.clone());

        val
    }

    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }

    fn visit_to_left_end(&mut self, node: Option<Rc<RefCell<TreeNode>>>) {
        let mut cur = node;
        while let Some(n) = cur {
            self.stack.push(n.clone());
            cur = n.borrow().left.clone();
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
