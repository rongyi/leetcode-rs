use std::usize;

pub struct Solution;
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
    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        // The number of nodes in the list is in the range [2, 105].
        let mut critical_index = vec![];

        let mut prev = head.as_ref().unwrap().val;
        let mut curo = head.unwrap().next.take();
        let mut cur_index = 1;
        while let Some(mut cur) = curo {
            if cur.next.is_some() {
                let mut is_maxima = false;
                let mut is_minima = false;
                if cur.val > prev && cur.val > cur.next.as_ref().unwrap().val {
                    is_maxima = true;
                }
                if cur.val < prev && cur.val < cur.next.as_ref().unwrap().val {
                    is_minima = true;
                }
                if is_maxima || is_minima {
                    critical_index.push(cur_index);
                }

                cur_index += 1;
                prev = cur.val;
                curo = cur.next.take();
            } else {
                break;
            }
        }

        if critical_index.len() < 2 {
            return vec![-1, -1];
        }
        let mut min_diff = usize::MAX;
        for i in 0..critical_index.len() - 1 {
            min_diff = min_diff.min(critical_index[i + 1] - critical_index[i]);
        }

        let max_distance = *critical_index.last().unwrap() - *critical_index.first().unwrap();

        vec![min_diff as i32, max_distance as i32]
    }
}

fn main() {}
