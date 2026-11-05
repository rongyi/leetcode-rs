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
    pub fn spiral_matrix(m: i32, n: i32, head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        let m = m as usize;
        let n = n as usize;
        let mut ret: Vec<Vec<i32>> = vec![vec![-1; n]; m];
        let mut cur = head.as_ref();

        let mut col_end = n as i32;
        let mut row_end = m as i32;
        let mut row_start = 0;
        let mut col_start = -1;
        // 0 -> right
        // 1 -> down
        // 2 -> left
        // 3 -> up
        let mut dirs = 0;

        let mut i = 0i32;
        let mut j = 0i32;
        // number of nodes <= m * n, so no need to add thresh for total
        while let Some(node) = cur {
            let val = node.val;
            ret[i as usize][j as usize] = val;

            let cur_direction = dirs % 4;
            match cur_direction {
                0 => {
                    j += 1;
                    if j >= col_end {
                        i += 1;
                        j = col_end - 1;
                        col_end -= 1;
                        // change direction to down
                        dirs = 1;
                    }
                }
                1 => {
                    i += 1;
                    if i >= row_end {
                        i = row_end - 1;
                        j -= 1;
                        row_end -= 1;
                        dirs = 2;
                    }
                }
                2 => {
                    j -= 1;
                    if j <= col_start {
                        j = col_start + 1;
                        i -= 1;

                        col_start += 1;
                        dirs = 3;
                    }
                }
                3 => {
                    i -= 1;
                    if i <= row_start {
                        j += 1;
                        i = row_start + 1;

                        row_start += 1;
                        dirs = 0;
                    }
                }
                _ => unreachable!(),
            }

            cur = node.next.as_ref();
        }

        ret
    }
}

fn main() {}
