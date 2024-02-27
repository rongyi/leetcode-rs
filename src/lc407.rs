struct Solution;
use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let m = height_map.len();
        let n = height_map[0].len();
        let mut pq: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();
        let mut visited = vec![vec![false; n]; m];
        // insert the boundary
        for i in 0..m {
            for j in 0..n {
                if i == 0 || i == m - 1 || j == 0 || j == n - 1 {
                    pq.push(Reverse((height_map[i][j], (i * n + j) as i32)));
                    visited[i][j] = true;
                }
            }
        }
        let mut ret = 0;
        let mut max_height = i32::MIN;
        let dirs = [[0, 1], [1, 0], [0, -1], [-1, 0]];
        while !pq.is_empty() {
            let Reverse((height, pos)) = pq.pop().unwrap();
            let i = pos / n as i32;
            let j = pos % n as i32;
            max_height = max_height.max(height);

            for d in &dirs {
                let ni = i + d[0];
                let nj = j + d[1];
                if ni >= m as i32
                    || ni < 0
                    || nj >= n as i32
                    || nj < 0
                    || visited[ni as usize][nj as usize]
                {
                    continue;
                }
                let ni = ni as usize;
                let nj = nj as usize;
                visited[ni][nj] = true;

                if height_map[ni][nj] < max_height {
                    ret += max_height - height_map[ni][nj];
                }
                pq.push(Reverse((height_map[ni][nj], (ni * n + nj) as i32)));
            }
        }

        ret
    }
}

fn main() {
    println!("{}", 10i32.pow(2));
}
