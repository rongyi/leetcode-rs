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
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let n = match root {
            None => return vec![],
            Some(n) => n,
        };
        let mut q = VecDeque::new();
        let v = n.borrow().val;
        q.push_back((n, vec![v]));
        let mut out = vec![];
        while let Some((cur, path_vec)) = q.pop_front() {
            let cur = cur.borrow();
            if cur.left.is_none()
                && cur.right.is_none()
                && path_vec.iter().sum::<i32>() == target_sum
            {
                out.push(path_vec.clone());
            }
            if let Some(l) = cur.left.as_ref()  {
                let mut new_path = path_vec.clone();
                new_path.push(l.borrow().val);
                q.push_back((l.clone(), new_path));
            }
            if let Some(r) = cur.right.as_ref() {
                let mut new_path = path_vec.clone();
                new_path.push(r.borrow().val);
                q.push_back((r.clone(), new_path));
            }
        }

        out
    }
}
fn main() {}
