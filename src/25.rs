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
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(-1));
        let mut tail = &mut dummy;

        let mut remain = head;

        loop {
            let (cur_chunk, next_chunk, is_full) = Self::split(remain, k);
            if is_full {
                // insert as reverse order
                let mut cur = cur_chunk;
                while cur.is_some() {
                    let mut node = cur.unwrap();
                    // in origin chunk
                    let cache = node.next.take();

                    let next_next = tail.next.take();
                    node.next = next_next;
                    tail.next = Some(node);

                    cur = cache;
                }
            } else {
                // no reverse
                tail.next = cur_chunk;
                break;
            }

            remain = next_chunk;
            // need to drive tail to newlys chain end
            while tail.next.is_some() {
                tail = tail.next.as_mut().unwrap();
            }
        }

        dummy.next
    }

    fn split(
        mut head: Option<Box<ListNode>>,
        k: i32,
    ) -> (Option<Box<ListNode>>, Option<Box<ListNode>>, bool) {
        let mut dummy = Box::new(ListNode::new(-1));
        dummy.next = head;
        let mut i = 0;
        let mut tail = &mut dummy;
        while i < k {
            // short case
            if tail.next.is_none() {
                break;
            }
            tail = tail.next.as_mut().unwrap();

            i += 1;
        }
        // now tail is in current chunk's last position
        let next_chunk = tail.next.take();

        (dummy.next, next_chunk, i == k)
    }
}
fn main() {}
