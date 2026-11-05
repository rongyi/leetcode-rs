
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
        let min_diff = critical_index
            .windows(2)
            .map(|w| w[1] - w[0])
            .min()
            .unwrap();

        let max_distance = *critical_index.last().unwrap() - *critical_index.first().unwrap();

        vec![min_diff, max_distance]
    }
}

pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
    if head.is_none() || head.as_ref().unwrap().next.is_none() {
        return vec![-1, -1];
    }

    let mut prev_val = head.as_ref().unwrap().val;
    let mut node = head.as_ref().unwrap().next.as_ref();
    let mut index = 1;
    let mut critical_points = Vec::new();

    while let Some(current) = node {
        if let Some(next) = current.next.as_ref() {
            let current_val = current.val;
            let next_val = next.val;

            if (current_val > prev_val && current_val > next_val)
                || (current_val < prev_val && current_val < next_val)
            {
                critical_points.push(index);
            }

            prev_val = current_val;
            node = current.next.as_ref();
            index += 1;
        } else {
            break;
        }
    }

    if critical_points.len() < 2 {
        return vec![-1, -1];
    }

    let min_distance = critical_points
        .windows(2)
        .map(|w| w[1] - w[0])
        .min()
        .unwrap() as i32;
    let max_distance = critical_points.last().unwrap() - critical_points.first().unwrap();

    vec![min_distance, max_distance as i32]
}

fn main() {}
