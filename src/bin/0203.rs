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
        let mut cur = head;
        let mut dummy = Box::new(ListNode::new(10));
        let mut tail = &mut dummy;
        while let Some(mut n) = cur {
            let next = n.next.take();

            if n.val != val {
                tail.next = Some(n);
                tail = tail.next.as_mut().unwrap();
            }

            cur = next;
        }

        dummy.next
    }
}

fn main() {}
