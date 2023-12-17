
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(-1)));
        dummy.as_mut().unwrap().next = head;

        let mut cur = dummy.as_mut().unwrap();

        while cur.next.is_some() && cur.next.as_ref().unwrap().next.is_some() {
            if cur.next.as_ref().unwrap().val
                == cur.next.as_ref().unwrap().next.as_ref().unwrap().val
            {
                let val = cur.next.as_ref().unwrap().val;
                while cur.next.is_some() && cur.next.as_ref().unwrap().val == val {
                    cur.next = cur.next.as_mut().unwrap().next.take();
                }
            } else {
                cur = cur.next.as_mut().unwrap();
            }
        }

        dummy.unwrap().next
    }
}

