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

use std::collections::VecDeque;



impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let l1 = Self::reverse_list(l1);
        let l2 = Self::reverse_list(l2);
        let mut carry = 0;
        let mut ret = None;
        let mut stack: VecDeque<i32> = VecDeque::new();

        let mut p1 = &l1;
        let mut p2 = &l2;
        while p1.is_some() || p2.is_some() {
            let val1 = p1.as_ref().map_or(0, |node| node.val);
            let val2 = p2.as_ref().map_or(0, |node| node.val);
            let sum = val1 + val2 + carry;
            carry = sum / 10;
            let d = sum % 10;
            stack.push_back(d);

            if let Some(node) = p1 {
                p1 = &node.next;
            }
            if let Some(node) = p2 {
                p2 = &node.next;
            }
        }
        if carry > 0 {
            stack.push_back(carry);
        }

        while let Some(d) = stack.pop_front() {
            let mut new_node = ListNode::new(d);
            new_node.next = ret;
            ret = Some(Box::new(new_node));
        }

        ret
    }

    fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        while let Some(mut node) = head {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            head = next;
        }
        prev
    }
}

fn main() {}
