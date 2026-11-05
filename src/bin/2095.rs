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
    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let sz = Self::len(head.as_ref());
        let mid = sz / 2;
        if mid == 0 {
            return None;
        }
        let mut new_list = head;
        let mut cur = new_list.as_mut().unwrap();
        let mut i = 0;
        while i < sz {
            if i + 1 == mid {
                let delete_target = cur.next.as_mut().take().unwrap();
                cur.next = delete_target.next.take();
                break;
            }

            cur = cur.next.as_mut().unwrap();
            i += 1;
        }
        new_list
    }

    fn len(head: Option<&Box<ListNode>>) -> usize {
        match head {
            Some(n) => 1 + Self::len(n.next.as_ref()),
            None => 0,
        }
    }
}

fn main() {}
