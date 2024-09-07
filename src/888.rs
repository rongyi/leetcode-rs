
struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
        let sum_alice: i32 = alice_sizes.iter().sum();
        let sum_bob: i32 = bob_sizes.iter().sum();
        let diff = (sum_bob - sum_alice) / 2;

        let bob_set: HashSet<i32> = bob_sizes.into_iter().collect();

        for &x in alice_sizes.iter() {
            let y = x + diff;
            if bob_set.contains(&y) {
                return vec![x, y];
            }
        }
        unreachable!()
    }
}

fn main() {}
