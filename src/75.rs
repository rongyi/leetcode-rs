struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut lo = 0;
        let mut hi = nums.len() as isize - 1;
        let mut mid = 0;

        while mid <= hi {
            match nums[mid as usize] {
                0 => {
                    nums.swap(lo, mid as usize);
                    lo += 1;
                    mid += 1;
                }
                1 => {
                    mid += 1;
                }
                2 => {
                    nums.swap(hi as usize, mid as usize);
                    hi -= 1;
                }

                _ => unreachable!(),
            }
        }
    }
}

fn main() {}
