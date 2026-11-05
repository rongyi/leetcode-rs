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
use std::collections::BTreeMap;
use std::rc::Rc;

impl Solution {
    // think grid, a grid may contain multiple values in one cell
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut cache = BTreeMap::new();
        Self::dfs(root.as_ref(), 0, 0, &mut cache);
        cache
            .into_iter()
            .map(|(_, v)| {
                v.into_iter()
                    .flat_map(|(_, mut v)| {
                        // 这是局部的sort，不是全局column的
                        v.sort_unstable();
                        v
                    })
                    .collect()
            })
            .collect()
    }
    fn dfs(
        root: Option<&Rc<RefCell<TreeNode>>>,
        x: i32,
        y: i32,
        cache: &mut BTreeMap<i32, BTreeMap<i32, Vec<i32>>>,
    ) {
        if let Some(node) = root {
            let node = node.borrow();

            cache
                .entry(y)
                .or_insert(BTreeMap::new())
                .entry(x)
                .or_insert(Vec::new())
                .push(node.val);

            Self::dfs(node.left.as_ref(), x + 1, y - 1, cache);
            Self::dfs(node.right.as_ref(), x + 1, y + 1, cache);
        }
    }
}
fn main() {}
