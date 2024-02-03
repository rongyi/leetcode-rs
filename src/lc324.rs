struct Solution;

impl Solution {
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        let mut sorted = nums.clone();
        sorted.sort();
        let sz = nums.len();
        let mut j = (sz - 1) / 2;
        let mut k = sz - 1;
        for i in 0..sz {
            nums[i] = if i & 1 == 1 {
                let val = sorted[k];
                k -= 1;
                val
            } else {
                let val = sorted[j];
                j -= 1;
                val
            };
        }
    }
}

fn main() {}
