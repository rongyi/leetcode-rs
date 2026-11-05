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
    pub fn get_directions(
        root: Option<Rc<RefCell<TreeNode>>>,
        start_value: i32,
        dest_value: i32,
    ) -> String {
        let mut to_start: Vec<char> = Vec::new();
        let mut to_end: Vec<char> = Vec::new();
        Self::dfs(root.as_ref(), &mut to_start, start_value);
        Self::dfs(root.as_ref(), &mut to_end, dest_value);
        // to reach the lca
        while !to_start.is_empty()
            && !to_end.is_empty()
            && *to_start.last().unwrap() == *to_end.last().unwrap()
        {
            to_start.pop();
            to_end.pop();
        }
        to_end.reverse();

        vec!['U'; to_start.len()].into_iter().collect::<String>()
            + &to_end.into_iter().collect::<String>()
    }

    fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, path: &mut Vec<char>, target: i32) -> bool {
        match root {
            Some(node) => {
                let node = node.borrow();
                if node.val == target {
                    return true;
                }
                // not match current node, ok, try left?
                if node.left.is_some() && Self::dfs(node.left.as_ref(), path, target) {
                    path.push('L');
                } else if node.right.is_some() && Self::dfs(node.right.as_ref(), path, target) {
                    path.push('R');
                }

                return !path.is_empty();
            }
            None => !path.is_empty(),
        }
    }
}

fn main() {}
