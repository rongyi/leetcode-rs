struct Solution;

impl Solution {
    pub fn alternating_subarray(nums: Vec<i32>) -> i32 {
        let mut ret = -1;
        let mut cur_cnt = 1;
        let mut expected_diff = 1;

        for w in nums.windows(2) {
            let diff = w[1] - w[0];
            if diff != expected_diff {
                cur_cnt = 1;
                expected_diff = 1;
            }

            if diff == expected_diff {
                cur_cnt += 1;
                expected_diff = -expected_diff;
                ret = ret.max(cur_cnt);
            }
        }

        ret
    }
}

fn main() {}
