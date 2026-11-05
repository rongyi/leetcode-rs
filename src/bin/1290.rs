#![allow(dead_code)]

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
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut val = 0;

        let mut p = head;
        while let Some(node) = p {
            val = (val << 1) | node.val;
            p = node.next;
        }

        val
    }
}

fn main() {}
