struct Solution;

impl Solution {
    pub fn count_pairs(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let mut ret = 0;
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] + nums[j] < target {
                    ret += 1;
                } else {
                    break;
                }
            }
        }

        ret
    }
}

fn main() {}
