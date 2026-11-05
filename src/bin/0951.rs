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
    pub fn flip_equiv(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Self::do_cmp(root1.as_ref(), root2.as_ref())
    }

    fn do_cmp(
        root1: Option<&Rc<RefCell<TreeNode>>>,
        root2: Option<&Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (root1, root2) {
            (None, None) => true,
            (Some(n1), Some(n2)) => {
                let n1 = n1.borrow();
                let n2 = n2.borrow();
                n1.val == n2.val
                    && ((Self::do_cmp(n1.left.as_ref(), n2.left.as_ref())
                        && Self::do_cmp(n1.right.as_ref(), n2.right.as_ref()))
                        || (Self::do_cmp(n1.left.as_ref(), n2.right.as_ref())
                            && Self::do_cmp(n1.right.as_ref(), n2.left.as_ref())))
            }

            _ => false,
        }
    }
}
fn main() {}
