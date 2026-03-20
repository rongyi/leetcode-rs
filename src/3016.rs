struct Solution;

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        // 1. Count frequency of each character (a-z)
        let mut freq = [0; 26];
        for b in word.bytes() {
            freq[(b - b'a') as usize] += 1;
        }

        // 2. Sort frequencies in descending order
        freq.sort_unstable_by(|a, b| b.cmp(a));

        let mut total_pushes = 0;

        // 3. Iterate through sorted frequencies and calculate cost
        for (i, &f) in freq.iter().enumerate() {
            if f == 0 {
                break;
            } // Optimization: stop if no more letters

            // i / 8 tells us which "layer" the letter is in
            // Layer 0 (indices 0-7) costs 1 push
            // Layer 1 (indices 8-15) costs 2 pushes, etc.
            let pushes_per_char = (i as i32 / 8) + 1;
            total_pushes += f * pushes_per_char;
        }

        total_pushes
    }
}

fn main() {}
