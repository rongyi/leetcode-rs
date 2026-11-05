struct Solution;

impl Solution {
    pub fn min_swaps(mut nums: Vec<i32>) -> i32 {
        let ones = nums.iter().filter(|x| **x == 1).count();
        // double it
        nums.extend(nums.clone());
        let mut ones_in_win = 0;
        let mut cur_one_in_win = 0;
        // so how to set window size? just make it to one count
        for (i, &val) in nums.iter().enumerate() {
            if i >= ones && nums[i - ones] == 1 {
                cur_one_in_win -= 1;
            }
            if val == 1 {
                cur_one_in_win += 1;
            }
            ones_in_win = ones_in_win.max(cur_one_in_win);
        }

        (ones - ones_in_win) as _
    }
}

fn main() {}
