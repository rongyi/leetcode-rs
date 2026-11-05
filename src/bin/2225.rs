struct Solution;

use std::collections::{BTreeSet, HashMap};
impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut indegree: HashMap<i32, i32> = HashMap::new();
        let mut all_players: BTreeSet<i32> = BTreeSet::new();
        for m in matches.iter() {
            let (from, to) = (m[0], m[1]);
            *indegree.entry(to).or_default() += 1;
            all_players.insert(from);
            all_players.insert(to);
        }
        let mut no_lose = vec![];
        let mut lose_onlyl_one = vec![];

        for p in all_players.iter() {
            if let Some(v) = indegree.get(p) {
                if *v == 1 {
                    lose_onlyl_one.push(*p);
                }
            } else {
                no_lose.push(*p);
            }
        }
        // no_lose.sort_unstable();
        // lose_onlyl_one.sort_unstable();
        vec![no_lose, lose_onlyl_one]
    }
}

fn main() {}
