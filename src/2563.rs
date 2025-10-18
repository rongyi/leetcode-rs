struct Solution;

impl Solution {
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        nums.sort_unstable();
        let count_less = |val: i32| -> i64 {
            let mut acc = 0;
            let mut l = 0;
            let mut r = nums.len() - 1;

            while l < r {
                if nums[l] + nums[r] > val {
                    r -= 1;
                } else {
                    acc += (r - l) as i64;
                    l += 1;
                }
            }

            acc
        };

        count_less(upper) - count_less(lower - 1)
    }
}

fn main() {}
