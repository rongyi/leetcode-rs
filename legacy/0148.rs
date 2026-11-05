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
        let sz = Self::len(&head);
        Self::sort_helper(head, sz)
    }

    fn sort_helper(mut head: Option<Box<ListNode>>, len: usize) -> Option<Box<ListNode>> {
        if len <= 1 {
            return head;
        }
        let mid = len / 2;

        let mut p = head.as_mut();
        for _ in 0..mid - 1 {
            p = p.unwrap().next.as_mut();
        }

        Self::merge_two_lists(
            Self::sort_helper(p.unwrap().next.take(), len - mid),
            Self::sort_helper(head, mid),
        )
    }

    fn len(head: &Option<Box<ListNode>>) -> usize {
        let mut sz = 0;
        let mut p = head.as_ref();

        while let Some(node) = p {
            p = node.next.as_ref();
            sz += 1;
        }

        sz
    }

    fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (Some(l1), None) => Some(l1),
            (None, Some(l2)) => Some(l2),
            (Some(mut l1), Some(mut l2)) => {
                if l1.val <= l2.val {
                    l1.next = Self::merge_two_lists(l1.next, Some(l2));
                    Some(l1)
                } else {
                    l2.next = Self::merge_two_lists(Some(l1), l2.next);
                    Some(l2)
                }
            }
        }
    }
}

fn main() {}
