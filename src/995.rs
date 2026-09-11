struct Solution;

mod ai {
    struct Solution;

    impl Solution {
        pub fn min_k_bit_flips(mut nums: Vec<i32>, k: i32) -> i32 {
            let n = nums.len();
            let k = k as usize;
            let mut total_flips = 0;
            let mut current_flips = 0;

            for i in 0..n {
                // Remove the flip that started at (i - k) since it no longer covers index i
                if i >= k && nums[i - k] >= 2 {
                    current_flips -= 1;
                }

                // An element's effective value is 0 if its raw value equals (current_flips % 2)
                if nums[i] == (current_flips & 1) {
                    // Not enough elements left to complete a K-bit flip
                    if i + k > n {
                        return -1;
                    }

                    total_flips += 1;
                    current_flips += 1;

                    // Mark in-place that a flip originated at index i
                    nums[i] += 2;
                }
            }

            total_flips
        }
    }
}

impl Solution {
    // Create a new array isFlipped[n].
    // isFlipped[i] = 1 iff we flip K consecutive bits starting at A[i].

    // We maintain a variable flipped and flipped = 1 iff the current bit is flipped.

    // If flipped = 0 and A[i] = 0, we need to flip at A[i].
    // If flipped = 1 and A[i] = 1, we need to flip at A[i].
    // flipped[i - k] tells us if we started a flip K positions ago.
    // If flipped[i - k] is 1, we started a flip there, and now it's ending.
    // If flipped[i - k] is 0, we didn't start a flip there, so nothing changes.
    pub fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
        let sz = nums.len();
        let mut flipped = vec![0; sz];
        let mut cur_flip = 0;
        let mut ret = 0;
        let k = k as usize;
        for i in 0..sz {
            if i >= k {
                cur_flip ^= flipped[i - k];
            }

            if cur_flip == nums[i] {
                if i + k > sz {
                    return -1;
                }
                flipped[i] = 1;
                cur_flip ^= 1;
                ret += 1;
            }
        }

        ret
    }
}

fn main() {}
