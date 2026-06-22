struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let sz = nums.len();
        // missing one
        let mut bucket = vec![-1; sz + 1];
        for v in nums {
            bucket[v as usize] = v;
        }
        for (i, &v) in bucket.iter().enumerate() {
            if v == -1 {
                return i as i32;
            }
        }
        unreachable!()
    }
}
fn main() {}
