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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(10);
        let p1 = &mut dummy;
        let mut p2 = head;

        while let Some(mut node) = p2.take() {
            p2 = node.next.take();
            node.next = p1.next.take();
            p1.next = Some(node);
        }

        dummy.next
    }
}

fn main() {}
