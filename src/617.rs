struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::do_merge(root1.as_ref(), root2.as_ref())
    }
    fn do_merge(
        node1: Option<&Rc<RefCell<TreeNode>>>,
        node2: Option<&Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (node1, node2) {
            (Some(node1), Some(node2)) => {
                let node1 = node1.borrow();
                let node2 = node2.borrow();
                let mut sum_node = TreeNode::new(node1.val + node2.val);
                sum_node.left = Self::do_merge(node1.left.as_ref(), node2.left.as_ref());
                sum_node.right = Self::do_merge(node1.right.as_ref(), node2.right.as_ref());

                Some(Rc::new(RefCell::new(sum_node)))
            }
            (None, Some(node2)) => {
                let node2 = node2.borrow();
                let mut sum_node = TreeNode::new(node2.val);
                sum_node.left = Self::do_merge(None, node2.left.as_ref());
                sum_node.right = Self::do_merge(None, node2.right.as_ref());

                Some(Rc::new(RefCell::new(sum_node)))
            }
            (Some(node1), None) => {
                let node1 = node1.borrow();
                let mut sum_node = TreeNode::new(node1.val);
                sum_node.left = Self::do_merge(node1.left.as_ref(), None);
                sum_node.right = Self::do_merge(node1.right.as_ref(), None);

                Some(Rc::new(RefCell::new(sum_node)))
            }
            (None, None) => {
                return None;
            }
        }
    }
}

fn main() {}
