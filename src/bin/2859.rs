struct Solution;

impl Solution {
    pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
        let mut ret = 0;
        let k = k as u32;
        // fn bitcount(mut v: usize) -> i32 {
        //     let mut acc = 0;
        //     while v > 0 {
        //         acc += 1;
        //         v &= v - 1;
        //     }
        //     acc
        // }
        for (i, &v) in nums.iter().enumerate() {
            if i.count_ones() == k {
                ret += v;
            }
        }

        ret
    }
}

fn main() {}
