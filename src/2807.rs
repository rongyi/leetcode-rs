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
    pub fn insert_greatest_common_divisors(
        mut head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(-1));
        let mut tail = dummy.as_mut();

        while let Some(mut cur) = head.take() {
            let val = cur.val;
            tail.next = Some(Box::new(ListNode::new(val)));
            tail = tail.next.as_mut().unwrap();
            if let Some(next) = cur.next.as_ref() {
                let next_val = next.val;
                let new_val = Self::gcd(val.max(next_val), val.min(next_val));
                tail.next = Some(Box::new(ListNode::new(new_val)));
                tail = tail.next.as_mut().unwrap();
            }
            head = cur.next.take();
        }

        dummy.next.take()
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }
}

fn main() {}
