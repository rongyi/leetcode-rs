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
    pub fn btree_game_winning_move(root: Option<Rc<RefCell<TreeNode>>>, n: i32, x: i32) -> bool {
        let mut lcount = 0;
        let mut rcount = 0;

        fn count_node(
            root: Option<&Rc<RefCell<TreeNode>>>,
            l: &mut i32,
            r: &mut i32,
            x: i32,
        ) -> i32 {
            if let Some(node) = root {
                let node = node.borrow();
                let subl = count_node(node.left.as_ref(), l, r, x);
                let subr = count_node(node.right.as_ref(), l, r, x);
                if node.val == x {
                    *l = subl;
                    *r = subr;
                }

                subl + subr + 1
            } else {
                0
            }
        }

        count_node(root.as_ref(), &mut lcount, &mut rcount, x);

        lcount > n / 2 || rcount > n / 2 || lcount + rcount < n / 2
    }
}

fn main() {}
