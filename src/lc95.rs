use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n == 0 {
            return vec![];
        }

        Self::generate_trees_helper(1, n)
    }

    fn generate_trees_helper(start: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut result = Vec::new();

        if start > end {
            result.push(None);
            return result;
        }

        for i in start..=end {
            let left_subtrees = Self::generate_trees_helper(start, i - 1);
            let right_subtrees = Self::generate_trees_helper(i + 1, end);

            for left in &left_subtrees {
                for right in &right_subtrees {
                    let root = Some(Rc::new(RefCell::new(TreeNode::new(i))));
                    root.as_ref().unwrap().borrow_mut().left = left.clone();
                    root.as_ref().unwrap().borrow_mut().right = right.clone();
                    result.push(root);
                }
            }
        }

        result
    }
}
