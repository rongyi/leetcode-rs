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
    pub fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut out: Vec<Vec<i32>> = Vec::new();
        Self::level_order(root.as_ref(), &mut out, 1);

        let mut ret = 0;
        for l in out.iter_mut() {
            let mut ids: Vec<usize> = (0..l.len()).collect();
            ids.sort_by_key(|&i| l[i]);

            for i in 0..ids.len() {
                while ids[i] != i {
                    let j = ids[i];
                    ids.swap(i, j);
                    ret += 1;
                }
            }
        }

        ret
    }

    fn level_order(
        node: Option<&Rc<RefCell<TreeNode>>>,
        levels: &mut Vec<Vec<i32>>,
        height: usize,
    ) {
        if let Some(n) = node {
            if levels.len() < height {
                levels.push(vec![]);
            }
            let n = n.borrow();
            levels[height - 1].push(n.val);
            Self::level_order(n.left.as_ref(), levels, height + 1);
            Self::level_order(n.right.as_ref(), levels, height + 1);
        }
    }
}

fn main() {}
