struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut sorted = vec![];

        for &num in nums.iter() {
            let pos = sorted.binary_search(&num).unwrap_or_else(|e| e);
            if pos == sorted.len() {
                sorted.push(num);
            } else {
                sorted[pos] = num;
            }
        }

        sorted.len() as _
    }
}

fn main() {}
