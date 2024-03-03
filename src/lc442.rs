struct Solution;

impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        let mut ret = Vec::new();

        for i in 0..nums.len() {
            let idx = nums[i].abs() as usize - 1;
            if nums[idx] > 0 {
                nums[idx] *= -1;
            } else {
                ret.push(idx as i32 + 1);
            }
        }

        ret
    }
}

fn main() {}
