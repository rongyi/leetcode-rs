struct Solution;

impl Solution {
    pub fn max_sum_two_no_overlap(nums: Vec<i32>, first_len: i32, second_len: i32) -> i32 {
        fn max_sum(nums: &[i32], l: usize, m: usize) -> i32 {
            let sz = nums.len();
            let mut prefix_sum = vec![0; sz + 1];
            for i in 0..sz {
                prefix_sum[i + 1] = prefix_sum[i] + nums[i];
            }
            let mut max_l = 0;
            let mut ret = 0;
            for i in l..(sz - m + 1) {
                max_l = max_l.max(prefix_sum[i] - prefix_sum[i - l]);

                let sum_m = prefix_sum[i + m] - prefix_sum[i];
                ret = ret.max(max_l + sum_m);
            }

            ret
        }
        let l = first_len as usize;
        let m = second_len as usize;
        max_sum(&nums, l, m).max(max_sum(&nums, m, l))
    }
}

fn main() {}
