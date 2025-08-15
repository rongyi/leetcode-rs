struct Solution;

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let sz = nums.len();
        let mut cnt = 0;
        for i in 0..sz {
            for j in i + 1..sz {
                if nums[i] == nums[j] && (i * j) % k as usize == 0 {
                    cnt += 1;
                }
            }
        }
        cnt
    }
}

fn main() {}
