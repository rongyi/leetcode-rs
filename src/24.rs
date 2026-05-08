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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // pain in the ass using rust
        match head {
            None => None, // empty node
            Some(mut cur) => match cur.next {
                Some(mut next_next) => {
                    cur.next = Self::swap_pairs(next_next.next.take());
                    next_next.next = Some(cur);

                    Some(next_next)
                }
                None => Some(cur), // only one node
            },
        }
    }
}

fn main() {}
