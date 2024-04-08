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
    pub fn find_duplicate_subtrees(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut cache = HashMap::new();
        Self::tolist(root.as_ref(), &mut cache);

        cache
            .values()
            .into_iter()
            .filter_map(|v| {
                if v.len() > 1 {
                    Some(v[0].clone())
                } else {
                    None
                }
            })
            .collect()
    }

    fn tolist(
        node: Option<&Rc<RefCell<TreeNode>>>,
        cache: &mut HashMap<String, Vec<Option<Rc<RefCell<TreeNode>>>>>,
    ) -> String {
        if let Some(node) = node {
            let mut ret = String::new();
            ret.push('(');
            let l = Self::tolist(node.borrow().left.as_ref(), cache);
            ret.push_str(&l);
            ret.push_str(&node.borrow().val.to_string());
            let r = Self::tolist(node.borrow().right.as_ref(), cache);
            ret.push_str(&r);

            ret.push(')');

            cache
                .entry(ret.clone())
                .or_insert(Vec::new())
                .push(Some(node.clone()));

            ret
        } else {
            return "".to_string();
        }
    }
}

fn main() {}
