struct Solution;

impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut ret = 0;
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                for k in j + 1..nums.len() {
                    if nums[i] + nums[j] > nums[k] {
                        ret += 1;
                    }
                }
            }
        }
        ret
    }
}

fn main() {}
