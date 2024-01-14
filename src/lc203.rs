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
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        dummy.next = head;
        let mut p = &mut dummy;
        while let Some(mut node) = p.next.take() {
            if node.val != val {
                p.next = Some(node);
                p = p.next.as_mut().unwrap();
            } else {
                p.next = node.next.take();
            }
        }
        dummy.next
    }
}

fn main() {}
