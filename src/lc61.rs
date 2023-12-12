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
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        if head.is_none() || k == 0 {
            return head;
        }
        let mut sz = 0;
        let mut cur = head.as_ref();
        while let Some(node) = cur {
            sz += 1;
            cur = node.next.as_ref();
        }
        let cutoff = sz - k % sz;
        if cutoff == sz {
            return head;
        }
        let mut cur = head.as_mut().unwrap();
        for _ in 0..(cutoff - 1) {
            cur = cur.next.as_mut().unwrap();
        }
        let mut new_head = cur.next.take();
        let mut cur = new_head.as_mut();
        while let Some(node) = cur {
            if node.next.is_none() {
                cur = Some(node);
                break;
            }
            cur = node.next.as_mut();
        }
        cur.unwrap().next = head;


        new_head

    }
}
