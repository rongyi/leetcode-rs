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
        let mut first: Option<Rc<RefCell<TreeNode>>> = None;
        let mut second: Option<Rc<RefCell<TreeNode>>> = None;
        let mut prev: Option<Rc<RefCell<TreeNode>>> = None;

        fn inorder_traversal(
            node: &Option<Rc<RefCell<TreeNode>>>,
            first: &mut Option<Rc<RefCell<TreeNode>>>,
            second: &mut Option<Rc<RefCell<TreeNode>>>,
            prev: Option<Rc<RefCell<TreeNode>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(inner) = node {
                let innver_borrow = inner.borrow();
                let updated_prev = inorder_traversal(&innver_borrow.left, first, second, prev);

                if let Some(prev_inner) = updated_prev {
                    if prev_inner.borrow().val > innver_borrow.val {
                        if first.is_none() {
                            *first = Some(prev_inner.clone());
                        }
                        *second = Some(inner.clone());
                    }
                }

                inorder_traversal(&innver_borrow.right, first, second, Some(inner.clone()))
            } else {
                prev
            }
        }
        inorder_traversal(root, &mut first, &mut second, None);
        if let (Some(first_node), Some(second_node)) = (first, second) {
            let first_val = first_node.borrow().val;

            first_node.borrow_mut().val = second_node.borrow().val;
            second_node.borrow_mut().val = first_val;
        }
    }
}

