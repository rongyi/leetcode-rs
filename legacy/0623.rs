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
    pub fn add_one_row(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
        depth: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if depth == 1 {
            let ret = Rc::new(RefCell::new(TreeNode::new(val)));
            ret.borrow_mut().left = root;
            return Some(ret);
        }

        Self::do_add(root.clone(), 1, val, depth);

        return root;
    }
    fn do_add(
        mut node: Option<Rc<RefCell<TreeNode>>>,
        cur_height: i32,
        val: i32,
        expect_height: i32,
    ) {
        // not yet, keep going
        if cur_height + 1 < expect_height {
            if let Some(inner) = node {
                let node = inner.borrow();
                Self::do_add(node.left.clone(), cur_height + 1, val, expect_height);
                Self::do_add(node.right.clone(), cur_height + 1, val, expect_height);
            }
        } else if cur_height + 1 == expect_height {
            // insert action
            if let Some(inner) = node {
                let mut n = inner.borrow_mut();
                let origin_left = n.left.take();
                let origin_right = n.right.take();

                let mut newnode = TreeNode::new(val);
                newnode.left = origin_left.clone();
                n.left = Some(Rc::new(RefCell::new(newnode)));

                // make them different
                let mut newnode = TreeNode::new(val);
                newnode.right = origin_right.clone();
                n.right = Some(Rc::new(RefCell::new(newnode)));
            }
        }
    }
}

fn main() {}
