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
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.clone() {
            let mut node = node.borrow_mut();
            if key < node.val {
                node.left = Self::delete_node(node.left.take(), key);
            } else if key > node.val {
                node.right = Self::delete_node(node.right.take(), key);
            } else {
                if node.left.is_none() {
                    return node.right.take();
                } else if node.right.is_none() {
                    return node.left.take();
                } else {
                    let successor = Self::find_successor(node.right.clone());
                    node.val = successor.borrow().val;
                    node.right = Self::delete_node(node.right.take(), node.val);
                }
            }
        }
        root
    }

    fn find_successor(root: Option<Rc<RefCell<TreeNode>>>) -> Rc<RefCell<TreeNode>> {
        let mut cur = root;
        // this ref
        while let Some(ref node) = cur.clone() {
            if node.borrow().left.is_none() {
                break;
            }
            cur = node.borrow().left.clone()
        }

        cur.clone().unwrap()
    }
}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));
    let key = 3;
    let result = Solution::delete_node(root, key);
    println!("Result: {:?}", result);
}
