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
        let mut slow = head.as_ref();
        let mut fast = head.as_ref();

        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = slow.as_ref().unwrap().next.as_ref();
            fast = fast.as_ref().unwrap().next.as_ref().unwrap().next.as_ref();
        }

        slow.map(|v| v.clone())
    }
}
fn main() {}
