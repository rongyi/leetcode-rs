use std::cell::RefCell;
use std::rc::Rc;

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

struct CBTInserter {
    root: Option<Rc<RefCell<TreeNode>>>,
    nodes: Vec<Rc<RefCell<TreeNode>>>,
}

impl CBTInserter {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut nodes = Vec::new();
        if let Some(r) = root.as_ref() {
            nodes.push(r.clone());
            let mut i = 0;
            while i < nodes.len() {
                let left = nodes[i].borrow().left.clone();
                let right = nodes[i].borrow().right.clone();
                if let Some(left) = left {
                    nodes.push(left);
                }
                if let Some(right) = right {
                    nodes.push(right);
                }
                i += 1;
            }
        }

        Self { root, nodes }
    }

    fn insert(&mut self, val: i32) -> i32 {
        let new_node = Rc::new(RefCell::new(TreeNode::new(val)));
        let parent_idx = (self.nodes.len() - 1) / 2;
        let parent = &self.nodes[parent_idx];
        let parent_val = parent.borrow().val;

        // perfect sum is 2^n - 1 must be odd number
        if self.nodes.len() % 2 == 1 {
            parent.borrow_mut().left = Some(new_node.clone());
        } else {
            parent.borrow_mut().right = Some(new_node.clone());
        }

        self.nodes.push(new_node);

        parent_val
    }

    fn get_root(&self) -> Option<Rc<RefCell<TreeNode>>> {
        self.root.clone()
    }
}

fn main() {}
