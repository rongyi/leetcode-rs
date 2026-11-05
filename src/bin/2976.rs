struct Solution;

impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<char>,
        changed: Vec<char>,
        cost: Vec<i32>,
    ) -> i64 {
        let mut dist = vec![vec![i64::MAX; 26]; 26];
        for i in 0..26 {
            dist[i][i] = 0;
        }
        for i in 0..original.len() {
            let u = (original[i] as u8 - b'a') as usize;
            let v = (changed[i] as u8 - b'a') as usize;
            let c = cost[i] as i64;
            if c < dist[u][v] {
                dist[u][v] = c;
            }
        }
        for k in 0..26 {
            for i in 0..26 {
                if dist[i][k] < i64::MAX {
                    for j in 0..26 {
                        if dist[k][j] < i64::MAX {
                            let new_dist = dist[i][k] + dist[k][j];
                            if new_dist < dist[i][j] {
                                dist[i][j] = new_dist;
                            }
                        }
                    }
                }
            }
        }
        let mut total_cost: i64 = 0;
        for (s, t) in source.chars().zip(target.chars()) {
            let u = (s as u8 - b'a') as usize;
            let v = (t as u8 - b'a') as usize;
            if dist[u][v] == i64::MAX {
                return -1;
            }
            total_cost += dist[u][v];
        }
        total_cost
    }
}

fn main() {}
