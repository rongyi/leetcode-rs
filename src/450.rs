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
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root.clone() {
            Some(node) => {
                let mut n = node.borrow_mut();
                let val = n.val;
                if key < val {
                    n.left = Self::delete_node(n.left.take(), key);
                } else if key > val {
                    n.right = Self::delete_node(n.right.take(), key);
                } else {
                    // equal delete this one
                    // only one node
                    if n.left.is_none() {
                        return n.right.take();
                    } else if n.right.is_none() {
                        return n.left.take();
                    } else {
                        // find next val in inorder and replace this
                        let next_val = Self::find_next_val_in_inorder(n.right.clone());
                        n.val = next_val;
                        n.right = Self::delete_node(n.right.take(), next_val);
                    }
                }

                root
            }
            None => None,
        }
    }

    fn find_next_val_in_inorder(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut p = node.clone();

        while let Some(cur) = p {
            let cur = cur.borrow();
            match cur.left.as_ref() {
                Some(_next) => p = cur.left.clone(),
                None => return cur.val,
            }
        }
        unreachable!()
    }
}

fn main() {}
