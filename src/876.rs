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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let sz = Self::len(head.as_ref());
        let mut cur = head.as_ref();
        for _ in 0..sz / 2 {
            let next = cur.unwrap().next.as_ref();
            cur = next;
        }
        cur.map(|c| c.clone())
    }
    fn len(node: Option<&Box<ListNode>>) -> usize {
        if let Some(node) = node {
            1 + Self::len(node.next.as_ref())
        } else {
            0
        }
    }
}

fn main() {}
