struct Solution;

impl Solution {
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        let sz: i32 = nums.len() as i32;
        let mut i: i32 = 0;
        let mut j: i32 = sz as i32 - 1;

        while i < j {
            while i < sz && nums[i as usize] % 2 == 0 {
                i += 1;
            }
            while j >= 0 && nums[j as usize] % 2 == 1 {
                j -= 1;
            }
            if i < j {
                nums.swap(i as usize, j as usize);
                i += 1;
                j -= 1;
            }
        }

        nums
    }
}

fn main() {}
