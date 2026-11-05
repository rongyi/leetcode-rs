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
    pub fn swap_nodes(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let n = Self::len(head.as_ref());
        let mut first_val = 0;
        let mut second_val = 0;

        let mut cur = head.as_ref();

        for i in 1..=n {
            if let Some(node) = cur {
                if i == k {
                    first_val = node.val;
                }
                if i == n - k + 1 {
                    second_val = node.val;
                }
                cur = node.next.as_ref();
            }
        }
        let mut cur = head.as_mut();

        for i in 1..=n {
            if let Some(node) = cur {
                if i == k {
                    node.val = second_val;
                }
                if i == n - k + 1 {
                    node.val = first_val;
                }

                cur = node.next.as_mut();
            }
        }

        head
    }
    fn len(head: Option<&Box<ListNode>>) -> i32 {
        if let Some(head) = head {
            1 + Self::len(head.next.as_ref())
        } else {
            0
        }
    }
}

fn main() {}
