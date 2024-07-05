#![allow(dead_code)]

use std::collections::HashSet;

struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

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
