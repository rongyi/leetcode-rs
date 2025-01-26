#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        const INF: i32 = 1e6 as i32;
        let n = n as usize;
        let mut dist = vec![vec![INF; n]; n];
        for i in 0..n {
            dist[i][i] = 0;
        }
        for e in edges.iter() {
            let (v0, v1, w) = (e[0], e[1], e[2]);
            dist[v0 as usize][v1 as usize] = w;
            dist[v1 as usize][v0 as usize] = w;
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                }
            }
        }
        let mut min_reach = n;
        let mut ret = 0;
        for i in 0..n {
            let mut cur_reach = 0;
            for j in 0..n {
                if dist[i][j] <= distance_threshold {
                    cur_reach += 1;
                }
            }
            if cur_reach <= min_reach {
                min_reach = cur_reach;
                ret = i;
            }
        }

        ret as i32
    }
}

fn main() {}
