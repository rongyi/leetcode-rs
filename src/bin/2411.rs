struct Solution;

impl Solution {
    pub fn smallest_subarrays(nums: Vec<i32>) -> Vec<i32> {
        let sz = nums.len();
        let mut min_idx: Vec<usize> = vec![0; 32];
        // reverse order
        let mut ret = vec![0; sz];
        for i in (0..sz).rev() {
            let cur = nums[i];
            for j in 0..32 {
                if (cur & (1 << j)) != 0 {
                    // mark minimum index
                    min_idx[j] = i;
                }
            }
            let upper = *min_idx.iter().max().unwrap();
            ret[i] = (upper.max(i) - i + 1) as i32;
        }

        ret
    }
}

fn main() {}
