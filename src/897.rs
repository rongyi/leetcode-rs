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
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let dummy = Rc::new(RefCell::new(TreeNode::new(0)));
        let mut cur = dummy.clone();

        Self::inorder_traversal(root, &mut cur);

        let val = dummy.borrow_mut().right.take();

        val
    }
    fn inorder_traversal(node: Option<Rc<RefCell<TreeNode>>>, cur: &mut Rc<RefCell<TreeNode>>) {
        if let Some(n) = node {
            let mut n = n.borrow_mut();
            Self::inorder_traversal(n.left.take(), cur);

            cur.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(n.val))));
            let next = cur.borrow().right.clone().unwrap();
            *cur = next;

            Self::inorder_traversal(n.right.take(), cur);
        }
    }
}

fn main() {}
