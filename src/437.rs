struct Solution;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        if let Some(inner) = root.clone() {
            let node = inner.borrow();
            Self::dfs(root.as_ref(), target_sum as i64, 0i64)
                + Self::path_sum(node.left.clone(), target_sum)
                + Self::path_sum(node.right.clone(), target_sum)
        } else {
            0
        }
    }
    fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, target: i64, mut cur: i64) -> i32 {
        if let Some(inner) = root {
            let node = inner.borrow();
            cur += node.val as i64;
            if cur == target {
                return 1
                    + Self::dfs(node.left.as_ref(), target, cur)
                    + Self::dfs(node.right.as_ref(), target, cur);
            }

            Self::dfs(node.left.as_ref(), target, cur) + Self::dfs(node.right.as_ref(), target, cur)
        } else {
            0
        }
    }
}



fn main() {}
