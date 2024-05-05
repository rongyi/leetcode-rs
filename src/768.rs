#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let sz = arr.len();
        let mut ret = 1;
        let mut max_till_cur = arr.clone();
        let mut min_after_cur = arr.clone();

        for i in 1..sz {
            max_till_cur[i] = max_till_cur[i - 1].max(arr[i]);
        }
        for i in (0..sz - 1).rev() {
            min_after_cur[i] = min_after_cur[i + 1].min(arr[i]);
        }

        for i in 0..sz - 1 {
            // 能在某个位置劈开的条件是，前面一坨的最大值都不能比后面最小的还要小，否则就拼不起来了
            // Algorithm: Iterate through the array, each time all elements to the left are smaller (or equal) to all elements to the right, there is a new chunck.
            if max_till_cur[i] <= min_after_cur[i + 1] {
                ret += 1;
            }
        }

        ret
    }
}

fn main() {}
