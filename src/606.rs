struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if let Some(node) = root {
            let node = node.borrow();
            let mut ret = node.val.to_string();
            if node.left.is_some() || node.right.is_some() {
                ret.push_str(&format!("({})", Self::tree2str(node.left.clone())));
                if node.right.is_some() {
                    ret.push_str(&format!("({})", Self::tree2str(node.right.clone())));
                }
            }

            ret
        } else {
            String::new()
        }
    }
}

fn main() {}
