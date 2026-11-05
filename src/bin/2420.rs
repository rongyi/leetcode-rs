struct Solution;

impl Solution {
    pub fn good_indices(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let sz = nums.len();
        let mut dp_prev = vec![1; sz];
        let mut dp_after = vec![1; sz];

        for i in 1..sz {
            if nums[i - 1] >= nums[i] {
                dp_prev[i] = dp_prev[i - 1] + 1;
            }
        }
        for i in (0..sz - 1).rev() {
            if nums[i] <= nums[i + 1] {
                dp_after[i] = dp_after[i + 1] + 1;
            }
        }
        let mut ret = vec![];
        let k = k as usize;
        for i in k..sz - k {
            if dp_prev[i - 1] >= k as i32 && dp_after[i + 1] >= k as i32 {
                ret.push(i as i32);
            }
        }
        ret
    }
}

fn main() {}
