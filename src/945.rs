struct Solution;

impl Solution {
    pub fn min_increment_for_unique(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut ret = 0;
        let mut prev = nums[0];

        for &num in nums.iter().skip(1) {
            if num <= prev {
                ret += prev + 1 - num;
                prev += 1;
            } else {
                prev = num;
            }
        }

        ret
    }
}

fn main() {}
