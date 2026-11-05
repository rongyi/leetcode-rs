
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
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(-1)));
        dummy.as_mut().unwrap().next = head;
        let mut pre = dummy.as_mut().unwrap();

        for _ in 0..left - 1 {
            pre = pre.next.as_mut().unwrap();
        }
        // the first reversed  node
        let mut cur = pre.next.take();

        for _ in 0..right - left {
            // inserted at head of each next node
            let mut next = cur.as_mut().unwrap().next.take();
            cur.as_mut().unwrap().next = next.as_mut().unwrap().next.take();
            next.as_mut().unwrap().next = pre.next.take();
            pre.next = next;
        }
        // and finally chain the first cur to last
        while pre.next.is_some() {
            pre = pre.next.as_mut().unwrap();
        }
        pre.next = cur;

        dummy.unwrap().next
    }
}

