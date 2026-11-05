struct Solution;

impl Solution {
    pub fn find_middle_index(nums: Vec<i32>) -> i32 {
        let sz = nums.len();
        let mut prefix_sum = vec![0; sz + 1];
        for i in 0..sz {
            prefix_sum[i + 1] = prefix_sum[i] + nums[i];
        }
        for i in 0..sz {
            let left = prefix_sum[i];
            let right = prefix_sum[sz] - prefix_sum[i + 1];
            if left == right {
                return i as i32;
            }
        }

        -1
    }
}

fn main() {}
