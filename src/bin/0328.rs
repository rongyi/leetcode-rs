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
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut p = head;

        let mut odd_head = Box::new(ListNode::new(-1));
        let mut tail_odd = &mut odd_head;
        let mut even_head = Box::new(ListNode::new(-1));
        let mut tail_even = &mut even_head;
        let mut is_odd = true;
        while let Some(mut cur) = p {
            let next = cur.next.take();

            if is_odd {
                tail_odd.next = Some(cur);
                tail_odd = tail_odd.next.as_mut().unwrap();
            } else {
                tail_even.next = Some(cur);
                tail_even = tail_even.next.as_mut().unwrap();
            }
            is_odd = !is_odd;

            p = next;
        }
        // chain together
        tail_odd.next = even_head.next.take();

        odd_head.next
    }
}

fn main() {}
