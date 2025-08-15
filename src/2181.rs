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
    pub fn merge_nodes(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut cur = dummy.as_mut();

        // we will consume head!
        let mut acc = 0;
        let mut old_cur = head.unwrap().next.take();
        // thanks for the last zero
        while let Some(node) = old_cur.as_mut() {
            if node.val != 0 {
                acc += node.val;
            } else {
                cur.next = Some(Box::new(ListNode::new(acc)));
                cur = cur.next.as_mut().unwrap();
                acc = 0;
            }
            old_cur = node.next.take();
        }

        dummy.next.take()
    }
}

fn main() {}
