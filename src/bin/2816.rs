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
    pub fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // first, we need to get the origin value
        let mut digits: Vec<i32> = Vec::new();

        // digits order like
        // list: 1 -> 2 -> 3
        // digits: [1, 2, 3]
        Self::value(head.as_ref(), &mut digits);
        let mut doubled: Vec<i32> = Vec::new();
        let mut carry = 0;
        while let Some(cur) = digits.pop() {
            let d = cur * 2 + carry;
            doubled.push(d % 10);
            carry = d / 10;
        }
        if carry > 0 {
            doubled.push(carry);
        }
        // digits: [1, 2, 3]
        // doubled: [6, 4, 2], we use pop last to insert to tail

        if doubled.is_empty() {
            return Some(Box::new(ListNode::new(0)));
        }
        let mut dummy = Box::new(ListNode::new(-1));
        let mut tail = dummy.as_mut();
        while let Some(cur) = doubled.pop() {
            tail.next = Some(Box::new(ListNode::new(cur as i32)));
            tail = tail.next.as_mut().unwrap();
        }

        dummy.next.take()
    }

    fn value(head: Option<&Box<ListNode>>, acc: &mut Vec<i32>) {
        if let Some(cur) = head {
            // push first
            acc.push(cur.val);
            Self::value(cur.next.as_ref(), acc)
        }
    }
}

fn main() {}
