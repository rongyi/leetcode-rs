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
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;
impl Solution {
    // uniq values
    pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        Self::dfs(root.as_ref(), -1, &mut graph);
        let mut q: VecDeque<i32> = VecDeque::new();
        q.push_back(start);
        let mut visited: HashSet<i32> = HashSet::new();

        let mut layers = 0;
        while !q.is_empty() {
            let sz = q.len();
            for _ in 0..sz {
                let cur = q.pop_front().unwrap();
                visited.insert(cur);
                if let Some(vs) = graph.get(&cur) {
                    for &v in vs.iter() {
                        if visited.contains(&v) {
                            continue;
                        }
                        q.push_back(v);
                    }
                }
            }
            layers += 1;
        }

        layers - 1
    }

    fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, parent: i32, graph: &mut HashMap<i32, Vec<i32>>) {
        if let Some(node) = root {
            let node = node.borrow();
            let cur_val = node.val;
            if parent != -1 {
                graph.entry(cur_val).or_default().push(parent);
                graph.entry(parent).or_default().push(cur_val);
            }
            Self::dfs(node.left.as_ref(), cur_val, graph);
            Self::dfs(node.right.as_ref(), cur_val, graph);
        }
    }
}

fn main() {}
