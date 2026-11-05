struct Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut ret: Vec<String> = Vec::new();

        if nums.is_empty() {
            return ret;
        }

        let mut cur_start = nums[0];
        let mut cur_acc = 1;

        // not here: enumerate first then skip
        for (i, &c) in nums.iter().enumerate().skip(1) {
            if c == nums[i - 1] + 1 {
                cur_acc += 1;
            } else {
                if cur_acc == 1 {
                    ret.push(cur_start.to_string());
                } else {
                    let s = format!("{}->{}", cur_start, cur_start + cur_acc - 1);
                    ret.push(s);
                }
                cur_start = c;
                cur_acc = 1;
            }
        }

        // last round
        if cur_acc == 1 {
            ret.push(cur_start.to_string());
        } else {
            let s = format!("{}->{}", cur_start, cur_start + cur_acc - 1);
            ret.push(s);
        }

        ret
    }
}

fn main() {
    let input = vec![0, 1, 2, 4, 5, 7];
    Solution::summary_ranges(input);
}
