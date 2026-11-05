struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let mut max_val = nums[0];
        for &num in nums.iter() {
            if num > max_val {
                max_val = num;
            }
        }
        let mut left = 0;
        let mut count_max = 0;
        let mut total = 0i64;
        for right in 0..nums.len() {
            if nums[right] == max_val {
                count_max += 1;
            }
            while count_max >= k {
                if nums[left] == max_val {
                    count_max -= 1;
                }
                left += 1;
            }
            total += left as i64;
        }

        total
    }
}

fn main() {}
