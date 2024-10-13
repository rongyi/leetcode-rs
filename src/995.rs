struct Solution;

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
