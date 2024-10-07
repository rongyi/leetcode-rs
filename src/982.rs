struct Solution;

impl Solution {
    pub fn count_triplets(nums: Vec<i32>) -> i32 {
        let mut tuples = vec![0; 1 << 16];
        for &num1 in nums.iter() {
            for &num2 in nums.iter() {
                tuples[(num1 & num2) as usize] += 1;
            }
        }

        let mut ret = 0;
        for &a in nums.iter() {
            for i in 0..(1 << 16) {
                if i & a == 0 {
                    ret += tuples[i as usize];
                }
            }
        }

        ret
    }
}

fn main() {}
