#![allow(dead_code)]

struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
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
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return false;
        }
        Self::check(head.clone(), root.clone())
            || Self::is_sub_path(head.clone(), root.clone().unwrap().borrow().left.clone())
            || Self::is_sub_path(head, root.clone().unwrap().borrow().right.clone())
    }
    fn check(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if head.is_none() {
            return true;
        }
        if root.is_none() {
            return false;
        }
        head.clone().unwrap().val == root.clone().unwrap().borrow().val
            && (Self::check(
                head.clone().unwrap().next.clone(),
                root.clone().unwrap().borrow().left.clone(),
            ) || Self::check(
                head.unwrap().clone().next.clone(),
                root.unwrap().borrow().right.clone(),
            ))
    }
}

fn main() {}
