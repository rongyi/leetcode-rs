struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap, i64};
impl Solution {
    pub fn minimum_difference(nums: Vec<i32>) -> i64 {
        let nums: Vec<i64> = nums.into_iter().map(|num| num as i64).collect();
        let sz = nums.len();
        let n = sz / 3;
        let mut cache: Vec<i64> = vec![0; sz];
        // left part pop bigger one, make sum as samller as possible
        let mut ql: BinaryHeap<i64> = BinaryHeap::new();

        // right part, pop smaller one, make sum as bigger as possible
        let mut qr: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
        let mut cur_sum = 0;

        for i in (n..sz).rev() {
            qr.push(Reverse(nums[i]));
            cur_sum += nums[i];
            if qr.len() > n {
                cur_sum -= qr.pop().unwrap().0;
            }
            if qr.len() == n {
                cache[i] = cur_sum;
            }
        }
        cur_sum = 0;
        let mut ret = i64::MAX;
        for i in 0..2 * n {
            ql.push(nums[i]);
            cur_sum += nums[i];
            if ql.len() > n {
                cur_sum -= ql.pop().unwrap();
            }
            if ql.len() == n {
                ret = ret.min(cur_sum - cache[i + 1]);
            }
        }

        ret as _
    }
}

fn main() {}
