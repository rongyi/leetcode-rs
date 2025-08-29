struct Solution;

impl Solution {
    pub fn min_max_game(mut nums: Vec<i32>) -> i32 {
        let mut is_min = true;
        while nums.len() > 1 {
            let mut next_round = vec![];
            let mut i = 0;
            while i < nums.len() {
                let v1 = nums[i];
                let v2 = nums[i + 1];
                if is_min {
                    next_round.push(v1.min(v2));
                } else {
                    next_round.push(v1.max(v2));
                }
                is_min = !is_min;

                i += 2;
            }
            nums = next_round;
        }
        nums[0]
    }
}

fn main() {}
