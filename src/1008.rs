#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// LeetCode 1008: Construct Binary Search Tree from Preorder Traversal
    ///
    /// Single pass with an upper bound.
    ///
    /// In preorder: root, then left subtree (all < root), then right subtree
    /// (all > root). So we walk the array with a moving index, and for each
    /// node the `bound` is the largest value allowed in its subtree. When the
    /// next value exceeds `bound`, this subtree is done — return None and let
    /// the caller (which has a larger bound) take over.
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build(&preorder)
    }

    fn build(preorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            return None;
        }
        let cur_val = preorder[0];
        let node = Rc::new(RefCell::new(TreeNode::new(cur_val)));
        let sz = preorder.len();
        let mut j = 1;
        while j < sz && preorder[j] < cur_val {
            j += 1;
        }

        // Left subtree: all values must be < val.
        node.borrow_mut().left = Self::build(&preorder[1..j]);
        // Right subtree: still bounded by our own inherited bound.
        node.borrow_mut().right = Self::build(&preorder[j..]);

        Some(node)
    }
}

fn main() {
    // inorder should be sorted ascending for a valid BST.
    fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, out: &mut Vec<i32>) {
        if let Some(n) = node {
            let n = n.borrow();
            inorder(&n.left, out);
            out.push(n.val);
            inorder(&n.right, out);
        }
    }

    let tests = [
        (vec![8, 5, 1, 7, 10, 12], vec![1, 5, 7, 8, 10, 12]),
        (vec![1, 3], vec![1, 3]),
        (vec![4, 2, 1, 3, 6, 5, 7], vec![1, 2, 3, 4, 5, 6, 7]),
    ];

    for (preorder, expected_inorder) in &tests {
        let root = Solution::bst_from_preorder(preorder.clone());
        let mut got = Vec::new();
        inorder(&root, &mut got);
        println!(
            "{} preorder={:?} → inorder={:?} (expected {:?})",
            if got == *expected_inorder {
                "✓"
            } else {
                "✗"
            },
            preorder,
            got,
            expected_inorder
        );
    }
}
