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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(-300));
        let mut tail = &mut dummy;

        let mut p1 = head.as_ref();
        loop {
            match p1 {
                None => {
                    break;
                }
                Some(cur) => {
                    if cur.val != tail.val {
                        tail.next = Some(Box::new(ListNode::new(cur.val)));
                        tail = tail.next.as_mut().unwrap();
                    }
                    p1 = cur.next.as_ref();
                }
            }
        }

        dummy.next.take()
    }
}

fn main() {}
