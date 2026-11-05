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
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
impl Solution {
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut all_nodes: HashSet<i32> = HashSet::new();
        let mut childs: HashSet<i32> = HashSet::new();
        let mut graph: HashMap<i32, Vec<(i32, bool)>> = HashMap::new();

        for d in descriptions.iter() {
            let (from, to, is_left) = (d[0], d[1], d[2] == 1);
            graph.entry(from).or_default().push((to, is_left));
            all_nodes.insert(from);
            all_nodes.insert(to);
            childs.insert(to);
        }
        let mut root: i32 = 0;
        for node in all_nodes.iter() {
            if childs.contains(node) {
                continue;
            }
            root = *node;

            break;
        }
        let root_node = Rc::new(RefCell::new(TreeNode::new(root)));

        Self::build(root_node.clone(), &graph);

        Some(root_node)
    }

    fn build(root_node: Rc<RefCell<TreeNode>>, graph: &HashMap<i32, Vec<(i32, bool)>>) {
        let mut cur = root_node.borrow_mut();
        if let Some(childs) = graph.get(&cur.val) {
            for c in childs.iter() {
                let node = Rc::new(RefCell::new(TreeNode::new(c.0)));
                if c.1 {
                    cur.left = Some(node.clone());
                    Self::build(node, graph);
                } else {
                    cur.right = Some(node.clone());
                    Self::build(node, graph);
                }
            }
        }
    }
}

fn main() {}
