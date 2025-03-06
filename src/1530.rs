#![allow(dead_code)]

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
    pub fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
        let mut result = 0;
        Self::dfs(root.as_ref(), distance, &mut result);
        result
    }

    fn dfs(
        node: Option<&Rc<RefCell<TreeNode>>>,
        distance: i32,
        result: &mut i32,
    ) -> HashMap<i32, i32> {
        match node {
            None => HashMap::new(),
            Some(rc) => {
                let node = rc.borrow();

                // Leaf node case
                if node.left.is_none() && node.right.is_none() {
                    let mut map = HashMap::new();
                    map.insert(1, 1);
                    return map;
                }

                // Process subtrees
                let left_map = Self::dfs(node.left.as_ref().map(|rc| rc), distance, result);
                let right_map = Self::dfs(node.right.as_ref().map(|rc| rc), distance, result);

                // Count good pairs
                for (&l_dist, &l_count) in &left_map {
                    for (&r_dist, &r_count) in &right_map {
                        if l_dist + r_dist <= distance {
                            *result += l_count * r_count;
                        }
                    }
                }

                // Prepare result map
                let mut current_map = HashMap::new();

                // Helper function to add leaves with increased distance
                let add_to_map = |map: &mut HashMap<i32, i32>, source: &HashMap<i32, i32>| {
                    for (&dist, &count) in source {
                        if dist + 1 <= distance {
                            *map.entry(dist + 1).or_insert(0) += count;
                        }
                    }
                };

                // Add leaves from both subtrees
                add_to_map(&mut current_map, &left_map);
                add_to_map(&mut current_map, &right_map);

                current_map
            }
        }
    }
}
fn main() {}
