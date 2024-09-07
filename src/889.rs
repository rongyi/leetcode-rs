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
    // 前序和后序是不能唯一确定一颗树的，所以答案要求的是any
    pub fn construct_from_pre_post(
        preorder: Vec<i32>,
        postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn build(pre: &[i32], post: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if pre.is_empty() {
                return None;
            }
            let root = Rc::new(RefCell::new(TreeNode::new(pre[0])));
            if pre.len() > 1 {
                let left_root = pre[1];
                let left_size = post.iter().position(|&x| x == left_root).unwrap() + 1;

                root.borrow_mut().left = build(&pre[1..left_size + 1], &post[..left_size]);
                // and strip current node at last in post vec
                root.borrow_mut().right =
                    build(&pre[left_size + 1..], &post[left_size..post.len() - 1]);
            }

            Some(root)
        }

        build(&preorder, &postorder)
    }
}

fn main() {}
