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
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut hunt_cur = 0;
        let mut keep_cur = 0;
        let r = root.as_ref();

        fn dorob(node: Option<&Rc<RefCell<TreeNode>>>, hunt_cur: &mut i32, keep_cur: &mut i32) {
            if node.is_none() {
                return;
            }
            let mut hunt_left = 0;
            let mut keep_left = 0;
            let mut hunt_right = 0;
            let mut keep_right = 0;

            dorob(
                node.unwrap().borrow().left.as_ref(),
                &mut hunt_left,
                &mut keep_left,
            );
            dorob(
                node.unwrap().borrow().right.as_ref(),
                &mut hunt_right,
                &mut keep_right,
            );
            // if i rob cur, then child can not be touched
            *hunt_cur = node.unwrap().borrow().val + keep_left + keep_right;

            // dont rob cur, then we can choose whatever action on child
            *keep_cur = hunt_left.max(keep_left) + hunt_right.max(keep_right);
        }
        dorob(root.as_ref(), &mut hunt_cur, &mut keep_cur);

        hunt_cur.max(keep_cur)
    }
}

fn main() {}
