struct Solution;

use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;
impl Solution {
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut cnt: HashMap<i32, i32> = HashMap::new();

        Self::recur(root.as_ref(), &mut cnt);

        let mut group: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
        for (&k, &v) in cnt.iter() {
            group.entry(v).or_insert(Vec::new()).push(k);
        }

        group.into_iter().rev().next().unwrap().1
    }

    fn recur(node: Option<&Rc<RefCell<TreeNode>>>, cnt: &mut HashMap<i32, i32>) {
        if let Some(node) = node {
            let val = node.borrow().val;
            *cnt.entry(val).or_insert(0) += 1;
            Self::recur(node.borrow().left.as_ref(), cnt);
            Self::recur(node.borrow().right.as_ref(), cnt);
        }
    }
}

fn main() {}
