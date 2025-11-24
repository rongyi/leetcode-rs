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
    pub fn replace_value_in_tree(
        mut root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(root) => {
                let mut level_sum = 0;
                let mut q = vec![(root.clone(), 0)];
                while !q.is_empty() {
                    let mut new_layer = vec![];
                    let mut new_layer_sum = 0;
                    for (node, subtree_sum) in q.into_iter() {
                        node.borrow_mut().val = level_sum - subtree_sum;
                        let mut child = vec![];
                        let mut cur_subtree_sum = 0;
                        if let Some(l) = node.borrow().left.clone() {
                            cur_subtree_sum += l.borrow().val;
                            child.push(l);
                        }
                        if let Some(r) = node.borrow().right.clone() {
                            cur_subtree_sum += r.borrow().val;
                            child.push(r);
                        }
                        child
                            .into_iter()
                            .for_each(|node| new_layer.push((node, cur_subtree_sum)));
                        new_layer_sum += cur_subtree_sum;
                    }
                    level_sum = new_layer_sum;
                    q = new_layer;
                }

                Some(root.clone())
            }
        }
    }
}

fn main() {}
