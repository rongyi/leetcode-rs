struct Solution;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

/// LeetCode 662: Maximum Width of Binary Tree
///
/// BFS level by level, assigning each node a heap index:
///   left = 2·idx, right = 2·idx+1
/// Width of a level = rightmost_idx - leftmost_idx + 1.
/// Indices are re-normalized to the level's leftmost node (`idx - l`)
/// to avoid overflow with deep trees.
impl Solution {
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut q = VecDeque::new();
        q.push_back((root.clone(), 0i64));
        let mut max_width: i64 = 0;

        while !q.is_empty() {
            let level_len = q.len();
            let left = q.front().unwrap().1; // leftmost index this level
            let mut right = 0;

            for _ in 0..level_len {
                let (node, idx) = q.pop_front().unwrap();
                if let Some(node) = node {
                    let node = node.borrow();
                    if node.left.is_some() {
                        q.push_back((node.left.clone(), (idx - left) * 2));
                    }
                    if node.right.is_some() {
                        q.push_back((node.right.clone(), (idx - left) * 2 + 1));
                    }
                }
                right = right.max(idx);
            }

            max_width = max_width.max(right - left + 1);
        }

        max_width as i32
    }
}

fn main() {
    // [1,3,2,5,3,null,9] → 4
    let n3a = Rc::new(RefCell::new(TreeNode::new(3)));
    let n5 = Rc::new(RefCell::new(TreeNode::new(5)));
    let n3b = Rc::new(RefCell::new(TreeNode::new(3)));
    let n2 = Rc::new(RefCell::new(TreeNode::new(2)));
    let n9 = Rc::new(RefCell::new(TreeNode::new(9)));
    n3a.borrow_mut().left = Some(n5.clone());
    n2.borrow_mut().right = Some(n9.clone());
    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    root.borrow_mut().left = Some(n3a.clone());
    root.borrow_mut().right = Some(n2.clone());

    let result = Solution::width_of_binary_tree(Some(root));
    println!(
        "{} width = {} (expected 4)",
        if result == 4 { "✓" } else { "✗" },
        result
    );

    // Single node → 1
    let single = Rc::new(RefCell::new(TreeNode::new(1)));
    let result = Solution::width_of_binary_tree(Some(single));
    println!(
        "{} width = {} (expected 1)",
        if result == 1 { "✓" } else { "✗" },
        result
    );
}
