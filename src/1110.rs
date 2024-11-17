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
use std::collections::HashSet;
use std::rc::Rc;
impl Solution {
    pub fn del_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        to_delete: Vec<i32>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut ret = Vec::new();
        let to_deletes: HashSet<i32> = to_delete.into_iter().collect();

        Self::delete_and_collect(root.clone(), &to_deletes, &mut ret, true);

        ret
    }

    fn delete_and_collect(
        root: Option<Rc<RefCell<TreeNode>>>,
        target_vals: &HashSet<i32>,
        acc: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        should_add_acc: bool,
    ) {
        if let Some(node) = root.clone() {
            let val = node.borrow().val;
            // ok delete this node
            if target_vals.contains(&val) {
                let l = node.borrow_mut().left.take();
                let r = node.borrow_mut().right.take();
                Self::delete_and_collect(l, target_vals, acc, true);
                Self::delete_and_collect(r, target_vals, acc, true);
            } else {
                if should_add_acc {
                    acc.push(root.clone());
                }
                if node.borrow().left.is_some() {
                    let lval = node.borrow().left.as_ref().unwrap().borrow().val;
                    // chop l and add
                    if target_vals.contains(&lval) {
                        Self::delete_and_collect(
                            node.borrow_mut().left.take(),
                            target_vals,
                            acc,
                            true,
                        );
                    } else {
                        // just chop, dont add to acc
                        Self::delete_and_collect(
                            node.borrow().left.clone(),
                            target_vals,
                            acc,
                            false,
                        );
                    }
                }

                if node.borrow().right.is_some() {
                    let rval = node.borrow().right.as_ref().unwrap().borrow().val;
                    // chop r and add
                    if target_vals.contains(&rval) {
                        Self::delete_and_collect(
                            node.borrow_mut().right.take(),
                            target_vals,
                            acc,
                            true,
                        );
                    } else {
                        // just chop dont add to acc
                        Self::delete_and_collect(
                            node.borrow().right.clone(),
                            target_vals,
                            acc,
                            false,
                        );
                    }
                }
            }
        }
    }
}
fn main() {}
