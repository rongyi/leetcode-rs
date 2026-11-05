struct Solution;

impl Solution {
    pub fn kth_largest_number(mut nums: Vec<String>, k: i32) -> String {
        // string is too long, dont try to convert to integer
        // longer the bigger, same length, just compare as string
        nums.sort_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(&b)));

        nums[nums.len() - k as usize].clone()
    }
}

fn main() {}
