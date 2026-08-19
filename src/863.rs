struct Solution;

// Definition for a binary tree node
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
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
impl Solution {
    pub fn distance_k(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: Option<Rc<RefCell<TreeNode>>>,
        k: i32,
    ) -> Vec<i32> {
        if k == 0 {
            // If K is 0, just return the target node's value
            return vec![target.as_ref().unwrap().borrow().val];
        }

        // Build parent map
        let mut parent_map: HashMap<i32, Rc<RefCell<TreeNode>>> = HashMap::new();
        Self::build_parent_map(root.as_ref(), None, &mut parent_map);

        // BFS from target node
        let target_val = target.as_ref().unwrap().borrow().val;
        let mut result = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut queue = VecDeque::new();

        // Start BFS from target node
        queue.push_back((target.unwrap(), 0));
        visited.insert(target_val);

        while let Some((node, dist)) = queue.pop_front() {
            if dist == k {
                result.push(node.borrow().val);
                continue;
            }

            let node_ref = node.borrow();
            let current_val = node_ref.val;

            // Check left child
            if let Some(left) = node_ref.left.as_ref() {
                let left_val = left.borrow().val;
                if !visited.contains(&left_val) {
                    visited.insert(left_val);
                    queue.push_back((left.clone(), dist + 1));
                }
            }

            // Check right child
            if let Some(right) = node_ref.right.as_ref() {
                let right_val = right.borrow().val;
                if !visited.contains(&right_val) {
                    visited.insert(right_val);
                    queue.push_back((right.clone(), dist + 1));
                }
            }

            // Check parent
            if let Some(parent) = parent_map.get(&current_val) {
                let parent_val = parent.borrow().val;
                if !visited.contains(&parent_val) {
                    visited.insert(parent_val);
                    queue.push_back((parent.clone(), dist + 1));
                }
            }
        }

        result
    }

    fn build_parent_map(
        node: Option<&Rc<RefCell<TreeNode>>>,
        parent: Option<Rc<RefCell<TreeNode>>>,
        parent_map: &mut HashMap<i32, Rc<RefCell<TreeNode>>>,
    ) {
        if let Some(n) = node {
            if let Some(p) = parent {
                parent_map.insert(n.borrow().val, p);
            }

            let n_ref = n.borrow();
            Self::build_parent_map(n_ref.left.as_ref(), Some(n.clone()), parent_map);
            Self::build_parent_map(n_ref.right.as_ref(), Some(n.clone()), parent_map);
        }
    }
}

fn main() {}
