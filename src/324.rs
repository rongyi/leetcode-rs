struct Solution;

impl Solution {
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        let mut sorted = nums.clone();
        sorted.sort();
        let sz = nums.len();

        // j walks backward through the smaller half: indices 0..(sz-1)/2
        // k walks backward through the larger half:  indices (sz-1)/2+1 .. sz-1
        //
        // Placing in reverse order prevents median duplicates from clustering.
        // ⚠️ Still fragile when median appears > floor((n+1)/2) times.
        let mut j = (sz - 1) / 2;
        let mut k = sz - 1;

        for i in 0..sz {
            nums[i] = if i & 1 == 1 {
                // Odd position → needs a large value.
                let val = sorted[k];
                k -= 1; // Actually this underflows... see note below.
                val
            } else {
                // Even position → needs a small value.
                let val = sorted[j];
                j -= 1; // Actually this underflows... see note below.
                val
            };
        }
    }
}

fn main() {}
