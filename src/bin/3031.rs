struct Solution;

impl Solution {
    pub fn minimum_time_to_initial_state(word: String, k: i32) -> i32 {
        let s = word.as_bytes();
        let n = s.len();
        let k = k as usize;

        // 1. Compute the Z-array
        let mut z = vec![0; n];
        let (mut l, mut r) = (0, 0);

        for i in 1..n {
            if i <= r {
                z[i] = std::cmp::min(r - i + 1, z[i - l]);
            }
            while i + z[i] < n && s[z[i]] == s[i + z[i]] {
                z[i] += 1;
            }
            if i + z[i] - 1 > r {
                l = i;
                r = i + z[i] - 1;
            }
        }

        // 2. Check each possible shift point
        let mut t = 1;
        while t * k < n {
            let suffix_start = t * k;
            let suffix_len = n - suffix_start;

            // If the LCP at this suffix matches the suffix length, it's a prefix!
            if z[suffix_start] >= suffix_len {
                return t as i32;
            }
            t += 1;
        }

        // 3. If no match, we clear the whole word
        ((n + k - 1) / k) as i32
    }
}
fn main() {}
