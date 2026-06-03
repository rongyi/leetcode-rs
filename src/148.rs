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
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cur = head.as_ref();
        let mut sz = 0;
        while let Some(n) = cur {
            sz += 1;
            cur = n.next.as_ref();
        }

        Self::sort_n(head, sz)
    }

    fn merge(
        mut h1: Option<Box<ListNode>>,
        mut h2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (h1, h2) {
            (Some(mut l1), Some(mut l2)) => {
                if l1.val < l2.val {
                    l1.next = Self::merge(l1.next, Some(l2));
                    Some(l1)
                } else {
                    l2.next = Self::merge(Some(l1), l2.next);
                    Some(l2)
                }
            }
            (Some(l1), None) => Some(l1),
            (None, Some(l2)) => Some(l2),
            (None, None) => None,
        }
    }

    fn sort_n(mut h: Option<Box<ListNode>>, len: usize) -> Option<Box<ListNode>> {
        if len <= 1 {
            return h;
        }
        let mid = len / 2;
        let mut p = h.as_mut();
        for _ in 0..mid - 1 {
            p = p.unwrap().next.as_mut();
        }
        let second_half = p.unwrap().next.take();
        let sorted_h1 = Self::sort_n(h, mid);
        let sorted_h2 = Self::sort_n(second_half, len - mid);

        Self::merge(sorted_h1, sorted_h2)
    }
}

fn main() {}
