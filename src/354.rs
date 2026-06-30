pub struct Solution;

impl Solution {
    /// LeetCode 354: Russian Doll Envelopes
    ///
    /// Envelope (w,h) fits into (w',h') iff w < w' AND h < h'.
    /// Find the maximum nesting depth.
    ///
    /// # Approach: reduce to LIS
    ///
    /// 1. Sort by width ASC. For equal widths, sort by height DESC.
    ///    → Descending height ensures at most ONE envelope per width
    ///      is picked (since two same-width envelopes can't nest).
    ///
    /// 2. Find LIS on heights (patience sorting / binary search).
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        // Sort: width ↑, then height ↓ (for same width).
        // -e[1] gives descending; safe since heights are 1..=10^5.
        envelopes.sort_unstable_by_key(|e| (e[0], -e[1]));

        let mut lis: Vec<i32> = Vec::new();

        for e in &envelopes {
            let h = e[1];
            let idx = match lis.binary_search(&h) {
                Ok(pos) | Err(pos) => pos, // first position >= h
            };
            if idx == lis.len() {
                lis.push(h);
            } else {
                lis[idx] = h;
            }
        }

        lis.len() as i32
    }
}

fn main() {}
