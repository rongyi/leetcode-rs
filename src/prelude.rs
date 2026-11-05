//! The types LeetCode injects into every Rust snippet but that are not part of
//! `std`. Solutions that need them start with `use lc::prelude::*;`.
//!
//! `Rc` / `RefCell` are re-exported as a convenience because LeetCode's
//! templates also assume they are in scope.

pub use std::cell::RefCell;
pub use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    /// Wrap a value into the `Option<Rc<RefCell<..>>>` form used by the API.
    #[inline]
    pub fn leaf(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(Self::new(val))))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }

    /// Build a list from values, keeping their order.
    pub fn from_vec(vals: impl IntoIterator<Item = i32>) -> Option<Box<ListNode>> {
        let mut vals: Vec<i32> = vals.into_iter().collect();
        vals.reverse();

        let mut head: Option<Box<ListNode>> = None;
        for val in vals {
            head = Some(Box::new(ListNode { val, next: head }));
        }
        head
    }
}
