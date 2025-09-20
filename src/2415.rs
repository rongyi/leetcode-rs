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
    pub fn reverse_odd_levels(
        mut root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut output: Vec<Vec<i32>> = vec![];
        Self::level_tranverse(root.as_ref(), &mut output, 1);
        for (i, l) in output.iter_mut().enumerate() {
            if i % 2 == 1 {
                l.reverse();
            }
        }
        let mut all_nodes: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
        for lst in output.iter() {
            for &val in lst.iter() {
                let cur = Rc::new(RefCell::new(TreeNode::new(val)));
                all_nodes.push(Some(cur));
            }
        }
        for i in 0..all_nodes.len() / 2 {
            all_nodes[i].as_mut().unwrap().borrow_mut().left = all_nodes[2 * i + 1].clone();
            all_nodes[i].as_mut().unwrap().borrow_mut().right = all_nodes[2 * i + 2].clone();
        }

        all_nodes[0].clone()
    }
    fn level_tranverse(
        node: Option<&Rc<RefCell<TreeNode>>>,
        output: &mut Vec<Vec<i32>>,
        height: usize,
    ) {
        if height > output.len() {
            output.push(vec![]);
        }
        if let Some(node) = node {
            let node = node.borrow();
            output[height - 1].push(node.val);
            Self::level_tranverse(node.left.as_ref(), output, height + 1);
            Self::level_tranverse(node.right.as_ref(), output, height + 1);
        }
    }
}

fn main() {}
