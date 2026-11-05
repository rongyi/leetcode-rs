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

// pain in the ass
impl Solution {
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut p = head;

        let mut odd_head = Some(Box::new(ListNode::new(1)));
        let mut op = &mut odd_head;
        let mut even_head = Some(Box::new(ListNode::new(1)));
        let mut ep = &mut even_head;
        let mut is_odd = true;

        while let Some(node) = p.take() {
            if is_odd {
                op.as_mut().unwrap().next = Some(node.clone());
                op = &mut op.as_mut().unwrap().next;
            } else {
                ep.as_mut().unwrap().next = Some(node.clone());
                ep = &mut ep.as_mut().unwrap().next;
            }
            p = node.next.clone();
            is_odd = !is_odd;
        }
        ep.as_mut().unwrap().next = None;
        op.as_mut().unwrap().next = even_head.unwrap().next;

        odd_head.unwrap().next
    }
}

fn main() {}
