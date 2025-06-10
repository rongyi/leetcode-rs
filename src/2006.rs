struct Solution;

impl Solution {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut cnt = 0;
        let sz = nums.len();

        for i in 0..sz {
            for j in i + 1..sz {
                if (nums[i] - nums[j]).abs() == k {
                    cnt += 1;
                }
            }
        }

        cnt
    }
}

fn main() {}
