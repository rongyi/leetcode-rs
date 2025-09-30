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
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    pub fn tree_queries(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<i32> {
        // val's left child height, not includ it self
        let mut left_heights: HashMap<i32, i32> = HashMap::new();
        // val's right child height, not include it self
        let mut right_heights: HashMap<i32, i32> = HashMap::new();
        Self::edges(root.as_ref(), &mut left_heights, &mut right_heights);

        let mut the_other_road: HashMap<i32, i32> = HashMap::new();
        Self::peek_other_side(
            root.as_ref().unwrap().borrow().left.as_ref(),
            right_heights[&root.as_ref().unwrap().borrow().val],
            1,
            &mut the_other_road,
            &left_heights,
            &right_heights,
        );
        Self::peek_other_side(
            root.as_ref().unwrap().borrow().right.as_ref(),
            left_heights[&root.as_ref().unwrap().borrow().val],
            1,
            &mut the_other_road,
            &left_heights,
            &right_heights,
        );
        let mut ret = vec![];
        for q in &queries {
            ret.push(the_other_road[q]);
        }

        ret
    }

    fn peek_other_side(
        node: Option<&Rc<RefCell<TreeNode>>>,
        other_road_height: i32,
        node_height: i32,
        the_other_road: &mut HashMap<i32, i32>,
        left_heights: &HashMap<i32, i32>,
        right_heights: &HashMap<i32, i32>,
    ) {
        if let Some(node) = node {
            let node = node.borrow();
            the_other_road.insert(node.val, other_road_height);

            Self::peek_other_side(
                node.left.as_ref(),
                // crucial part to understand this solution,
                // when we delete a node, we need to stand at the node's parent persipective
                // so if node is at left child, either we pick
                // 1. this parent node's right child
                //        node
                //      left   right...
                // 2. or when deep down, for this case
                //            parent
                //          /       \
                //       left      right...
                //       /   \
                //     target sub right...
                //     we pick parent right most or current height + sub right most
                //     you need to think about this
                other_road_height.max(node_height + right_heights[&node.val]),
                node_height + 1,
                the_other_road,
                left_heights,
                right_heights,
            );

            Self::peek_other_side(
                node.right.as_ref(),
                other_road_height.max(node_height + left_heights[&node.val]),
                node_height + 1,
                the_other_road,
                left_heights,
                right_heights,
            );
        }
    }

    // height means edge
    fn edges(
        node: Option<&Rc<RefCell<TreeNode>>>,
        left_heights: &mut HashMap<i32, i32>,
        right_heights: &mut HashMap<i32, i32>,
    ) -> i32 {
        if let Some(node) = node {
            let n = node.borrow();
            let val = n.val;

            let lh = Self::edges(n.left.as_ref(), left_heights, right_heights);
            let rh = Self::edges(n.right.as_ref(), left_heights, right_heights);

            // cur's left height is
            left_heights.insert(val, lh);
            // cur's right height is
            right_heights.insert(val, rh);

            lh.max(rh) + 1
        } else {
            0
        }
    }
}

fn main() {}
