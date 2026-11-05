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
struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut ret = String::new();

        fn recur(node: &Option<Rc<RefCell<TreeNode>>>, s: &mut String) {
            match node {
                Some(node) => {
                    s.push_str(&node.borrow().val.to_string());
                    s.push(',');
                    recur(&node.borrow().left, s);
                    recur(&node.borrow().right, s);
                }
                None => {
                    s.push_str("null,");
                }
            }
        }
        recur(&root, &mut ret);
        ret.pop();
        ret
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        fn recur(data: &mut Vec<&str>) -> Option<Rc<RefCell<TreeNode>>> {
            if data.is_empty() {
                return None;
            }
            let val = data.remove(0);
            if val == "null" {
                return None;
            }
            let node = Rc::new(RefCell::new(TreeNode::new(val.parse().unwrap())));
            node.borrow_mut().left = recur(data);
            node.borrow_mut().right = recur(data);

            Some(node)
        }
        let mut nodes: Vec<&str> = data.split(',').collect();
        recur(&mut nodes)
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */

fn main() {}
