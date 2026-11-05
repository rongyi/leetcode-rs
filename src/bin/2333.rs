struct Solution;

impl Solution {
    pub fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
        let sz = nums1.len();
        let mut diff: Vec<i32> = nums1
            .into_iter()
            .zip(nums2.into_iter())
            .map(|p| (p.0 - p.1).abs())
            .collect();
        let max_diff = *diff.iter().max().unwrap() as usize;
        let mut bucket = vec![0; max_diff + 1];
        for &d in diff.iter() {
            bucket[d as usize] += 1;
        }
        let mut k = k1 + k2;
        for i in (0..bucket.len()).rev() {
            if k <= 0 {
                break;
            }
            let cut = bucket[i].min(k);
            k -= cut;
            bucket[i] -= cut;
            if i > 0 {
                bucket[i - 1] += cut;
            }
        }
        let mut ret = 0;

        for i in 0..bucket.len() {
            let i = i as i64;
            ret += i * i * bucket[i as usize] as i64;
        }

        ret
    }
}
fn main() {}
