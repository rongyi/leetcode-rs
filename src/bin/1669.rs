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
    pub fn merge_in_between(
        list1: Option<Box<ListNode>>,
        a: i32,
        b: i32,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode {
            val: 0,
            next: list1,
        }));
        let mut curr = &mut dummy;

        // Move to the node before position 'a'
        for _ in 0..a {
            curr = &mut curr.as_mut().unwrap().next;
        }

        // Save reference to node before 'a'
        let mut prev_a = curr.as_mut().unwrap();

        {
            // Move to the node at position 'b'
            let mut curr = &mut prev_a.next;
            for _ in a..=b {
                curr = &mut curr.as_mut().unwrap().next;
            }

            // Save the node after 'b'
            let after_b = curr.take();

            // Find the end of list2
            let mut list2_tail = &mut list2;
            while list2_tail.as_ref().unwrap().next.is_some() {
                list2_tail = &mut list2_tail.as_mut().unwrap().next;
            }

            // Connect list2's tail to after_b
            list2_tail.as_mut().unwrap().next = after_b;
        }

        // Connect node before 'a' to list2
        prev_a.next = list2;

        dummy.unwrap().next
    }
}

fn main() {}
