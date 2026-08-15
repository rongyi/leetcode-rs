struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn num_components(head: Option<Box<ListNode>>, nums: Vec<i32>) -> i32 {
        let nums: HashSet<i32> = nums.into_iter().collect();
        let mut ret = 0;

        let mut p = head.as_ref();
        let mut in_group = false;
        while let Some(node) = p {
            if nums.contains(&node.val) {
                if !in_group {
                    in_group = true;
                }

                if node.next.is_none() && in_group {
                    ret += 1;
                }
            } else {
                if in_group {
                    in_group = false;
                    ret += 1;
                }
            }

            p = node.next.as_ref();
        }

        ret
    }
}

fn main() {}
