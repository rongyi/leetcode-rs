// Definition for singly-linked list.

struct Solution;
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
        let mut dummy = ListNode::new(-1);
        dummy.next = head;
        let mut current = &mut dummy;

        while let Some(mut node1) = current.next.take() {
            if let Some(mut node2) = node1.next.take() {
                // cache next round start
                let next = node2.next.take();
                // swap action
                // prev -> next = node2
                // node2 -> next = node1
                node2.next = Some(node1);
                current.next = Some(node2);
                current = current.next.as_mut().unwrap().next.as_mut().unwrap();
                current.next = next;
            } else {
                current.next = Some(node1);
                break;
            }
        }

        dummy.next
    }
}
