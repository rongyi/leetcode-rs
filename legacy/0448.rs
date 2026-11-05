
struct Solution;

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut occ = vec![0; nums.len()];
        let mut ret = Vec::new();
        for num in nums {
            occ[(num - 1) as usize] = 1;
        }
        for i in 0..occ.len() {
            if occ[i] == 0 {
                ret.push((i + 1) as i32);
            }
        }
        ret
    }
}

fn main() {}
