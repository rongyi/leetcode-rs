struct Solution;

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();

        for i in 0..n {
            while nums[i] > 0 && nums[i] <= n as i32 && nums[nums[i] as usize - 1] != nums[i] {
                let target_idx = nums[i] as usize - 1;
                nums.swap(target_idx, i);
            }
        }

        for i in 0..n {
            if nums[i] != i as i32 + 1 {
                return (i + 1) as i32;
            }
        }

        (n + 1) as i32
    }
}

fn main() {}
