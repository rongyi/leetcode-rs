impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut cur = head.as_mut();

        while cur.is_some() && cur.as_ref().unwrap().next.is_some() {
            if cur.as_ref().unwrap().val == cur.as_ref().unwrap().next.as_ref().unwrap().val {
                cur.as_mut().unwrap().next =
                    cur.as_mut().unwrap().next.as_mut().unwrap().next.take();
            } else {
                cur = cur.unwrap().next.as_mut();
            }
        }

        head
    }
}
