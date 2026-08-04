struct Sollution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::make_node(&nums)
    }

    fn make_node(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }
        if nums.len() == 1 {
            return Some(Rc::new(RefCell::new(TreeNode::new(nums[0]))));
        }
        let mut max_val = i32::MIN;
        let mut max_idx = 0;
        for (i, &v) in nums.iter().enumerate() {
            if v > max_val {
                max_idx = i;
                max_val = v;
            }
        }

        let mut node = TreeNode::new(max_val);
        // all in right
        if max_idx == 0 {
            node.right = Self::make_node(&nums[1..])
        } else if max_idx == nums.len() - 1 {
            node.left = Self::make_node(&nums[0..nums.len() - 1]);
        } else {
            node.left = Self::make_node(&nums[0..max_idx]);
            node.right = Self::make_node(&nums[max_idx + 1..]);
        }

        Some(Rc::new(RefCell::new(node)))
    }
}

fn main() {}
