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
struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut ret = String::new();
        Self::serialize_helper(&root, &mut ret);

        ret
    }
    fn serialize_helper(root: &Option<Rc<RefCell<TreeNode>>>, ret: &mut String) {
        if let Some(node) = root {
            let node = node.borrow();
            ret.push_str(&node.val.to_string());
            ret.push(',');
            Self::serialize_helper(&node.left, ret);
            Self::serialize_helper(&node.right, ret);
        } else {
            ret.push_str("null,");
        }
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let nodes: Vec<&str> = data.trim_end_matches(',').split(',').collect();
        let mut iter = nodes.iter().map(|&s| s.parse::<i32>().ok());
        Self::de_helper(&mut iter)
    }

    fn de_helper(iter: &mut impl Iterator<Item = Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(Some(val)) = iter.next() {
            let mut root = TreeNode::new(val);
            root.left = Self::de_helper(iter);
            root.right = Self::de_helper(iter);
            Some(Rc::new(RefCell::new(root)))
        } else {
            None
        }
    }
}

fn main() {}
