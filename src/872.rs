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
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut c1 = Vec::new();
        let mut c2 = Vec::new();
        Self::collect(root1.as_ref(), &mut c1);
        Self::collect(root2.as_ref(), &mut c2);

        c1 == c2
    }
    fn collect(root: Option<&Rc<RefCell<TreeNode>>>, out: &mut Vec<i32>) {
        if let Some(node) = root {
            let node = node.borrow();
            if node.left.is_none() && node.right.is_none() {
                out.push(node.val);
            }
            Self::collect(node.left.as_ref(), out);
            Self::collect(node.right.as_ref(), out);
        }
    }
}

fn main() {}
