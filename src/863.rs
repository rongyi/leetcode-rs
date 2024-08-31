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
    pub fn distance_k(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: Option<Rc<RefCell<TreeNode>>>,
        k: i32,
    ) -> Vec<i32> {
        let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();
        Self::in_order(root.as_ref(), None, &mut graph);
        let mut q = VecDeque::new();
        q.push_back(target.as_ref().unwrap().borrow().val);

        let mut dis = 0;

        let mut ret = Vec::new();
        let mut visited: HashSet<i32> = HashSet::new();

        while !q.is_empty() {
            let sz = q.len();
            for _ in 0..sz {
                let cur = q.pop_front().unwrap();
                visited.insert(cur);
                if dis == k {
                    ret.push(cur);
                } else {
                    if let Some(neibs) = graph.get(&cur) {
                        for &neib in neibs.iter() {
                            if !visited.contains(&neib) {
                                q.push_back(neib);
                            }
                        }
                    }
                }
            }
            dis += 1;
        }

        ret
    }

    fn in_order(
        cur: Option<&Rc<RefCell<TreeNode>>>,
        parent: Option<&Rc<RefCell<TreeNode>>>,
        graph: &mut HashMap<i32, HashSet<i32>>,
    ) {
        if let Some(node) = cur {
            let cur = node.borrow();
            if let Some(parent) = parent {
                let p = parent.borrow();
                graph.entry(p.val).or_default().insert(cur.val);
                graph.entry(cur.val).or_default().insert(p.val);
            }
            Self::in_order(cur.left.as_ref(), Some(node), graph);
            Self::in_order(cur.right.as_ref(), Some(node), graph);
        }
    }
}

fn main() {}
