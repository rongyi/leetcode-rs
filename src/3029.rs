struct Solution;

impl Solution {
    pub fn minimum_time_to_initial_state(word: String, k: i32) -> i32 {
        let n = word.len();
        let k = k as usize;
        let bytes = word.as_bytes();

        // t is the number of seconds (shifts)
        // t * k is the number of characters removed
        let mut t = 1;

        while t * k < n {
            // Check if the remaining suffix matches the prefix of the same length
            let suffix = &bytes[t * k..];
            let prefix = &bytes[0..n - t * k];

            if suffix == prefix {
                return t as i32;
            }

            t += 1;
        }

        // If no overlap is found, it takes enough steps to clear the whole word
        ((n + k - 1) / k) as i32
    }
}

fn main() {}
