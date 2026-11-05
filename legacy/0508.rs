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
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut max_freq = 0;
        let mut cnt: HashMap<i32, i32> = HashMap::new();

        Self::dfs(root.as_ref(), &mut max_freq, &mut cnt);
        cnt.iter()
            .filter(|&(_, &sum_freq)| sum_freq == max_freq)
            .map(|(&sum, _)| sum)
            .collect()
    }
    // return subtree sum
    fn dfs(
        node: Option<&Rc<RefCell<TreeNode>>>,
        max_freq: &mut i32,
        cnt: &mut HashMap<i32, i32>,
    ) -> i32 {
        if let Some(node) = node {
            let node = node.borrow();
            let sum = node.val
                + Self::dfs(node.left.as_ref(), max_freq, cnt)
                + Self::dfs(node.right.as_ref(), max_freq, cnt);
            let cur = cnt.entry(sum).or_insert(0);
            *cur += 1;
            *max_freq = (*max_freq).max(*cur);

            sum
        } else {
            0
        }
    }
}
fn main() {}
