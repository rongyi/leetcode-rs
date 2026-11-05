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
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        Self::gen_all(1, n)
    }

    // [left..=right]
    fn gen_all(left: i32, right: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if left > right {
            return vec![None];
        }
        let mut ret = vec![];
        for r in left..=right {
            let left_child = Self::gen_all(left, r - 1);
            let right_child = Self::gen_all(r + 1, right);

            for lc in left_child.iter() {
                for rc in right_child.iter() {
                    let root = Rc::new(RefCell::new(TreeNode::new(r)));
                    root.borrow_mut().left = lc.clone();
                    root.borrow_mut().right = rc.clone();
                    ret.push(Some(root));
                }
            }
        }

        ret
    }
}

fn main() {}
