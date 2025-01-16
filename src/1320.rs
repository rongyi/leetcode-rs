#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn minimum_distance(word: String) -> i32 {
        let word: Vec<char> = word.chars().collect();
        // why 27? include the start state, i.e. from this state
        let mut dp = vec![vec![i32::MAX; 27]; 27];
        // fresh start for left/right hand
        dp[26][26] = 0;

        fn dist(from: i32, to: i32) -> i32 {
            if from == 26 {
                return 0;
            }
            let fromx = from / 6;
            let fromy = from % 6;
            let tox = to / 6;
            let toy = to % 6;

            (fromx - tox).abs() + (fromy - toy).abs()
        }
        let word: Vec<i32> = word.into_iter().map(|c| c as i32 - 'A' as i32).collect();

        for &w in word.iter() {
            let mut next_dp = vec![vec![i32::MAX; 27]; 27];
            // from this state to current, i can represent the last type in left hand, 26 means a fresh start typing
            //                             j can represent the last type in right hand, 26 means a fresh start typing
            for i in 0..=26 {
                for j in 0..=26 {
                    // prev state is not valid state
                    if dp[i][j] == i32::MAX {
                        continue;
                    }
                    // left hand type this char
                    let next = dp[i][j].checked_add(dist(i as i32, w));
                    if let Some(val) = next {
                        next_dp[w as usize][j] = next_dp[w as usize][j].min(val);
                    }
                    // right hand type this char
                    let next = dp[i][j].checked_add(dist(j as i32, w));
                    if let Some(val) = next {
                        next_dp[i][w as usize] = next_dp[i][w as usize].min(val);
                    }
                }
            }

            dp = next_dp;
        }

        let mut ret = i32::MAX;

        for i in 0..27 {
            for j in 0..27 {
                ret = ret.min(dp[i][j]);
            }
        }

        ret
    }
}

fn main() {}
