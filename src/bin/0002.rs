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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut carry = 0;

        let mut dummy = ListNode::new(0);
        let mut tail = &mut dummy;

        let mut cur1 = l1.as_ref();
        let mut cur2 = l2.as_ref();

        while cur1.is_some() || cur2.is_some() || carry != 0 {
            let sum = carry + cur1.map_or(0, |n| n.val) + cur2.map_or(0, |n| n.val);
            carry = sum / 10;
            tail.next = Some(Box::new(ListNode {
                val: sum % 10,
                next: None,
            }));
            tail = tail.next.as_mut().unwrap();

            cur1 = cur1.and_then(|n| n.next.as_ref());
            cur2 = cur2.and_then(|n| n.next.as_ref());
        }

        dummy.next.take()
    }
}

fn main() {}
