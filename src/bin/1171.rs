#![allow(dead_code)]

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

// Add this struct definition
pub struct Solution {}

use std::collections::HashSet;
impl Solution {
    pub fn remove_zero_sum_sublists(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut sum = 0;
        // to pop 0 val node aswell
        let mut sum_save = HashSet::from([0]);
        let mut vals = Vec::new();
        // consume this listnode
        while let Some(node) = head {
            vals.push(node.val);
            sum += node.val;
            // ok, need to pop
            if sum_save.contains(&sum) {
                let val = sum;
                while let Some(x) = vals.pop() {
                    sum_save.remove(&sum);
                    sum -= x;
                    // pop all interval
                    if sum == val {
                        break;
                    }
                }
            }
            sum_save.insert(sum);

            head = node.next;
        }

        vals.into_iter().rev().fold(None, |acc, cur| {
            Some(Box::new(ListNode {
                val: cur,
                next: acc,
            }))
        })
    }
}

fn main() {}
