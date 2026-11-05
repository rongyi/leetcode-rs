#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn maximum_sum(arr: Vec<i32>) -> i32 {
        let sz = arr.len();
        if sz == 1 {
            return arr[0];
        }
        // we drop one element end to i
        let mut with_drop = arr[0].max(arr[1]);
        // either start new sub from arr[1] or just accmulate
        let mut without_drop = arr[1].max(arr[0] + arr[1]);
        let mut ret = with_drop.max(without_drop);

        for i in 2..sz {
            // either merge with current or take prev without_drop and delete current
            // this is the hard part
            with_drop = (arr[i] + with_drop).max(without_drop);
            // either merge with current or just split here, let the arr[i] be the one element subarray
            without_drop = (without_drop + arr[i]).max(arr[i]);
            ret = ret.max(with_drop.max(without_drop));
        }

        ret
    }
}

fn main() {}
