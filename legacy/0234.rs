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
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        // Helper function to reverse a linked list
        fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut prev = None;
            while let Some(mut node) = head {
                let next = node.next.take();
                node.next = prev;
                prev = Some(node);
                head = next;
            }
            prev
        }

        // Helper function to find the middle of a linked list
        fn find_middle(head: &Option<Box<ListNode>>) -> Option<&Box<ListNode>> {
            let mut slow = head;
            let mut fast = head;
            while let (Some(node), Some(next_node)) =
                (&slow, &fast.as_ref().and_then(|n| n.next.as_ref()))
            {
                slow = &node.next;
                fast = &next_node.next;
            }
            slow.as_ref()
        }

        // Reverse the second half of the linked list
        let middle = find_middle(&head);
        let reversed_second_half = reverse_list(middle.cloned());

        // Compare the first and reversed second halves
        let mut first_half = &head;
        let mut second_half = &reversed_second_half;
        while let (Some(node1), Some(node2)) = (first_half.as_ref(), second_half.as_ref()) {
            if node1.val != node2.val {
                return false;
            }
            first_half = &node1.next;
            second_half = &node2.next;
        }

        true
    }
}

fn main() {}
