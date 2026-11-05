#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn rank_teams(votes: Vec<String>) -> String {
        let votes: Vec<Vec<char>> = votes.into_iter().map(|s| s.chars().collect()).collect();
        let mut ret: Vec<char> = votes[0].clone();
        let sz = votes[0].len();
        let mut rank = vec![vec![0; sz]; 26];

        for v in votes.iter() {
            for j in 0..sz {
                rank[v[j] as usize - 'A' as usize][j] += 1;
            }
        }

        ret.sort_by(|&l, &r| {
            rank[r as usize - 'A' as usize]
                .cmp(&rank[l as usize - 'A' as usize])
                .then(l.cmp(&r))
        });

        ret.into_iter().collect()
    }
}

fn main() {}
