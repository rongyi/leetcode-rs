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
    pub fn closest_nodes(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<Vec<i32>> {
        let mut lst = vec![];
        Self::collect_inorder(root.as_ref(), &mut lst);
        let mut ret = vec![];
        for q in queries.into_iter() {
            let idx = lst.partition_point(|&x| x <= q);
            let idx2 = lst.partition_point(|&x| x < q);
            let mut val1 = -1;
            if idx > 0 {
                val1 = lst[idx - 1];
            }
            let mut val2 = -1;
            if idx2 < lst.len() {
                val2 = lst[idx2];
            }
            ret.push(vec![val1, val2]);
        }

        ret
    }

    fn collect_inorder(node: Option<&Rc<RefCell<TreeNode>>>, out: &mut Vec<i32>) {
        if let Some(node) = node {
            let node = node.borrow();
            Self::collect_inorder(node.left.as_ref(), out);
            out.push(node.val);
            Self::collect_inorder(node.right.as_ref(), out);
        }
    }
}

fn main() {}
