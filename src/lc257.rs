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
        let mut ret: Vec<String> = Vec::new();
        let path: String = String::new();
        fn recur(node: Option<Rc<RefCell<TreeNode>>>, output: &mut Vec<String>, mut path: String) {
            if let Some(n) = node {
                let n = n.borrow();
                let vals = n.val.to_string();
                path.push_str(&vals);
                path.push_str("->");
                if n.left.is_none() && n.right.is_none() {
                    let mut cur = path.clone();
                    cur.pop();
                    cur.pop();
                    output.push(cur);
                }

                recur(n.left.clone(), output, path.clone());
                recur(n.right.clone(), output, path.clone());
            }
        }
        recur(root, &mut ret, path);

        ret
    }
}

fn main() {}
