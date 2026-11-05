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
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut href = head.as_ref();
        let mut n = 0;

        while let Some(node) = href {
            href = node.next.as_ref();
            n += 1;
        }

        let mut hmut = head.as_mut();
        let mut cnt = 1;

        while cnt < (n + 1) / 2 {
            if let Some(node) = hmut {
                hmut = node.next.as_mut();
                cnt += 1;
            } else {
                break;
            }
        }

        match hmut.take() {
            None => (),
            Some(node) => {
                let tail = Self::reverse(node.next.take());
                if let Some(node) = head {
                    node.next = Self::merge_two_list(tail, node.next.take());
                }
            }
        }
    }

    fn merge_two_list(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1.as_mut(), l2.as_mut()) {
            (None, None) => None,
            (Some(_node1), None) => l1,
            (None, Some(_node2)) => l2,
            (Some(node1), Some(_node2)) => {
                node1.next = Self::merge_two_list(l2, node1.next.take());
                l1
            }
        }
    }

    fn reverse(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut cur = head;
        while let Some(mut cur_inner) = cur.take() {
            cur = cur_inner.next.take();
            cur_inner.next = prev.take();
            prev = Some(cur_inner);
        }

        prev
    }
}

fn main() {
    let optional_value: Option<String> = Some("Hello, World!".to_string());

    // Use `as_ref` to get a reference to the value
    let mut optional_ref: Option<&String> = optional_value.as_ref();
    let c = optional_ref.take();

    println!("Optional value: {:?}", optional_value); // Output: None
    println!("Optional reference: {:?}", optional_ref); // Output: Some("Hello, World!")
    println!("Optional reference: {:?}", c); // Output: Some("Hello, World!")
}
