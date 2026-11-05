struct Solution;

impl Solution {
    pub fn moves_to_make_zigzag(nums: Vec<i32>) -> i32 {
        let sz = nums.len();
        if sz == 1 {
            return 0;
        }
        let mut even_smaller = 0;
        let mut odd_smaller = 0;
        for i in 0..sz {
            if i % 2 == 0 {
                let left = if i > 0 { nums[i - 1] } else { i32::MAX };
                let right = if i < sz - 1 { nums[i + 1] } else { i32::MAX };
                let target = left.min(right) - 1;
                if nums[i] >= target {
                    even_smaller += nums[i] - target;
                }
            }
        }

        // Check odd indices smaller
        for i in 0..sz {
            if i % 2 == 1 {
                let left = nums[i - 1];
                let right = if i < sz - 1 { nums[i + 1] } else { i32::MAX };
                let target = left.min(right) - 1;
                if nums[i] >= target {
                    odd_smaller += nums[i] - target;
                }
            }
        }
        even_smaller.min(odd_smaller)
    }
}

fn main() {}
