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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(-1)));
        dummy.as_mut().unwrap().next = head;
        let mut front = dummy.clone();
        let mut back = dummy.as_mut();
        for _ in 0..=n {
            front = front.unwrap().next
        }
        while let Some(node) = front {
            front = node.next;
            back = back.unwrap().next.as_mut();
        }

        // back now before the delete target
        let next_node = back.as_mut().unwrap().next.as_mut().unwrap().next.take();
        back.as_mut().unwrap().next = next_node;

        dummy.unwrap().next
    }
}
