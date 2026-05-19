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
        if head.is_none() {
            return None;
        }
        let sz = Self::len(head.as_ref());
        let k = k as usize % sz;
        // nothing change
        if k == 0 {
            return head;
        }
        let mut dummy = Box::new(ListNode::new(-1));
        dummy.next = head;
        let mut tail = &mut dummy;
        for _ in 0..(sz - k) {
            tail = tail.next.as_mut().unwrap();
        }
        let mut second_chain = tail.next.take().unwrap();
        let mut second_tail = &mut second_chain;
        // go to next tail end
        loop {
            if second_tail.next.is_some() {
                second_tail = second_tail.next.as_mut().unwrap();
            } else {
                break;
            }
        }
        second_tail.next = dummy.next.take();

        Some(second_chain)
    }

    fn len(head: Option<&Box<ListNode>>) -> usize {
        match head {
            Some(node) => 1 + Self::len(node.next.as_ref()),
            None => 0,
        }
    }
}

fn main() {}
