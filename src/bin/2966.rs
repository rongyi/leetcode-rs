struct Solution;

impl Solution {
    pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut res = Vec::new();
        for chunk in nums.chunks(3) {
            if chunk[2] - chunk[0] > k {
                return Vec::new();
            }
            res.push(chunk.to_vec());
        }
        res
    }
}
fn main() {}
