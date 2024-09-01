struct Solution;
use std::collections::BinaryHeap;


impl Solution {
    pub fn advantage_count(mut nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let sz = nums1.len();
        nums1.sort_unstable();
        let mut heap: BinaryHeap<(i32, usize)> = nums2
            .into_iter()
            .enumerate()
            .map(|(i, num)| (num, i))
            .collect();

        let mut ret = vec![0; sz];
        let mut low = 0;
        let mut high = sz - 1;

        while let Some((num2, idx)) = heap.pop() {
            if nums1[high] > num2 {
                ret[idx] = nums1[high];
                high -= 1;
            } else {
                ret[idx] = nums1[low];
                low += 1;
            }
        }

        ret
    }
}

fn main() {}
