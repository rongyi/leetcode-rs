// Definition for singly-linked list.
struct Solution;
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
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut current = head;
        let mut count = 0;

        let mut tmp = &current;
        while let Some(node) = tmp {
            count += 1;
            tmp = &node.next;
        }
        if count < k {
            return current;
        }
        let mut prev = None;
        for _ in 0..k {
            if let Some(mut node) = current {
                current = node.next.take();
                node.next = prev;
                prev = Some(node);
            }
        }
        let mut tmp = &mut prev;

        while let Some(node) = tmp {
            if node.next.is_none() {
                node.next = Solution::reverse_k_group(current, k);
                break;
            }
            tmp = &mut node.next;
        }

        prev
    }
}
