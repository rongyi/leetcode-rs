struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn maximize_square_area(
        m: i32,
        n: i32,
        mut h_fences: Vec<i32>,
        mut v_fences: Vec<i32>,
    ) -> i32 {
        // Add the field boundaries to the fence lists
        h_fences.push(1);
        h_fences.push(m);
        v_fences.push(1);
        v_fences.push(n);

        // Helper to get all possible differences (gaps) between any two fences
        fn get_gaps(mut fences: Vec<i32>) -> HashSet<i32> {
            fences.sort_unstable();
            let mut gaps = HashSet::new();
            for i in 0..fences.len() {
                for j in i + 1..fences.len() {
                    gaps.insert(fences[j] - fences[i]);
                }
            }
            gaps
        }

        let h_gaps = get_gaps(h_fences);
        let v_gaps = get_gaps(v_fences);

        let mut max_side = -1i64;

        // Find the largest gap present in both horizontal and vertical sets
        for &gap in &h_gaps {
            if v_gaps.contains(&gap) {
                max_side = max_side.max(gap as i64);
            }
        }

        if max_side == -1 {
            -1
        } else {
            ((max_side * max_side) % 1_000_000_007) as i32
        }
    }
}
fn main() {}
