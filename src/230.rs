struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut stack = Vec::new();

        let mut p = root.clone();
        while let Some(node) = p.clone() {
            stack.push(node.clone());
            p = node.as_ref().borrow().left.clone();
        }

        let mut k = k;
        println!("here");
        while !stack.is_empty() {
            let cur = stack.pop().unwrap();
            k -= 1;
            if k == 0 {
                return cur.as_ref().borrow().val;
            }
            let mut cur = cur.as_ref().borrow().right.clone();
            while let Some(node) = cur.clone() {
                stack.push(node.clone());
                cur = node.as_ref().borrow().left.clone();
            }
        }
        panic!("not to be here");
    }
}

fn main() {}
