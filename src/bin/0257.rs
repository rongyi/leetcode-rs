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
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut out = vec![];
        Self::down(root.as_ref(), vec![], &mut out);

        out
    }
    fn down(root: Option<&Rc<RefCell<TreeNode>>>, mut cur_path: Vec<i32>, out: &mut Vec<String>) {
        if let Some(cur) = root {
            let n = cur.borrow();
            cur_path.push(n.val);
            if n.left.is_none() && n.right.is_none() {
                out.push(
                    cur_path
                        .iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>()
                        .join("->"),
                );
            }
            if let Some(l) = n.left.as_ref() {
                Self::down(n.left.as_ref(), cur_path.clone(), out);
            }
            if let Some(r) = n.right.as_ref() {
                Self::down(n.right.as_ref(), cur_path.clone(), out);
            }
        }
    }
}

fn main() {}
