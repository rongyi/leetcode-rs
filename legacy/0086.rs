
impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut dummy1 = ListNode::new(0);
        let mut dummy2 = ListNode::new(0);
        let mut p1 = &mut dummy1;
        let mut p2 = &mut dummy2;

        let mut cur = head;

        while let Some(mut node) = cur {
            cur = node.next.take();
            if node.val < x {
                p1.next = Some(node);
                p1 = p1.next.as_mut().unwrap();
            } else {
                p2.next = Some(node);
                p2 = p2.next.as_mut().unwrap();
            }
        }

        p2.next = None;
        p1.next = dummy2.next;

        dummy1.next
    }
}
