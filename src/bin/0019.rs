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
        let mut dummy = Box::new(ListNode::new(-1));
        dummy.next = head;
        let mut dummy2 = dummy.clone();
        let mut fast = dummy.as_ref();
        let mut slow = dummy2.as_mut();
        for _ in 0..n {
            fast = fast.next.as_ref().unwrap();
        }

        while let Some(cur) = fast.next.as_ref() {
            fast = cur;
            slow = slow.next.as_mut().unwrap();
        }
        let tail = slow.next.as_mut().unwrap().next.take();
        slow.next = tail;

        dummy2.next
    }
}

fn main() {}
