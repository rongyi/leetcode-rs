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
    pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut nodes: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let s: Vec<char> = traversal.chars().collect();
        let sz = s.len();
        let mut i = 0;
        while i < sz {
            // level indicator
            let mut levels = 0;
            while i < sz && s[i] == '-' {
                i += 1;
                levels += 1;
            }
            // the node val
            let mut j = i;
            while j < sz && s[j].is_ascii_digit() {
                j += 1;
            }
            let val = s[i..j].iter().copied().collect::<String>();
            let val = val.parse::<i32>().unwrap();
            let new_node = Rc::new(RefCell::new(TreeNode::new(val)));

            while nodes.len() > levels {
                nodes.pop();
            }

            if let Some(top) = nodes.last_mut() {
                let mut node = top.borrow_mut();
                if node.left.is_none() {
                    node.left = Some(new_node.clone());
                } else {
                    node.right = Some(new_node.clone());
                }
            }

            nodes.push(new_node.clone());

            i = j;
        }

        while nodes.len() > 1 {
            nodes.pop();
        }

        nodes.pop()
    }
}

fn main() {}
