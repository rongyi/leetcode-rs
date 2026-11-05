struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug, PartialOrd, Ord)]
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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut lists = lists;
        let len = lists.len();
        if len == 0 {
            return None;
        } else if len == 1 {
            return lists[0].take();
        }

        let mut interval = 1;
        while interval < len {
            // to make sure the odd number of last is unmerged untill last round
            for i in (0..len - interval).step_by(interval * 2) {
                lists[i] = Self::merge_two_lists(lists[i].take(), lists[i + interval].take());
            }
            interval *= 2;
        }
        //or the dummy way
        //for i in 1..len {
        //lists[0] = Self::merge_two_lists(lists[0].take(), lists[i].take());
        //}

        lists[0].take()
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
