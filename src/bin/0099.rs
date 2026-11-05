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
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut first = None;
        let mut second = None;
        let mut prev = None;

        Self::inorder_tranverse(root.as_ref(), &mut first, &mut second, &mut prev);

        if let (Some(n1), Some(n2)) = (first, second) {
            let mut n1 = n1.borrow_mut();
            let mut n2 = n2.borrow_mut();
            std::mem::swap(&mut n1.val, &mut n2.val);
        }
    }
    fn inorder_tranverse(
        root: Option<&Rc<RefCell<TreeNode>>>,
        first: &mut Option<Rc<RefCell<TreeNode>>>,
        second: &mut Option<Rc<RefCell<TreeNode>>>,
        prev: &mut Option<Rc<RefCell<TreeNode>>>,
    ) {
        if let Some(cur) = root {
            Self::inorder_tranverse(cur.borrow().left.as_ref(), first, second, prev);

            // visit detail
            if let Some(p) = prev {
                if p.borrow().val > cur.borrow().val {
                    if first.is_none() {
                        *first = Some(p.clone());
                    }

                    *second = Some(cur.clone());
                }
            }
            *prev = Some(cur.clone());
            Self::inorder_tranverse(cur.borrow().right.as_ref(), first, second, prev);
        }
    }
}

fn main() {}
