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
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(-1));
        let mut cur = head;
        while cur.is_some() {
            let next = cur.as_mut().unwrap().next.take();
            let cur_val = cur.as_ref().unwrap().val;
            let mut p = &mut dummy;
            while p.next.is_some() {
                let next_val = p.next.as_mut().unwrap().val;
                if cur_val > next_val {
                    p = p.next.as_mut().unwrap();
                } else {
                    break;
                }
            }
            let sorted_next = p.next.take();
            cur.as_mut().unwrap().next = sorted_next;
            p.next = cur;

            cur = next;
        }
        dummy.next
    }
}

fn main() {}
